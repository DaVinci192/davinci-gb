use super::super::super::core::cpu::flags::Flags;
use super::super::super::core::cpu::Op;
use super::super::super::core::cpu::CPU;

pub struct CPUSnapshot {
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

    pub halted: bool,
    pub halt_bug: bool,

    pub temp: u8,
    pub temp16: u16,
    
    pub step: u8,

    pub current_op: Op,

    pub m_cycles: u64,
}

impl From<&CPU> for CPUSnapshot {
    fn from (cpu: &CPU) -> Self {
        Self {
            a: cpu.a,
            f: cpu.f,

            b: cpu.b,
            c: cpu.c,
            d: cpu.d,
            e: cpu.e,
            h: cpu.h,
            l: cpu.l,

            sp: cpu.sp,
            pc: cpu.pc,

            ir: cpu.ir,
            cb_prefix: cpu.cb_prefix,

            ime: cpu.ime,
            ime_scheduled: cpu.ime_scheduled,

            halted: cpu.halted,
            halt_bug: cpu.halt_bug,

            temp: cpu.temp,
            temp16: cpu.temp16,

            step: cpu.step,

            current_op: cpu.current_op,

            m_cycles: cpu.m_cycles,
        }
    }
}