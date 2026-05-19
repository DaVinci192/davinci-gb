use crate::core::bus::ppu::PPU_MODE;

use super::Bus;

const DIV: u16 = 0xFF04;

const ROM_LO: u16 = 0x0000; const ROM_HI: u16 = 0x7FFF;
const VRAM_LO: u16 = 0x8000; const VRAM_HI: u16 = 0x9FFF;
const RAM_LO: u16 = 0xA000; const RAM_HI: u16 = 0xBFFF; // external RAM
const WRAM_LO: u16 = 0xC000; const WRAM_HI: u16 = 0xFDFF;
const HRAM_LO: u16 = 0xFF80; const HRAM_HI: u16 = 0xFFFE;

const OAM_LO: u16 = 0xFE00; const OAM_HI: u16 = 0xFE9F;
const JOYPAD: u16 = 0xFF00;
const SERIAL_LO: u16 = 0xFF01; const SERIAL_HI: u16 = 0xFF02;
const TIMER_LO: u16 = 0xFF04; const TIMER_HI: u16 = 0xFF07;
const IF: u16 = 0xFF0F;
const APU_LO: u16 = 0xFF10; const APU_HI: u16 = 0xFF3F;
const PPU_LO0: u16 = 0xFF40; const PPU_HI0: u16 = 0xFF45;
const OAM_DMA: u16 = 0xFF46;
const PPU_LO1: u16 = 0xFF47; const PPU_HI1: u16 = 0xFF4B;
const SYS: u16 = 0xFF4C; 
const SPD: u16 = 0xFF4D;
const VRAM_BANK_SEL: u16 = 0xFF4F;
const BOOT_ROM_CTRL: u16 = 0xFF50;
const VRAM_DMA_LO: u16 = 0xFF51; const VRAM_DMA_HI: u16 = 0xFF55;
const IR: u16 = 0xFF56;
const PALETTES_LO: u16 = 0xFF68; const PALETTES_HI: u16 = 0xFF6B;
const OBJ_MODE: u16 = 0xFF6C;
const WRAM_BANK_SEL: u16 = 0xFF70;
const AUDIO_OUT_LO: u16 = 0xFF76; const AUDIO_OUT_HI: u16 = 0xFF77;
const IE: u16 = 0xFFFF;

pub struct CpuBusView<'a> {
    pub bus: &'a mut Bus,
}

const UNMAPPED_MEMORY: u8 = 0xFF;

impl CpuBusView<'_> {
    pub fn read8(&self, address: u16) -> u8 {
        if self.bus.oam_dma.is_active {
            if self.bus.oam_dma.is_reading_cartridge { // block rom/sram
                match address {
                    ROM_LO..=ROM_HI     => 0xFF,
                    RAM_LO..=RAM_HI     => 0xFF,
                    _                   => self.bus.read8(address),
                }
            } else {
                match address { // block wram
                    WRAM_LO..=WRAM_HI   => 0xFF,
                    _                   => self.bus.read8(address),
                }
            }
        } else if self.bus.ppu.mode() == PPU_MODE::DRAWING { // block vram/oam
            match address {
                VRAM_LO..=VRAM_HI       => 0xFF,
                OAM_LO..=OAM_HI         => 0xFF,
                _                       => self.bus.read8(address),
            }
        } else if self.bus.ppu.mode() == PPU_MODE::OAM_SCAN { // block oam
            match address {
                OAM_LO..=OAM_HI         => 0xFF,
                _                       => self.bus.read8(address),
            }
        } else {
            self.bus.read8(address)
        }
    }

    pub fn write8(&mut self, address: u16, val: u8) {
        if self.bus.oam_dma.is_active {
            if self.bus.oam_dma.is_reading_cartridge { // block rom/sram
                match address {
                    ROM_LO..=ROM_HI     => {},
                    RAM_LO..=RAM_HI     => {},
                    _                   => self.bus.write8(address, val),
                }
            } else {
                match address { // block wram
                    WRAM_LO..=WRAM_HI   => {},
                    _                   => self.bus.write8(address, val),
                }
            }
        } else if self.bus.ppu.mode() == PPU_MODE::DRAWING { // block vram/oam
            match address {
                VRAM_LO..=VRAM_HI       => {},
                OAM_LO..=OAM_HI         => {},
                _                       => self.bus.write8(address, val),
            }
        } else if self.bus.ppu.mode() == PPU_MODE::OAM_SCAN { // block oam
            match address {
                OAM_LO..=OAM_HI         => {},
                _                       => self.bus.write8(address, val),
            }
        } else {
            self.bus.write8(address, val);
        }
    }


    pub fn ie(&self) -> u8 {
        self.bus.ie()
    }

    pub fn iflag(&self) -> u8 {
        self.bus.iflag()
    }

    pub fn set_iflag(&mut self, val: u8) {
        self.bus.set_iflag(val);
    }
}
