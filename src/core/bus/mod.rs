pub mod joypad;
pub mod serial;
pub mod timer;
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


const DIV: u16 = 0xFF04;

use core::ops::RangeInclusive;
const JOYPAD: u16 = 0xFF00;
const SERIAL_LO: u16 = 0xFF01; const SERIAL_HI: u16 = 0xFF02;
const TIMER_LO: u16 = 0xFF04; const TIMER_HI: u16 = 0xFF07;
const IF: u16 = 0xFF0F;
const AUDIO_LO: u16 = 0xFF10; const AUDIO_HI: u16 = 0xFF26;
const WAVE_LO: u16 = 0xFF30; const WAVE_HI: u16 = 0xFF3F;
const LCD_LO: u16 = 0xFF40; const LCD_HI: u16 = 0xFF4B;
const KEY_LO: u16 = 0xFF4C; const KEY_HI: u16 = 0xFF4D;
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

    // serial  
    // pub serial_output: Vec<u8>,

    pub joypad: Joypad,
    pub serial: Serial,
    pub timer: Timer,
}

impl Bus {
    fn read_temp(&self, address: u16) -> u8 { self.memory[address as usize] }
    fn write_temp(&mut self, address: u16, val: u8) { self.memory[address as usize] = val; }

    pub fn read8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0xFEFF             => self.read_temp(address),
            JOYPAD                      => self.joypad.read(address),
            SERIAL_LO..=SERIAL_HI       => self.serial.read(address),
            0xFF03                      => self.read_temp(address),                      // ????
            TIMER_LO..=TIMER_HI         => self.timer.read(address),
            0xFF08..=0xFF0E             => self.read_temp(address),                      // ???
            IF                          => self.iflag,
            AUDIO_LO..=AUDIO_HI         => self.read_temp(address),
            0xFF27..=0xFF2F             => self.read_temp(address),                      // ?????
            WAVE_LO..=WAVE_HI           => self.read_temp(address),
            LCD_LO..=LCD_HI             => self.read_temp(address),                      // DMA INSIDE THIS RANGE
                                                                                  // 0xFF46 => self.read_temp(address),
            KEY_LO..=KEY_HI             => self.read_temp(address),
            0xFF4E                      => self.read_temp(address),                      // ??????
            VRAM_BANK_SEL               => self.read_temp(address),
            BOOT_ROM_CTRL               => self.read_temp(address),
            VRAM_DMA_LO..=VRAM_DMA_HI   => self.read_temp(address),
            IR                          => self.read_temp(address),
            0xFF57..=0xFF67             => self.read_temp(address),                      // ??????
            PALETTES_LO..=PALETTES_HI   => self.read_temp(address),
            OBJ_MODE                    => self.read_temp(address),
            0xFF6D..=0xFF6F             => self.read_temp(address),                      // ???????
            WRAM_BANK_SEL               => self.read_temp(address),
            0xFF71..=0xFF75             => self.read_temp(address),                      // ???????
            AUDIO_OUT_LO..=AUDIO_OUT_HI => self.read_temp(address),
            0xFF78..=0xFF7F             => self.read_temp(address),                      // ???????
            0xFF80..=0xFFFE             => self.memory[address as usize],   // HRAM
            IE                          => self.ie,
        }

        // self.memory[address as usize]
    }

    pub fn write8(&mut self, address: u16, val: u8) {
        match address {
            0x0000..=0xFEFF             => self.memory[address as usize] = val,
            JOYPAD                      => self.joypad.write(address, val),
            SERIAL_LO..=SERIAL_HI       => if let Some(irq) = self.serial.write(address, val) {self.request_interrupt(irq);},
            0xFF03                      => self.write_temp(address, val),                      // ????
            TIMER_LO..=TIMER_HI         => self.timer.write(address, val),
            0xFF08..=0xFF0E             => self.write_temp(address, val),                      // ???
            IF                          => self.iflag = val,
            AUDIO_LO..=AUDIO_HI         => self.write_temp(address, val),
            0xFF27..=0xFF2F             => self.write_temp(address, val),                      // ?????
            WAVE_LO..=WAVE_HI           => self.write_temp(address, val),
            LCD_LO..=LCD_HI             => self.write_temp(address, val),                      // DMA INSIDE THIS RANGE
                                                                                  // 0xFF46 => self.write_temp(address, val),
            KEY_LO..=KEY_HI             => self.write_temp(address, val),
            0xFF4E                      => self.write_temp(address, val),                      // ??????
            VRAM_BANK_SEL               => self.write_temp(address, val),
            BOOT_ROM_CTRL               => self.write_temp(address, val),
            VRAM_DMA_LO..=VRAM_DMA_HI   => self.write_temp(address, val),
            IR                          => self.write_temp(address, val),
            0xFF57..=0xFF67             => self.write_temp(address, val),                      // ??????
            PALETTES_LO..=PALETTES_HI   => self.write_temp(address, val),
            OBJ_MODE                    => self.write_temp(address, val),
            0xFF6D..=0xFF6F             => self.write_temp(address, val),                      // ???????
            WRAM_BANK_SEL               => self.write_temp(address, val),
            0xFF71..=0xFF75             => self.write_temp(address, val),                      // ???????
            AUDIO_OUT_LO..=AUDIO_OUT_HI => self.write_temp(address, val),
            0xFF78..=0xFF7F             => self.write_temp(address, val),                      // ???????
            0xFF80..=0xFFFE             => self.memory[address as usize] = val,   // HRAM
            IE                          => self.ie = val,
        }


        // self.memory[address as usize] = val;

        // if address == 0xFF01 {
        //     println!("SB write: {:02X}", val);
        // }

        // if address == 0xFF02 {
        //     println!("SC write: {:02X}", val);
        // }

        // div register handling
        // if address == DIV {
        //     self.div = 0;
        // }


        // if (address == 0xFF02) && (val == 0x81) {
        //     let byte = self.memory[0xFF01];
        //     self.serial_output.push(byte);

        //     print!("{}", byte as char);
        //     use std::io::{stdout, Write};
        //     stdout().flush().unwrap();

        //     self.memory[0xFF02] = 0x00; // serial transfer complete
        //     self.memory[0xFF0F] |= 0x08u8; // request serial interrupt
        // }
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

            joypad: Joypad::default(),
            serial: Serial::default(),
            timer: Timer::new(),
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