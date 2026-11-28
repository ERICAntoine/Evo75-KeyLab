use hidapi::{DeviceInfo, HidApi, HidError};
use serde::Serialize;

pub fn evo75_connection(device: &DeviceInfo) -> Option<&'static str> {
    if device.vendor_id() != 0x36b0 {
        return None;
    }
    if device.product_id() == 0x3002 {
        return Some("2,4 GHz");
    }
    let product = device.product_string()?.to_ascii_lowercase();
    let name: String = product.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    name.contains("evo75").then_some("USB")
}

#[derive(Serialize)]
pub struct Collection {
    pub product: String,
    pub connection: &'static str,
    pub product_id: u16,
    pub interface: i32,
    pub usage_page: u16,
    pub usage: u16,
}

pub fn scan_receiver() -> Result<Vec<Collection>, HidError> {
    let api = HidApi::new()?;
    let mut collections: Vec<_> = api
        .device_list()
        .filter_map(|device| {
            Some(Collection {
                product: device.product_string().unwrap_or("EVO75 2.4G").to_owned(),
                connection: evo75_connection(device)?,
                product_id: device.product_id(),
                interface: device.interface_number(),
                usage_page: device.usage_page(),
                usage: device.usage(),
            })
        })
        .collect();
    collections.sort_by_key(|item| (item.interface, item.usage_page, item.usage));
    Ok(collections)
}
