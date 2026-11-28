use evo75_keylab::evo75_connection;
use hidapi::{HidApi, HidDevice};
use hidparser::{Report, parse_report_descriptor};
use std::sync::Mutex;

#[derive(Clone, Copy)]
struct ReportIds {
    input: u8,
    output: u8,
}

fn describe(reports: &[Report]) -> String {
    if reports.is_empty() {
        return "aucun".into();
    }
    reports
        .iter()
        .map(|r| {
            let id = r.report_id.map(u32::from).unwrap_or(0);
            format!("ID {id} / {} bits", r.size_in_bits)
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn via_id(reports: &[Report]) -> Option<u8> {
    let [report] = reports else { return None };
    if report.size_in_bits != 256 {
        return None;
    }
    u8::try_from(report.report_id.map(u32::from).unwrap_or(0)).ok()
}

fn inspect(device: &HidDevice) -> Result<(ReportIds, String), String> {
    let mut raw = vec![0; hidapi::MAX_REPORT_DESCRIPTOR_SIZE];
    let len = device
        .get_report_descriptor(&mut raw)
        .map_err(|e| format!("Lecture du descripteur HID : {e}"))?;
    let d = parse_report_descriptor(&raw[..len])
        .map_err(|e| format!("Analyse du descripteur HID : {e:?}"))?;
    let summary = format!(
        "Entrée : {} · Sortie : {} · Feature : {}",
        describe(&d.input_reports),
        describe(&d.output_reports),
        describe(&d.features)
    );
    if d.bad_input_reports.is_empty() && d.bad_output_reports.is_empty() {
        if let (Some(input), Some(output)) = (via_id(&d.input_reports), via_id(&d.output_reports)) {
            return Ok((ReportIds { input, output }, summary));
        }
    }
    Err(format!("Format VIA 32 octets non confirmé. {summary}"))
}

static TRANSACTION: Mutex<()> = Mutex::new(());
const GET_VERSION: u8 = 1;
const GET: u8 = 8;
const SET: u8 = 7;
const MATRIX: u8 = 3;
const COLOR: u8 = 4;

#[derive(Clone, Copy)]
enum Output {
    Interrupt,
    Control,
}

fn exchange(
    device: &HidDevice,
    output: Output,
    ids: ReportIds,
    command: &[u8],
) -> Result<[u8; 32], String> {
    let mut packet = [0; 33];
    packet[0] = ids.output;
    packet[1..1 + command.len()].copy_from_slice(command);
    let id = command[0];
    match output {
        Output::Interrupt => {
            let written = device
                .write(&packet)
                .map_err(|e| format!("Envoi interrupt 0x{id:02X} : {e}"))?;
            if written != packet.len() {
                return Err(format!("Envoi interrupt 0x{id:02X} incomplet."));
            }
        }
        Output::Control => device
            .send_output_report(&packet)
            .map_err(|e| format!("Envoi contrôle 0x{id:02X} : {e}"))?,
    }
    let mut raw = [0; 33];
    let count = device
        .read_timeout(&mut raw, 1500)
        .map_err(|e| format!("Lecture après 0x{id:02X} : {e}"))?;
    if count == 0 {
        return Err(format!("Aucune réponse à 0x{id:02X} sous 1,5 s."));
    }
    let offset = usize::from(ids.input != 0 || (count == 33 && raw[0] == 0));
    if count != 32 + offset || (offset == 1 && raw[0] != ids.input) {
        return Err(format!("Rapport entrant inattendu ({count} octets)."));
    }
    let mut reply = [0; 32];
    reply.copy_from_slice(&raw[offset..count]);
    let header_len = if command[0] == GET_VERSION { 1 } else { 3 };
    if reply[..header_len] != command[..header_len] {
        return Err(format!(
            "Réponse inattendue ({} octets, commande 0x{:02X}).",
            count, reply[0]
        ));
    }
    Ok(reply)
}

fn read_color(device: &HidDevice, output: Output, ids: ReportIds) -> Result<[u8; 2], String> {
    // Two different padding values distinguish returned data from a request echo.
    let first = exchange(device, output, ids, &[GET, MATRIX, COLOR, 0, 0])?;
    let second = exchange(device, output, ids, &[GET, MATRIX, COLOR, 255, 255])?;
    if first[3..5] != second[3..5] {
        return Err("Lecture de teinte non confirmée par le récepteur.".into());
    }
    Ok([first[3], first[4]])
}

pub fn communicate(color: Option<[u8; 2]>) -> Result<String, String> {
    let _guard = TRANSACTION.lock().map_err(|e| e.to_string())?;
    let api = HidApi::new().map_err(|e| e.to_string())?;
    let mut usb = Vec::new();
    let mut receivers = Vec::new();
    for device in api.device_list() {
        if (device.usage_page(), device.usage()) != (0xff60, 0x61) {
            continue;
        }
        match evo75_connection(device) {
            Some("USB") => usb.push(device),
            Some("2,4 GHz") if device.interface_number() == 1 => receivers.push(device),
            _ => {}
        }
    }
    let (devices, connection) = if usb.is_empty() {
        (receivers, "2,4 GHz")
    } else {
        (usb, "USB")
    };
    let info = match devices.as_slice() {
        [info] => *info,
        [] => return Err("Interface VIA Evo75 introuvable. Branche le clavier en mode USB.".into()),
        _ => return Err(format!("Plusieurs interfaces VIA Evo75 en {connection}.")),
    };
    let open = || {
        info.open_device(&api)
            .map_err(|e| format!("Ouverture HID : {e}"))
    };
    let device = open()?;
    let (ids, summary) = inspect(&device)?;
    let (device, output, answer) = match exchange(&device, Output::Interrupt, ids, &[GET_VERSION]) {
        Ok(answer) => (device, Output::Interrupt, answer),
        Err(first) => {
            drop(device);
            let device = open()?;
            let answer =
                exchange(&device, Output::Control, ids, &[GET_VERSION]).map_err(|second| {
                    format!("{summary} · Essai interrupt : {first} · Essai contrôle : {second}")
                })?;
            (device, Output::Control, answer)
        }
    };
    let version = u16::from_be_bytes([answer[1], answer[2]]);
    if !(11..=13).contains(&version) {
        return Err(format!(
            "Version reçue : 0x{version:04X}. Commandes couleur non activées."
        ));
    }
    let current =
        read_color(&device, output, ids).map_err(|e| format!("VIA {version} répond. {e}"))?;
    if let Some([hue, sat]) = color {
        exchange(&device, output, ids, &[SET, MATRIX, COLOR, hue, sat])?;
        if read_color(&device, output, ids)? != [hue, sat] {
            return Err("Commande envoyée, mais la relecture de la teinte diffère.".into());
        }
        Ok(format!(
            "{connection} · VIA {version} · Couleur confirmée par relecture."
        ))
    } else {
        Ok(format!(
            "{connection} · VIA {version} · Teinte {} et saturation {} lues sur le clavier.",
            current[0], current[1]
        ))
    }
}
