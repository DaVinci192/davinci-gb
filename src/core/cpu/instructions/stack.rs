use crate::core::cpu::instructions::op_fetch_next;

use super::super::alu;

use super::{op_read_n8};

use super::super::registers::{Reg16};
use super::super::CpuExec;

/* ADD_hl_sp */

pub fn op_add_hl_sp(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            alu::alu_add16(ctx.cpu, ctx.cpu.sp);
            ctx.cpu.step += 1;
        },
        2 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* ADD_sp_e8 */

pub fn op_add_sp_e8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => { op_read_n8(ctx); },
        2 => { 
            alu::alu_add_sp_e8(ctx.cpu, ctx.cpu.temp);
            ctx.cpu.step += 1;
        },
        3 => {
            // extra cycle since previous spans 2 cycles
            // this would be MSB operation
            ctx.cpu.step += 1;
        },
        4 => {
            // write result sp <- temp16
            // already done so not necessary
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* DEC_sp */

pub fn op_dec_sp(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.sp = alu::alu_dec16(ctx.cpu.sp);
            ctx.cpu.step += 1;
        },
        2 => {
            // extra cycle since IDU also used for fetch
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* INC_sp */

pub fn op_inc_sp(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.sp = alu::alu_inc16(ctx.cpu.sp);
            ctx.cpu.step += 1;
        },
        2 => {
            // extra cycle since IDU also used for fetch
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}


/* LD_sp_n16 */

pub fn op_ld_sp_n16(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_n8(ctx);
            ctx.cpu.temp16 = ctx.cpu.temp as u16;
        },
        2 => {
            op_read_n8(ctx);
            ctx.cpu.temp16 = ctx.cpu.temp16 | ((ctx.cpu.temp as u16) << 8);
        },
        3 => {
            ctx.cpu.sp = ctx.cpu.temp16;
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}


/* LD_n16_sp */

pub fn op_ld_n16_sp(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            op_read_n8(ctx);
            ctx.cpu.temp16 = ctx.cpu.temp as u16;
        },
        2 => {
            op_read_n8(ctx);
            ctx.cpu.temp16 = ctx.cpu.temp16 | ((ctx.cpu.temp as u16) << 8);
        },
        3 => {
            let lsb = (0x00FF & ctx.cpu.sp) as u8;
            ctx.bus.write8(ctx.cpu.temp16, lsb);
            
            ctx.cpu.temp16 = ctx.cpu.temp16.wrapping_add(1);

            ctx.cpu.step += 1;
        },
        4 => {
            let msb = (ctx.cpu.pc >> 8) as u8;
            ctx.bus.write8(ctx.cpu.temp16, msb);

            ctx.cpu.step += 1;
        },
        5 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}


/* LD_hl_sp_e8 */

pub fn op_ld_hl_sp_e8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => { op_read_n8(ctx); },
        2 => { 
            alu::alu_add_sp_e8(ctx.cpu, ctx.cpu.temp);
            ctx.cpu.step += 1;
        },
        3 => {
            // technically msb part of sp_e8 would be performed here
            ctx.cpu.set_reg16(Reg16::HL, ctx.cpu.sp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}


/* LD SP, HL */

pub fn op_ld_sp_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.sp = ctx.cpu.get_reg16(Reg16::HL);
            ctx.cpu.step += 1;
        },
        2 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}


/* POP_r16 */

pub fn op_pop_r16(ctx: &mut CpuExec, reg: Reg16) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
            ctx.cpu.temp16 = ctx.cpu.temp as u16;

            ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

            ctx.cpu.step += 1;
        },
        2 => {
            ctx.cpu.temp = ctx.bus.read8(ctx.cpu.sp);
            ctx.cpu.temp16 = ctx.cpu.temp16 | ((ctx.cpu.temp as u16) << 8);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

            ctx.cpu.step += 1;
        },
        3 => {
            ctx.cpu.set_reg16(reg, ctx.cpu.temp16);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_pop_bc(ctx: &mut CpuExec) { op_pop_r16(ctx, Reg16::BC) }
pub fn op_pop_de(ctx: &mut CpuExec) { op_pop_r16(ctx, Reg16::DE) }
pub fn op_pop_hl(ctx: &mut CpuExec) { op_pop_r16(ctx, Reg16::HL) }
pub fn op_pop_af(ctx: &mut CpuExec) { op_pop_r16(ctx, Reg16::AF) }

/* PUSH */

pub fn op_push_r16(ctx: &mut CpuExec, reg: Reg16) {
    match ctx.cpu.step {
        1 => {
            ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);
            ctx.cpu.step += 1;
        },
        2 => {
            let msb = (ctx.cpu.get_reg16(reg) >> 8) as u8;
            ctx.bus.write8(ctx.cpu.sp, msb);

            ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);

            ctx.cpu.step += 1;
        },
        3 => {
            let lsb = (0x00FF & ctx.cpu.get_reg16(reg)) as u8;
            ctx.bus.write8(ctx.cpu.sp, lsb);

            ctx.cpu.step += 1;
        },
        4 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

pub fn op_push_bc(ctx: &mut CpuExec) { op_push_r16(ctx, Reg16::BC) }
pub fn op_push_de(ctx: &mut CpuExec) { op_push_r16(ctx, Reg16::DE) }
pub fn op_push_hl(ctx: &mut CpuExec) { op_push_r16(ctx, Reg16::HL) }
pub fn op_push_af(ctx: &mut CpuExec) { op_push_r16(ctx, Reg16::AF) }

