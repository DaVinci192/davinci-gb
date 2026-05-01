use crate::core::bus::Interrupt;

const SB: u16 = 0xFF01;
const SC: u16 = 0xFF02;

pub struct Serial {
    sb: u8,
    sc: u8,

    pub serial_output: Vec<u8>,
}

impl Default for Serial {
    fn default() -> Self {
        Self {
            sb: 0,
            sc: 0x7E,

            serial_output: Vec::new(),
        }
    }
}


impl Serial {
    pub fn write(&mut self, address: u16, val: u8) -> Option<Interrupt> {
        if (address == SC) && (val == 0x81) {
            let byte = self.sb;
            self.serial_output.push(byte);

            print!("{}", byte as char);
            use std::io::{stdout, Write};
            stdout().flush().unwrap();

            self.sc = 0; // serial transfer complete
            return Some(Interrupt::SerialInt);
        }

        match address {
            SB => self.sb = val,
            SC => self.sc = 0b10000011 & val,
            _ => unreachable!(),
        };

        None        
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            SB => self.sb,
            SC => self.sc,
            _ => unreachable!(),
        }
    }
}