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

use super::CpuExec;
use super::registers::{Reg16};

pub fn op_read_n8(ctx: &mut CpuExec) {
    ctx.cpu.temp = ctx.bus.read8(ctx.cpu.pc);
    ctx.cpu.pc = ctx.cpu.pc.wrapping_add(1);
    ctx.cpu.step += 1;
}

pub fn op_read_hl(ctx: &mut CpuExec) {
    let address = ctx.cpu.get_reg16(Reg16::HL);
    ctx.cpu.temp = ctx.bus.read8(address);
    ctx.cpu.step += 1;
}

pub fn op_write_hl(ctx: &mut CpuExec) {
    let address = ctx.cpu.get_reg16(Reg16::HL);
    ctx.bus.write8(address, ctx.cpu.temp);
    ctx.cpu.step += 1;
}

pub fn op_write_r16(ctx: &mut CpuExec, reg: Reg16) {
    let address = ctx.cpu.get_reg16(reg);
    ctx.bus.write8(address, ctx.cpu.temp);
    ctx.cpu.step += 1;
}

pub fn op_fetch_next(ctx: &mut CpuExec) {
    let opcode = ctx.bus.read8(ctx.cpu.pc);

    ctx.cpu.ir = opcode;

    ctx.cpu.pc = ctx.cpu.pc.wrapping_add(1);

    if ctx.cpu.pc == 0xFFFF {
        println!("PC has reached 0xFFFF - M-cycles: {}", ctx.cpu.m_cycles);
        std::process::exit(1);
    }

    ctx.cpu.step = 0;
}