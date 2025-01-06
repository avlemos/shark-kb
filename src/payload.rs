use chrono::prelude::*;

pub(crate) struct TimeHexGenerator {
    prefix: [u8; 8],
    suffix: [u8; 45],
}

impl TimeHexGenerator {
    pub fn new() -> Self {
        TimeHexGenerator {
            prefix: [0u8, 0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            suffix: [0x00; 45],
        }
    }

    pub fn generate_hex(&self) -> Vec<u8> {
        // Get the current date and time
        let now = Local::now();
        let year: u16 = now.year() as u16;
        let month: u8 = now.month() as u8;
        let day: u8 = now.day() as u8;
        let hour: u8 = now.hour() as u8;
        let minute: u8 = now.minute() as u8;
        let second: u8 = now.second() as u8;

        // Create the dynamic part of the message
        let mut message = vec![];
        message.extend_from_slice(&self.prefix);
        message.push(0xd7); // Constant byte
        message.push((year >> 8) as u8);
        message.push((year & 0xFF) as u8);
        message.push(month as u8);
        message.push(day as u8);
        message.push(hour as u8);
        message.push(minute as u8);
        message.push(second as u8);
        message.extend_from_slice(&self.suffix);
        // let hex_string: String = message.iter().map(|byte| format!("{:02x}", byte)).collect();
        // println!("Hex array: {}", hex_string);
        message
    }

}
