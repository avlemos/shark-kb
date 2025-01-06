use chrono::prelude::*;

pub(crate) struct TimeHexGenerator {
    prefix: [u8; 7],
    suffix: [u8; 50],
}

//28000000000000 ?? yyyy mm dd       hh mm ss
//28000000000000 d7 07e8 0b 06       0a 1d 1d   00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
impl TimeHexGenerator {
    pub fn new() -> Self {
        TimeHexGenerator {
            prefix: [0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            suffix: [0x00; 50],
        }
    }

    pub fn generate_hex(&self) -> Vec<u8> {
        // Get the current UTC date and time
        let now = Utc::now();
        let year = now.year();
        let month = now.month();
        let day = now.day();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();

        // Create the dynamic part of the message
        let mut message = vec![];
        message.extend_from_slice(&self.prefix);
        message.push(0xd7); // Constant byte
        message.push(year as u8);
        message.push(month as u8);
        message.push(day as u8);
        message.push(hour as u8);
        message.push(minute as u8);
        message.push(second as u8);
        message.extend_from_slice(&self.suffix);

        message
    }

}
