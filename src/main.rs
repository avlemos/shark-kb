extern crate hidapi;

use hidapi::HidApi;

mod payload;
use payload::TimeHexGenerator;

/*
you can use hid_device_list() to list all the connected HID devices in case you need to make changes
and USBDeview to assist (https://www.nirsoft.net/utils/usb_devices_view.html)
*/
const VENDOR_ID: u16 = 0x3151;
const PRODUCT_ID: u16 = 0x4015;

fn main() {
    let api = HidApi::new().unwrap();
    // Connect to device using its VID and PID
    let (vid, pid) = (VENDOR_ID, PRODUCT_ID);
    let device = api.open(vid, pid).unwrap();
    let hex_message = TimeHexGenerator::new().generate_hex();

    println!("{:?}", hex_message);
    let res = device.send_feature_report(&hex_message).expect("Failed to write data");
    println!("Wrote: {:?} byte(s)", res);
}

fn hid_device_list() {
    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                println!("{:04x}:{:04x}", device.vendor_id(), device.product_id());
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
