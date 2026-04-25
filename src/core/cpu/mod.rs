pub mod registers;
pub mod decoder;
pub mod execute;
pub mod alu;
pub mod core;
pub mod instructions;
pub mod flags;

use flags::{Flags};
use super::bus::Bus;

pub type Op = fn(&mut CpuExec);

pub struct CPU {
    // internal registers
    pub a: u8, // accumulator
    pub f: Flags, // flag register

    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub sp: u16,
    pub pc: u16,

    pub ir: u8,
    pub cb_prefix: bool,

    pub ime: bool,
    pub ime_scheduled: bool,
    pub ie: u8,
    pub iflag: u8,

    pub halted: bool,
    pub halt_bug: bool,

    pub temp: u8,
    pub temp16: u16,
    
    pub step: u8,

    pub current_op: Op,

    pub m_cycles: u64,
    pub t_cycles: u64,
}

pub struct CpuExec<'a> {
    pub cpu: &'a mut CPU,
    pub bus: &'a mut Bus,
}