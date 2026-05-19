const NR10: u16 = 0xFF10;
const NR11: u16 = 0xFF11;
const NR12: u16 = 0xFF12;
const NR13: u16 = 0xFF13;
const NR14: u16 = 0xFF14;

const NR21: u16 = 0xFF16;
const NR22: u16 = 0xFF17;
const NR23: u16 = 0xFF18;
const NR24: u16 = 0xFF19;

const NR30: u16 = 0xFF1A;
const NR31: u16 = 0xFF1B;
const NR32: u16 = 0xFF1C;
const NR33: u16 = 0xFF1D;
const NR34: u16 = 0xFF1E;

const NR41: u16 = 0xFF20;
const NR42: u16 = 0xFF21;
const NR43: u16 = 0xFF22;
const NR44: u16 = 0xFF23;

const NR50: u16 = 0xFF24;
const NR51: u16 = 0xFF25;
const NR52: u16 = 0xFF26;

const WRAM_LO: u16 = 0xFF30;
const WRAM_HI: u16 = 0xFF3F;

const PCM12: u16 = 0xFF76;
const PCM34: u16 = 0xFF77;

pub struct APU {
    nr10: u8,
    nr11: u8,
    nr12: u8,
    nr13: u8,
    nr14: u8,

    nr21: u8,
    nr22: u8,
    nr23: u8,
    nr24: u8,

    nr30: u8,
    nr31: u8,
    nr32: u8,
    nr33: u8,
    nr34: u8,

    nr41: u8,
    nr42: u8,
    nr43: u8,
    nr44: u8,

    nr50: u8,
    nr51: u8,
    nr52: u8,

    wram: [u8; 0x10],

    pcm12: u8,
    pcm34: u8,
}

// TODO apu not implemented yet, only basic hardware registers

impl APU {
    pub fn new() -> Self {
        Self {
            nr10: 0x80,
            nr11: 0xBF,
            nr12: 0xF3,
            nr13: 0xFF,
            nr14: 0xBF,

            nr21: 0x3F,
            nr22: 0x00,
            nr23: 0xFF,
            nr24: 0xBF,

            nr30: 0x7F,
            nr31: 0xFF,
            nr32: 0x9F,
            nr33: 0xFF,
            nr34: 0xBF,

            nr41: 0xFF,
            nr42: 0x00,
            nr43: 0x00,
            nr44: 0xBF,

            nr50: 0x77,
            nr51: 0xF3,
            nr52: 0xF1,

            wram: [0; 0x10],

            pcm12: 0x00,
            pcm34: 0x00,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            NR10 => self.nr10,
            NR11 => self.nr11,
            NR12 => self.nr12,
            NR13 => self.nr13,
            NR14 => self.nr14,
            0xFF15 => 0xFF, // unmapped/unknown
            NR21 => self.nr21,
            NR22 => self.nr22,
            NR23 => self.nr23,
            NR24 => self.nr24,
            NR30 => self.nr30,
            NR31 => self.nr31,
            NR32 => self.nr32,
            NR33 => self.nr33,
            NR34 => self.nr34,
            0xFF1F => 0xFF, // unmapped/unknown
            NR41 => self.nr41,
            NR42 => self.nr42,
            NR43 => self.nr43,
            NR44 => self.nr44,
            NR50 => self.nr50,
            NR51 => self.nr51,
            NR52 => self.nr52,
            0xFF27..=0xFF2F => 0xFF, // unmapped/unknown
            WRAM_LO..=WRAM_HI => self.wram[(address - WRAM_LO) as usize],
            PCM12 => self.pcm12,
            PCM34 => self.pcm34,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, address: u16, val: u8) {
        match address {
            NR10 => self.nr10 = val,
            NR11 => self.nr11 = val,
            NR12 => self.nr12 = val,
            NR13 => self.nr13 = val,
            NR14 => self.nr14 = val,
            0xFF15 => {}, // unmapped/unknown
            NR21 => self.nr21 = val,
            NR22 => self.nr22 = val,
            NR23 => self.nr23 = val,
            NR24 => self.nr24 = val,
            NR30 => self.nr30 = val,
            NR31 => self.nr31 = val,
            NR32 => self.nr32 = val,
            NR33 => self.nr33 = val,
            NR34 => self.nr34 = val,
            0xFF1F => {}, // unmapped/unknown
            NR41 => self.nr41 = val,
            NR42 => self.nr42 = val,
            NR43 => self.nr43 = val,
            NR44 => self.nr44 = val,
            NR50 => self.nr50 = val,
            NR51 => self.nr51 = val,
            NR52 => self.nr52 = val,
            0xFF27..=0xFF2F => {}, // unmapped/unknown
            WRAM_LO..=WRAM_HI => self.wram[(address - WRAM_LO) as usize] = val,
            PCM12 => {}, // read only
            PCM34 => {}, // read only
            _ => unreachable!(),

        }
    }
}