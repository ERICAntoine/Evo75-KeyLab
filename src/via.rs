use hidapi::{HidApi, HidDevice};
use std::sync::Mutex;

static TRANSACTION: Mutex<()> = Mutex::new(());
const GET_VERSION: u8 = 1;
const GET: u8 = 8;
const SET: u8 = 7;
const MATRIX: u8 = 3;
const COLOR: u8 = 4;

fn exchange(device: &HidDevice, command: &[u8]) -> Result<[u8; 32], String> {
    let mut packet = [0; 33];
    packet[1..1 + command.len()].copy_from_slice(command);
    let written = device.write(&packet).map_err(|e| e.to_string())?;
    if written != packet.len() {
        return Err("Écriture HID incomplète.".into());
    }
    let mut reply = [0; 32];
    let count = device
        .read_timeout(&mut reply, 1500)
        .map_err(|e| e.to_string())?;
    if count == 0 {
        return Err("Aucune réponse sous 1,5 s. Réveille le clavier et réessaie.".into());
    }
    let header_len = if command[0] == GET_VERSION { 1 } else { 3 };
    if count != reply.len() || reply[..header_len] != command[..header_len] {
        return Err(format!(
            "Réponse inattendue ({} octets, commande 0x{:02X}).",
            count, reply[0]
        ));
    }
    Ok(reply)
}

fn read_color(device: &HidDevice) -> Result<[u8; 2], String> {
    // Two different padding values distinguish returned data from a request echo.
    let first = exchange(device, &[GET, MATRIX, COLOR, 0, 0])?;
    let second = exchange(device, &[GET, MATRIX, COLOR, 255, 255])?;
    if first[3..5] != second[3..5] {
        return Err("Lecture de teinte non confirmée par le récepteur.".into());
    }
    Ok([first[3], first[4]])
}

pub fn communicate(color: Option<[u8; 2]>) -> Result<String, String> {
    let _guard = TRANSACTION.lock().map_err(|e| e.to_string())?;
    let api = HidApi::new().map_err(|e| e.to_string())?;
    let mut matching = api.device_list().filter(|d| {
        (
            d.vendor_id(),
            d.product_id(),
            d.interface_number(),
            d.usage_page(),
            d.usage(),
        ) == (0x36b0, 0x3002, 1, 0xff60, 0x61)
    });
    let info = matching.next().ok_or("Interface Raw HID Evo75 absente.")?;
    if matching.next().is_some() {
        return Err("Plusieurs récepteurs Evo75 détectés.".into());
    }
    let device = info.open_device(&api).map_err(|e| e.to_string())?;
    let answer = exchange(&device, &[GET_VERSION])?;
    let version = u16::from_be_bytes([answer[1], answer[2]]);
    if !(11..=13).contains(&version) {
        return Err(format!(
            "Version reçue : 0x{version:04X}. Commandes couleur non activées."
        ));
    }
    let current = read_color(&device).map_err(|e| format!("VIA {version} répond. {e}"))?;
    if let Some([hue, sat]) = color {
        exchange(&device, &[SET, MATRIX, COLOR, hue, sat])?;
        if read_color(&device)? != [hue, sat] {
            return Err("Commande envoyée, mais la relecture de la teinte diffère.".into());
        }
        Ok(format!("VIA {version} · Teinte confirmée par relecture."))
    } else {
        Ok(format!(
            "VIA {version} · Teinte {} et saturation {} lues sur le clavier.",
            current[0], current[1]
        ))
    }
}
