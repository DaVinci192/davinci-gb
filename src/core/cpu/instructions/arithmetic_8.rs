use crate::core::cpu::instructions::op_fetch_next;

use super::{op_read_hl, op_read_n8, op_write_hl};

use super::super::alu;
use super::super::registers::{Reg};
use super::super::CpuExec;

/* ADC */

pub fn op_adc_a_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    alu::alu_adc(ctx.cpu, val);

    op_fetch_next(ctx);
}

pub fn op_adc_a_b(ctx: &mut CpuExec) { op_adc_a_r8(ctx, Reg::B); }
pub fn op_adc_a_c(ctx: &mut CpuExec) { op_adc_a_r8(ctx, Reg::C); }
pub fn op_adc_a_d(ctx: &mut CpuExec) { op_adc_a_r8(ctx, Reg::D); }
pub fn op_adc_a_e(ctx: &mut CpuExec) { op_adc_a_r8(ctx, Reg::E); }
pub fn op_adc_a_h(ctx: &mut CpuExec) { op_adc_a_r8(ctx, Reg::H); }
pub fn op_adc_a_l(ctx: &mut CpuExec) { op_adc_a_r8(ctx, Reg::L); }
pub fn op_adc_a_a(ctx: &mut CpuExec) { op_adc_a_r8(ctx, Reg::A); }

pub fn op_adc_a_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            alu::alu_adc(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_adc_a_n8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_n8(ctx),
        2 => {
            alu::alu_adc(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* ADD */

pub fn op_add_a_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    alu::alu_add(ctx.cpu, val);

    op_fetch_next(ctx);
}

pub fn op_add_a_b(ctx: &mut CpuExec) { op_add_a_r8(ctx, Reg::B); }
pub fn op_add_a_c(ctx: &mut CpuExec) { op_add_a_r8(ctx, Reg::C); }
pub fn op_add_a_d(ctx: &mut CpuExec) { op_add_a_r8(ctx, Reg::D); }
pub fn op_add_a_e(ctx: &mut CpuExec) { op_add_a_r8(ctx, Reg::E); }
pub fn op_add_a_h(ctx: &mut CpuExec) { op_add_a_r8(ctx, Reg::H); }
pub fn op_add_a_l(ctx: &mut CpuExec) { op_add_a_r8(ctx, Reg::L); }
pub fn op_add_a_a(ctx: &mut CpuExec) { op_add_a_r8(ctx, Reg::A); }

pub fn op_add_a_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            alu::alu_add(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_add_a_n8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_n8(ctx),
        2 => {
            alu::alu_add(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* CP */

pub fn op_cp_a_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    alu::alu_cp(ctx.cpu, val);

    op_fetch_next(ctx);
}

pub fn op_cp_a_b(ctx: &mut CpuExec) { op_cp_a_r8(ctx, Reg::B); }
pub fn op_cp_a_c(ctx: &mut CpuExec) { op_cp_a_r8(ctx, Reg::C); }
pub fn op_cp_a_d(ctx: &mut CpuExec) { op_cp_a_r8(ctx, Reg::D); }
pub fn op_cp_a_e(ctx: &mut CpuExec) { op_cp_a_r8(ctx, Reg::E); }
pub fn op_cp_a_h(ctx: &mut CpuExec) { op_cp_a_r8(ctx, Reg::H); }
pub fn op_cp_a_l(ctx: &mut CpuExec) { op_cp_a_r8(ctx, Reg::L); }
pub fn op_cp_a_a(ctx: &mut CpuExec) { op_cp_a_r8(ctx, Reg::A); }

pub fn op_cp_a_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            alu::alu_cp(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_cp_a_n8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_n8(ctx),
        2 => {
            alu::alu_cp(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* DEC */

pub fn op_dec_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    let res = alu::alu_dec(ctx.cpu, val);
    ctx.cpu.set_reg(reg, res);

    op_fetch_next(ctx);
}

pub fn op_dec_b(ctx: &mut CpuExec) { op_dec_r8(ctx, Reg::B); }
pub fn op_dec_c(ctx: &mut CpuExec) { op_dec_r8(ctx, Reg::C); }
pub fn op_dec_d(ctx: &mut CpuExec) { op_dec_r8(ctx, Reg::D); }
pub fn op_dec_e(ctx: &mut CpuExec) { op_dec_r8(ctx, Reg::E); }
pub fn op_dec_h(ctx: &mut CpuExec) { op_dec_r8(ctx, Reg::H); }
pub fn op_dec_l(ctx: &mut CpuExec) { op_dec_r8(ctx, Reg::L); }
pub fn op_dec_a(ctx: &mut CpuExec) { op_dec_r8(ctx, Reg::A); }

pub fn op_dec_hl_ptr(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            let res = alu::alu_dec(ctx.cpu, ctx.cpu.temp);
            ctx.cpu.temp = res;

            op_write_hl(ctx); // ctx.cpu.step += 1 within
        },
        3 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* INC */

pub fn op_inc_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    let res = alu::alu_inc(ctx.cpu, val);
    ctx.cpu.set_reg(reg, res);

    op_fetch_next(ctx);
}

pub fn op_inc_b(ctx: &mut CpuExec) { op_inc_r8(ctx, Reg::B); }
pub fn op_inc_c(ctx: &mut CpuExec) { op_inc_r8(ctx, Reg::C); }
pub fn op_inc_d(ctx: &mut CpuExec) { op_inc_r8(ctx, Reg::D); }
pub fn op_inc_e(ctx: &mut CpuExec) { op_inc_r8(ctx, Reg::E); }
pub fn op_inc_h(ctx: &mut CpuExec) { op_inc_r8(ctx, Reg::H); }
pub fn op_inc_l(ctx: &mut CpuExec) { op_inc_r8(ctx, Reg::L); }
pub fn op_inc_a(ctx: &mut CpuExec) { op_inc_r8(ctx, Reg::A); }

pub fn op_inc_hl_ptr(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            let res = alu::alu_inc(ctx.cpu, ctx.cpu.temp);
            ctx.cpu.temp = res;

            op_write_hl(ctx); // ctx.cpu.step += 1
        },
        3 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

/* SBC */

pub fn op_sbc_a_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    alu::alu_sbc(ctx.cpu, val);

    op_fetch_next(ctx);
}

pub fn op_sbc_a_b(ctx: &mut CpuExec) { op_sbc_a_r8(ctx, Reg::B); }
pub fn op_sbc_a_c(ctx: &mut CpuExec) { op_sbc_a_r8(ctx, Reg::C); }
pub fn op_sbc_a_d(ctx: &mut CpuExec) { op_sbc_a_r8(ctx, Reg::D); }
pub fn op_sbc_a_e(ctx: &mut CpuExec) { op_sbc_a_r8(ctx, Reg::E); }
pub fn op_sbc_a_h(ctx: &mut CpuExec) { op_sbc_a_r8(ctx, Reg::H); }
pub fn op_sbc_a_l(ctx: &mut CpuExec) { op_sbc_a_r8(ctx, Reg::L); }
pub fn op_sbc_a_a(ctx: &mut CpuExec) { op_sbc_a_r8(ctx, Reg::A); }

pub fn op_sbc_a_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            alu::alu_sbc(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_sbc_a_n8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_n8(ctx),
        2 => {
            alu::alu_sbc(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* SUB */

pub fn op_sub_a_r8(ctx: &mut CpuExec, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    alu::alu_sub(ctx.cpu, val);

    op_fetch_next(ctx);
}

pub fn op_sub_a_b(ctx: &mut CpuExec) { op_sub_a_r8(ctx, Reg::B); }
pub fn op_sub_a_c(ctx: &mut CpuExec) { op_sub_a_r8(ctx, Reg::C); }
pub fn op_sub_a_d(ctx: &mut CpuExec) { op_sub_a_r8(ctx, Reg::D); }
pub fn op_sub_a_e(ctx: &mut CpuExec) { op_sub_a_r8(ctx, Reg::E); }
pub fn op_sub_a_h(ctx: &mut CpuExec) { op_sub_a_r8(ctx, Reg::H); }
pub fn op_sub_a_l(ctx: &mut CpuExec) { op_sub_a_r8(ctx, Reg::L); }
pub fn op_sub_a_a(ctx: &mut CpuExec) { op_sub_a_r8(ctx, Reg::A); }

pub fn op_sub_a_hl(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            alu::alu_sub(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_sub_a_n8(ctx: &mut CpuExec) {
    match ctx.cpu.step {
        1 => op_read_n8(ctx),
        2 => {
            alu::alu_sub(ctx.cpu, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

/* DAA */

pub fn op_daa(ctx: &mut CpuExec) {
    debug_assert!(ctx.cpu.step == 1);

    if ctx.cpu.f.n {
        let mut adj = 0u8;

        if ctx.cpu.f.h {
            adj = adj.wrapping_add(0x6);
        }

        if ctx.cpu.f.c {
            adj = adj.wrapping_add(0x60);
        }

        ctx.cpu.a = ctx.cpu.a.wrapping_sub(adj);
    } else {
        let mut adj = 0u8;

        if ctx.cpu.f.h || ((ctx.cpu.a & 0xF) > 9) {
            adj = adj.wrapping_add(6);
        }

        if ctx.cpu.f.c || (ctx.cpu.a > 0x99) {
            adj = adj.wrapping_add(0x60);
            ctx.cpu.f.c = true;
        }

        ctx.cpu.a = ctx.cpu.a.wrapping_add(adj);
    }

    ctx.cpu.f.h = false;

    op_fetch_next(ctx);
}