#[cfg(not(windows))]
compile_error!("Evo75-KeyLab cible Windows.");

#[cfg(windows)]
fn main() -> Result<(), hidapi::HidError> {
    use hidapi::HidApi;

    const RECEIVER: (u16, u16) = (0x36B0, 0x3002);
    const QMK_RAW_HID: (u16, u16) = (0xFF60, 0x61);

    let api = HidApi::new()?;
    let mut found_receiver = false;
    let mut found_raw_hid = false;

    println!("Recherche du récepteur EVO75 2.4G (36B0:3002)...");
    for device in api.device_list() {
        if (device.vendor_id(), device.product_id()) != RECEIVER {
            continue;
        }

        found_receiver = true;
        let usage = (device.usage_page(), device.usage());
        let is_raw_hid = usage == QMK_RAW_HID;
        found_raw_hid |= is_raw_hid;
        println!(
            "{} | interface {} | page 0x{:04X} | usage 0x{:04X}{}",
            device.product_string().unwrap_or("Produit sans nom"),
            device.interface_number(),
            usage.0,
            usage.1,
            if is_raw_hid { " | QMK Raw HID" } else { "" }
        );
    }

    if !found_receiver {
        println!("Aucun récepteur 36B0:3002 détecté.");
    } else if found_raw_hid {
        println!("Collection QMK Raw HID détectée. La prise en charge de VIA reste à vérifier.");
    } else {
        println!("Récepteur détecté. Aucune collection avec l’usage QMK Raw HID par défaut.");
    }

    Ok(())
}
