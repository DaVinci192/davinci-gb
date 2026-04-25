pub mod misc_control;
pub mod jumps_calls;
pub mod load;
pub mod stack;
pub mod arithmetic_8;
pub mod arithmetic_16;
pub mod logic_8; 
pub mod shift;
pub mod bit;

// use misc_control::*;
// use jumps_calls::*;
// use load::*;
// use stack::*;
// use arithmetic_16::*;
// use arithmetic_8::*;
// use logic_8::*;
// use shift::*;
// use bit::*;

use super::CPU;
use super::registers::{Reg16};

pub fn op_read_n8(cpu: &mut CPU) {
    cpu.temp = cpu.bus.read8(cpu.pc);
    cpu.pc = cpu.pc.wrapping_add(1);
    cpu.step += 1;
}

pub fn op_read_hl(cpu: &mut CPU) {
    let address = cpu.get_reg16(Reg16::HL);
    cpu.temp = cpu.bus.read8(address);
    cpu.step += 1;
}

pub fn op_write_hl(cpu: &mut CPU) {
    let address = cpu.get_reg16(Reg16::HL);
    cpu.bus.write8(address, cpu.temp);
    cpu.step += 1;
}

pub fn op_write_r16(cpu: &mut CPU, reg: Reg16) {
    let address = cpu.get_reg16(reg);
    cpu.bus.write8(address, cpu.temp);
    cpu.step += 1;
}

pub fn op_fetch_next(cpu: &mut CPU) {
    let opcode = cpu.bus.read8(cpu.pc);

    cpu.ir = opcode;

    cpu.pc = cpu.pc.wrapping_add(1);

    cpu.step = 0;
}