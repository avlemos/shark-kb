extern crate hidapi;

use hidapi::{HidApi, HidDevice};

mod payload;
use payload::TimeHexGenerator;

/*
you can use hid_device_list() to list all the connected HID devices in case you need to make changes
and USBDeview to assist (https://www.nirsoft.net/utils/usb_devices_view.html).
Since this keyboard has multiple interfaces, you need to specify which will accept the HID message.
*/
const VENDOR_ID: u16 = 0x3151;
const PRODUCT_ID: u16 = 0x4015;
const INTERFACE_NUMBER: i32 = 2;

fn main() {
    let hex_message = TimeHexGenerator::new().generate_hex();
    match find_device() {
        Ok(d) => d
            .send_feature_report(&hex_message)
            .expect("Failed to write data"),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
}

fn find_device() -> Result<HidDevice, String> {
    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                if device.vendor_id() == VENDOR_ID
                    && device.product_id() == PRODUCT_ID
                    && device.interface_number() == INTERFACE_NUMBER
                {
                    // Return the matching device
                    return Ok(device.open_device(&api).unwrap());
                }
            }
            // Return an error if no matching device is found
            Err("Device not found :-(".to_string())
        }
        Err(e) => {
            // Return an error if we couldn't initialize HidApi
            Err(format!("Error initializing HidApi: {}", e))
        }
    }
}
