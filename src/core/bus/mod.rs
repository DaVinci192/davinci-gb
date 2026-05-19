pub mod joypad;
pub mod serial;
pub mod timer;
pub mod ppu;
pub mod vram;
pub mod oam;
pub mod cram;
pub mod cartridge;
pub mod wram;
pub mod oam_dma;
pub mod vram_dma;
pub mod hram;
pub mod apu;
pub mod ir;

pub mod cpu_bus_view;
// pub mod audio;
// pub mod wave_pattern;
// pub mod lcd;
// pub mod dma;
// pub mod key;
// pub mod vram_dma;
// pub mod ir;
// pub mod palettes;
// pub mod wram_bank;

use super::bus::joypad::Joypad;
use super::bus::serial::Serial;
use super::bus::timer::Timer;
use super::bus::vram::Vram;
use super::bus::cram::Cram;
use super::bus::wram::Wram;
use super::bus::oam::OAM;
use super::bus::oam_dma::OamDma;
use super::bus::cartridge::Cartridge;
use super::bus::ppu::PPU;
use super::bus::hram::Hram;
use super::bus::vram_dma::VramDma;
use super::bus::apu::APU;
use super::bus::ir::IR;

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

pub enum Interrupt {
    VblankInt,
    LcdInt,
    TimerInt,
    SerialInt,
    JoypadInt,
}

pub struct Bus {
    pub memory: [u8; 0x10000], 

    iflag: u8,
    ie: u8,

    sys: u8,
    spd: u8,

    bank: u8,

    opri: u8,

    // serial  
    // pub serial_output: Vec<u8>,

    pub cartridge: Cartridge,
    pub joypad: Joypad,
    pub serial: Serial,
    pub timer: Timer,
    pub vram: Vram,
    pub cram: Cram,
    pub wram: Wram,
    pub oam: OAM,
    pub oam_dma: OamDma,
    pub ppu: PPU,
    pub hram: Hram,
    pub vram_dma: VramDma,
    pub apu: APU,
    pub ir: IR,

}

const UNMAPPED_MEMORY: u8 = 0xFF;

impl Bus {
    pub fn read8(&self, address: u16) -> u8 {
        match address {
            ROM_LO..=ROM_HI             => self.cartridge.read(address),
            VRAM_LO..=VRAM_HI           => self.vram.read(address),
            RAM_LO..=RAM_HI             => self.cartridge.read(address),
            WRAM_LO..=WRAM_HI           => self.wram.read(address),          // includes echo ram (bank 0 mirror) - unusable
            OAM_LO..=OAM_HI             => self.oam.read(address),
            0xFEA0..=0xFEFF             => UNMAPPED_MEMORY,                      // "use of this area is prohibited"
            JOYPAD                      => self.joypad.read(address),
            SERIAL_LO..=SERIAL_HI       => self.serial.read(address),
            0xFF03                      => UNMAPPED_MEMORY,                      // ????
            TIMER_LO..=TIMER_HI         => self.timer.read(address),
            0xFF08..=0xFF0E             => UNMAPPED_MEMORY,                      // ???
            IF                          => self.iflag,
            APU_LO..=APU_HI             => self.apu.read(address),
            PPU_LO0..=PPU_HI0           => self.ppu.read(address),
            OAM_DMA                     => self.oam_dma.read(address),
            PPU_LO1..=PPU_HI1           => self.ppu.read(address),
            SYS                         => self.sys,
            SPD                         => self.spd,
            0xFF4E                      => UNMAPPED_MEMORY,                      // ??????
            VRAM_BANK_SEL               => self.vram.read(address),
            BOOT_ROM_CTRL               => self.bank,
            VRAM_DMA_LO..=VRAM_DMA_HI   => self.vram_dma.read(address),
            IR                          => self.ir.read(address),
            0xFF57..=0xFF67             => UNMAPPED_MEMORY,                      // ??????
            PALETTES_LO..=PALETTES_HI   => self.cram.read(address),
            OBJ_MODE                    => self.opri,
            0xFF6D..=0xFF6F             => UNMAPPED_MEMORY,                      // ???????
            WRAM_BANK_SEL               => self.wram.read(address),
            0xFF71..=0xFF75             => UNMAPPED_MEMORY,                      // ???????
            AUDIO_OUT_LO..=AUDIO_OUT_HI => self.apu.read(address),
            0xFF78..=0xFF7F             => UNMAPPED_MEMORY,                      // ???????
            HRAM_LO..=HRAM_HI           => self.hram.read(address),
            IE                          => self.ie,
        }
    }


    pub fn write8(&mut self, address: u16, val: u8) {
        match address {
            ROM_LO..=ROM_HI             => self.cartridge.write(address, val),
            VRAM_LO..=VRAM_HI           => self.vram.write(address, val),
            RAM_LO..=RAM_HI             => self.cartridge.write(address, val),
            WRAM_LO..=WRAM_HI           => self.wram.write(address, val),          // includes echo ram (bank 0 mirror) - unusable
            OAM_LO..=OAM_HI             => self.oam.write(address, val),
            0xFEA0..=0xFEFF             => {},                      // "use of this area is prohibited"
            JOYPAD                      => self.joypad.write(address, val),
            SERIAL_LO..=SERIAL_HI       => if let Some(irq) = self.serial.write(address, val) {self.request_interrupt(irq);},
            0xFF03                      => {},                      // ????
            TIMER_LO..=TIMER_HI         => self.timer.write(address, val),
            0xFF08..=0xFF0E             => {},                      // ???
            IF                          => self.iflag = val,
            APU_LO..=APU_HI             => self.apu.write(address, val),
            PPU_LO0..=PPU_HI0           => self.ppu.write(address, val),
            OAM_DMA                     => self.oam_dma.write(address, val),
            PPU_LO1..=PPU_HI1           => self.ppu.write(address, val),
            SYS                         => self.sys = val, // TODO check when write valid
            SPD                         => self.spd = val, // TODO check when write valid
            0xFF4E                      => {},                      // ??????
            VRAM_BANK_SEL               => self.vram.write(address, val),
            BOOT_ROM_CTRL               => self.bank = val, 
            VRAM_DMA_LO..=VRAM_DMA_HI   => self.vram_dma.write(address, val),
            IR                          => self.ir.write(address, val),
            0xFF57..=0xFF67             => {},                      // ??????
            PALETTES_LO..=PALETTES_HI   => self.cram.write(address, val),
            OBJ_MODE                    => self.opri = val,
            0xFF6D..=0xFF6F             => {},                      // ???????
            WRAM_BANK_SEL               => self.wram.write(address, val),
            0xFF71..=0xFF75             => {},                      // ???????
            AUDIO_OUT_LO..=AUDIO_OUT_HI => self.apu.write(address, val),
            0xFF78..=0xFF7F             => {},                      // ???????
            HRAM_LO..=HRAM_HI           => self.hram.write(address, val),
            IE                          => self.ie = val,
        }
    }

    pub fn tick_timer(&mut self) {
        if let Some(irq) = self.timer.tick() {
            self.request_interrupt(irq);
        }
    }

    pub fn request_interrupt(&mut self, irq: Interrupt) {
        match irq {
            Interrupt::VblankInt => self.iflag |= 0x01,
            Interrupt::LcdInt    => self.iflag |= 0x02,
            Interrupt::TimerInt  => self.iflag |= 0x04,
            Interrupt::SerialInt => self.iflag |= 0x08,
            Interrupt::JoypadInt => self.iflag |= 0x10,
        }
    }
}

impl Bus {
    pub fn new() -> Self {
        let mut b = Self { 
            memory: [0; 0x10000],

            iflag: 0xE1,
            ie: 0,

            sys: 0,
            spd: 0,

            bank: 0,

            opri: 0,

            joypad: Joypad::default(),
            serial: Serial::default(),
            timer: Timer::new(),
            vram: Vram::new(),
            cram: Cram::new(),

            cartridge: Cartridge::new(),
            wram: Wram::new(),
            oam: OAM::new(),
            oam_dma: OamDma::new(),
            ppu: PPU::new(),
            hram: Hram::new(),
            vram_dma: VramDma::new(),
            apu: APU::new(),
            ir: IR::new(),
        };

        b.memory[0xFF01] = 0x00; // SB
        b.memory[0xFF02] = 0x7E; // SC

        b.memory[0xFF05] = 0; // TIMA
        b.memory[0xFF06] = 0; // TMA
        b.memory[0xFF07] = 0xF8; // TAC

        b.memory[0xFF0F] = 0xE1; // IF

        b
    }

    pub fn reset(&mut self) {
        *self = Self::new();

        // println!("FF02: {}", self.memory[0xFF02]);
    }

    pub fn ie(&self) -> u8 {
        self.ie
    }

    pub fn iflag(&self) -> u8 {
        self.iflag
    }

    pub fn set_iflag(&mut self, val: u8) {
        self.iflag = val;
    }
}