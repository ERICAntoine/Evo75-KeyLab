use hidapi::{HidApi, HidError};
use serde::Serialize;

#[derive(Serialize)]
pub struct Collection {
    pub product: String,
    pub interface: i32,
    pub usage_page: u16,
    pub usage: u16,
}

pub fn scan_receiver() -> Result<Vec<Collection>, HidError> {
    let api = HidApi::new()?;
    let mut collections: Vec<_> = api
        .device_list()
        .filter(|device| (device.vendor_id(), device.product_id()) == (0x36B0, 0x3002))
        .map(|device| Collection {
            product: device.product_string().unwrap_or("EVO75 2.4G").to_owned(),
            interface: device.interface_number(),
            usage_page: device.usage_page(),
            usage: device.usage(),
        })
        .collect();
    collections.sort_by_key(|item| (item.interface, item.usage_page, item.usage));
    Ok(collections)
}
