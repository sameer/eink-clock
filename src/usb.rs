use crate::{KINDLE_PRODUCT_ID, KINDLE_VENDOR_ID};

use rusb::{Device, GlobalContext, Result};

pub fn get_kindle() -> Result<Option<Device<GlobalContext>>> {
    for device in rusb::devices()?.iter() {
        let desc = device.device_descriptor()?;
        if desc.vendor_id() == KINDLE_VENDOR_ID && desc.product_id() == KINDLE_PRODUCT_ID {
            return Ok(Some(device));
        }
    }
    Ok(None)
}

pub fn reset_kindle(kindle: &Device<GlobalContext>) -> Result<()> {
    kindle.open().and_then(|mut handle| handle.reset())
}
