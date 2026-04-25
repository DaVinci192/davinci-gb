

#[derive(Clone, Copy)]
pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool
}


impl Flags {
    pub fn as_u8(&self) -> u8 {
        ((self.z as u8) << 7) | 
        ((self.n as u8) << 6) | 
        ((self.h as u8) << 5) | 
        ((self.c as u8) << 4)
    }

    pub fn from_u8(val: u8) -> Self {
        Flags {
            z: 0x80 & val != 0,
            n: 0x40 & val != 0,
            h: 0x20 & val != 0,
            c: 0x10 & val != 0,
        }
    } 
}

