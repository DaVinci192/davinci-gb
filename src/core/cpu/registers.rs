use super::CPU;

#[derive(Copy, Clone, Debug)]
pub enum Reg {
    A, 
    B, 
    C, 
    D, 
    E, 
    H, 
    L,
}

#[derive(Copy, Clone, Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
}


impl CPU {

    pub fn get_reg(&self, reg: Reg) -> u8 {
        match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
        }
    }

    pub fn set_reg(&mut self, reg: Reg, val: u8) {
        match reg {
            Reg::A => self.a = val,
            Reg::B => self.b = val,
            Reg::C => self.c = val,
            Reg::D => self.d = val,
            Reg::E => self.e = val,
            Reg::H => self.h = val,
            Reg::L => self.l = val,
        }
    }

    pub fn get_reg16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => ((self.a as u16) << 8) | (self.f.as_u8() as u16),
            Reg16::BC => ((self.b as u16) << 8) | (self.c as u16),
            Reg16::DE => ((self.d as u16) << 8) | (self.e as u16),
            Reg16::HL => ((self.h as u16) << 8) | (self.l as u16),
        }
    }

    pub fn set_reg16(&mut self, reg: Reg16, val: u16) {
        match reg {
            Reg16::AF => {
                self.a = ((val | 0xFF00) >> 8) as u8;
                self.f = super::Flags::from_u8(((val | 0x00FF) as u8) & 0xF0);
            },
            Reg16::BC => {
                self.b = ((val | 0xFF00) >> 8) as u8;
                self.c = (val | 0x00FF) as u8;
            },
            Reg16::DE => {
                self.d = ((val | 0xFF00) >> 8) as u8;
                self.e = (val | 0x00FF) as u8;
            },
            Reg16::HL => {
                self.h = ((val | 0xFF00) >> 8) as u8;
                self.l = (val | 0x00FF) as u8;
            },
        }
    }
}