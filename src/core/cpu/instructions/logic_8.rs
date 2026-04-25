use crate::core::cpu::instructions::op_fetch_next;

use super::{op_read_hl, op_read_n8};

use super::super::alu;
use super::super::registers::{Reg};
use super::super::CpuExec;


/* AND */

pub fn op_and_a_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    alu::alu_and(ctx.cpu, val);

    op_fetch_next(ctx);
}

pub fn op_and_a_b(ctx: &mut CpuExec) { op_and_a_r8(ctx, Reg::B); }
pub fn op_and_a_c(ctx: &mut CpuExec) { op_and_a_r8(ctx, Reg::C); }
pub fn op_and_a_d(ctx: &mut CpuExec) { op_and_a_r8(ctx, Reg::D); }
pub fn op_and_a_e(ctx: &mut CpuExec) { op_and_a_r8(ctx, Reg::E); }
pub fn op_and_a_h(ctx: &mut CpuExec) { op_and_a_r8(ctx, Reg::H); }
pub fn op_and_a_l(ctx: &mut CpuExec) { op_and_a_r8(ctx, Reg::L); }
pub fn op_and_a_a(ctx: &mut CpuExec) { op_and_a_r8(ctx, Reg::A); }

pub fn op_and_a_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            alu::alu_and(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_and_a_n8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_n8(ctx),
        2 => {
            alu::alu_and(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* CPL */

pub fn op_cpl(ctx: &mut CpuExec) {
    alu::alu_cpl(ctx.cpu);
}

/* OR */

pub fn op_or_a_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    alu::alu_or(ctx.cpu, val);

    op_fetch_next(ctx);
}

pub fn op_or_a_b(ctx: &mut CpuExec) { op_or_a_r8(ctx, Reg::B); }
pub fn op_or_a_c(ctx: &mut CpuExec) { op_or_a_r8(ctx, Reg::C); }
pub fn op_or_a_d(ctx: &mut CpuExec) { op_or_a_r8(ctx, Reg::D); }
pub fn op_or_a_e(ctx: &mut CpuExec) { op_or_a_r8(ctx, Reg::E); }
pub fn op_or_a_h(ctx: &mut CpuExec) { op_or_a_r8(ctx, Reg::H); }
pub fn op_or_a_l(ctx: &mut CpuExec) { op_or_a_r8(ctx, Reg::L); }
pub fn op_or_a_a(ctx: &mut CpuExec) { op_or_a_r8(ctx, Reg::A); }

pub fn op_or_a_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            alu::alu_or(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_or_a_n8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_n8(ctx),
        2 => {
            alu::alu_or(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* XOR */

pub fn op_xor_a_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    alu::alu_xor(ctx.cpu, val);

    op_fetch_next(ctx);
}

pub fn op_xor_a_b(ctx: &mut CpuExec) { op_xor_a_r8(ctx, Reg::B); }
pub fn op_xor_a_c(ctx: &mut CpuExec) { op_xor_a_r8(ctx, Reg::C); }
pub fn op_xor_a_d(ctx: &mut CpuExec) { op_xor_a_r8(ctx, Reg::D); }
pub fn op_xor_a_e(ctx: &mut CpuExec) { op_xor_a_r8(ctx, Reg::E); }
pub fn op_xor_a_h(ctx: &mut CpuExec) { op_xor_a_r8(ctx, Reg::H); }
pub fn op_xor_a_l(ctx: &mut CpuExec) { op_xor_a_r8(ctx, Reg::L); }
pub fn op_xor_a_a(ctx: &mut CpuExec) { op_xor_a_r8(ctx, Reg::A); }

pub fn op_xor_a_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            alu::alu_xor(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_xor_a_n8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_n8(ctx),
        2 => {
            alu::alu_xor(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* CCF */

pub fn op_ccf(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);

    alu::alu_ccf(ctx.cpu);

    op_fetch_next(ctx);
}

/* SCF */

pub fn op_scf(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);

    alu::alu_scf(ctx.cpu);

    op_fetch_next(ctx);
}

