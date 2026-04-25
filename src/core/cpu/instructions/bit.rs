use crate::core::cpu::instructions::op_fetch_next;

use super::{op_read_hl, op_write_hl};

use super::super::alu;
use super::super::registers::{Reg};
use super::super::CpuExec;


// these take n-1 cycles b/c cb prefix read occurs at decode stage

/* BIT */

pub fn op_bit_u3_r8(ctx: &mut CpuExec, bit: u8, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    alu::alu_bit(ctx.cpu, bit, val);

    op_fetch_next(ctx);
}

pub fn op_bit_u3_hl(ctx: &mut CpuExec, bit: u8) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            alu::alu_bit(ctx.cpu, bit, ctx.cpu.temp);
            op_fetch_next(ctx);
        },
        _ => unreachable!(),
    }
}

pub fn op_bit_0_b(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 0, Reg::B); }
pub fn op_bit_0_c(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 0, Reg::C); }
pub fn op_bit_0_d(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 0, Reg::D); }
pub fn op_bit_0_e(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 0, Reg::E); }
pub fn op_bit_0_h(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 0, Reg::H); }
pub fn op_bit_0_l(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 0, Reg::L); }
pub fn op_bit_0_hl(ctx: &mut CpuExec) { op_bit_u3_hl(ctx, 0); }
pub fn op_bit_0_a(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 0, Reg::A); }

pub fn op_bit_1_b(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 1, Reg::B); }
pub fn op_bit_1_c(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 1, Reg::C); }
pub fn op_bit_1_d(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 1, Reg::D); }
pub fn op_bit_1_e(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 1, Reg::E); }
pub fn op_bit_1_h(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 1, Reg::H); }
pub fn op_bit_1_l(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 1, Reg::L); }
pub fn op_bit_1_hl(ctx: &mut CpuExec) { op_bit_u3_hl(ctx, 1); }
pub fn op_bit_1_a(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 1, Reg::A); }

pub fn op_bit_2_b(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 2, Reg::B); }
pub fn op_bit_2_c(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 2, Reg::C); }
pub fn op_bit_2_d(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 2, Reg::D); }
pub fn op_bit_2_e(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 2, Reg::E); }
pub fn op_bit_2_h(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 2, Reg::H); }
pub fn op_bit_2_l(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 2, Reg::L); }
pub fn op_bit_2_hl(ctx: &mut CpuExec) { op_bit_u3_hl(ctx, 2); }
pub fn op_bit_2_a(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 2, Reg::A); }

pub fn op_bit_3_b(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 3, Reg::B); }
pub fn op_bit_3_c(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 3, Reg::C); }
pub fn op_bit_3_d(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 3, Reg::D); }
pub fn op_bit_3_e(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 3, Reg::E); }
pub fn op_bit_3_h(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 3, Reg::H); }
pub fn op_bit_3_l(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 3, Reg::L); }
pub fn op_bit_3_hl(ctx: &mut CpuExec) { op_bit_u3_hl(ctx, 3); }
pub fn op_bit_3_a(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 3, Reg::A); }

pub fn op_bit_4_b(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 4, Reg::B); }
pub fn op_bit_4_c(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 4, Reg::C); }
pub fn op_bit_4_d(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 4, Reg::D); }
pub fn op_bit_4_e(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 4, Reg::E); }
pub fn op_bit_4_h(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 4, Reg::H); }
pub fn op_bit_4_l(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 4, Reg::L); }
pub fn op_bit_4_hl(ctx: &mut CpuExec) { op_bit_u3_hl(ctx, 4); }
pub fn op_bit_4_a(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 4, Reg::A); }

pub fn op_bit_5_b(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 5, Reg::B); }
pub fn op_bit_5_c(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 5, Reg::C); }
pub fn op_bit_5_d(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 5, Reg::D); }
pub fn op_bit_5_e(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 5, Reg::E); }
pub fn op_bit_5_h(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 5, Reg::H); }
pub fn op_bit_5_l(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 5, Reg::L); }
pub fn op_bit_5_hl(ctx: &mut CpuExec) { op_bit_u3_hl(ctx, 5); }
pub fn op_bit_5_a(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 5, Reg::A); }

pub fn op_bit_6_b(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 6, Reg::B); }
pub fn op_bit_6_c(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 6, Reg::C); }
pub fn op_bit_6_d(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 6, Reg::D); }
pub fn op_bit_6_e(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 6, Reg::E); }
pub fn op_bit_6_h(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 6, Reg::H); }
pub fn op_bit_6_l(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 6, Reg::L); }
pub fn op_bit_6_hl(ctx: &mut CpuExec) { op_bit_u3_hl(ctx, 6); }
pub fn op_bit_6_a(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 6, Reg::A); }

pub fn op_bit_7_b(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 7, Reg::B); }
pub fn op_bit_7_c(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 7, Reg::C); }
pub fn op_bit_7_d(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 7, Reg::D); }
pub fn op_bit_7_e(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 7, Reg::E); }
pub fn op_bit_7_h(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 7, Reg::H); }
pub fn op_bit_7_l(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 7, Reg::L); }
pub fn op_bit_7_hl(ctx: &mut CpuExec) { op_bit_u3_hl(ctx, 7); }
pub fn op_bit_7_a(ctx: &mut CpuExec) { op_bit_u3_r8(ctx, 7, Reg::A); }

/* RES */

pub fn op_res_u3_r8(ctx: &mut CpuExec, bit: u8, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    let res = alu::alu_res(ctx.cpu, bit, val);
    ctx.cpu.set_reg(reg, res);

    op_fetch_next(ctx);
}

pub fn op_res_u3_hl(ctx: &mut CpuExec, bit: u8) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            let res = alu::alu_res(ctx.cpu, bit, ctx.cpu.temp);
            ctx.cpu.temp = res;
            
            op_write_hl(ctx); // ctx.cpu.step += 1;
        },
        3 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

pub fn op_res_0_b(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 0, Reg::B); }
pub fn op_res_0_c(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 0, Reg::C); }
pub fn op_res_0_d(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 0, Reg::D); }
pub fn op_res_0_e(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 0, Reg::E); }
pub fn op_res_0_h(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 0, Reg::H); }
pub fn op_res_0_l(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 0, Reg::L); }
pub fn op_res_0_hl(ctx: &mut CpuExec) { op_res_u3_hl(ctx, 0); }
pub fn op_res_0_a(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 0, Reg::A); }

pub fn op_res_1_b(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 1, Reg::B); }
pub fn op_res_1_c(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 1, Reg::C); }
pub fn op_res_1_d(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 1, Reg::D); }
pub fn op_res_1_e(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 1, Reg::E); }
pub fn op_res_1_h(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 1, Reg::H); }
pub fn op_res_1_l(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 1, Reg::L); }
pub fn op_res_1_hl(ctx: &mut CpuExec) { op_res_u3_hl(ctx, 1); }
pub fn op_res_1_a(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 1, Reg::A); }

pub fn op_res_2_b(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 2, Reg::B); }
pub fn op_res_2_c(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 2, Reg::C); }
pub fn op_res_2_d(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 2, Reg::D); }
pub fn op_res_2_e(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 2, Reg::E); }
pub fn op_res_2_h(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 2, Reg::H); }
pub fn op_res_2_l(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 2, Reg::L); }
pub fn op_res_2_hl(ctx: &mut CpuExec) { op_res_u3_hl(ctx, 2); }
pub fn op_res_2_a(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 2, Reg::A); }

pub fn op_res_3_b(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 3, Reg::B); }
pub fn op_res_3_c(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 3, Reg::C); }
pub fn op_res_3_d(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 3, Reg::D); }
pub fn op_res_3_e(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 3, Reg::E); }
pub fn op_res_3_h(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 3, Reg::H); }
pub fn op_res_3_l(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 3, Reg::L); }
pub fn op_res_3_hl(ctx: &mut CpuExec) { op_res_u3_hl(ctx, 3); }
pub fn op_res_3_a(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 3, Reg::A); }

pub fn op_res_4_b(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 4, Reg::B); }
pub fn op_res_4_c(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 4, Reg::C); }
pub fn op_res_4_d(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 4, Reg::D); }
pub fn op_res_4_e(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 4, Reg::E); }
pub fn op_res_4_h(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 4, Reg::H); }
pub fn op_res_4_l(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 4, Reg::L); }
pub fn op_res_4_hl(ctx: &mut CpuExec) { op_res_u3_hl(ctx, 4); }
pub fn op_res_4_a(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 4, Reg::A); }

pub fn op_res_5_b(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 5, Reg::B); }
pub fn op_res_5_c(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 5, Reg::C); }
pub fn op_res_5_d(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 5, Reg::D); }
pub fn op_res_5_e(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 5, Reg::E); }
pub fn op_res_5_h(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 5, Reg::H); }
pub fn op_res_5_l(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 5, Reg::L); }
pub fn op_res_5_hl(ctx: &mut CpuExec) { op_res_u3_hl(ctx, 5); }
pub fn op_res_5_a(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 5, Reg::A); }

pub fn op_res_6_b(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 6, Reg::B); }
pub fn op_res_6_c(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 6, Reg::C); }
pub fn op_res_6_d(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 6, Reg::D); }
pub fn op_res_6_e(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 6, Reg::E); }
pub fn op_res_6_h(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 6, Reg::H); }
pub fn op_res_6_l(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 6, Reg::L); }
pub fn op_res_6_hl(ctx: &mut CpuExec) { op_res_u3_hl(ctx, 6); }
pub fn op_res_6_a(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 6, Reg::A); }

pub fn op_res_7_b(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 7, Reg::B); }
pub fn op_res_7_c(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 7, Reg::C); }
pub fn op_res_7_d(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 7, Reg::D); }
pub fn op_res_7_e(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 7, Reg::E); }
pub fn op_res_7_h(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 7, Reg::H); }
pub fn op_res_7_l(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 7, Reg::L); }
pub fn op_res_7_hl(ctx: &mut CpuExec) { op_res_u3_hl(ctx, 7); }
pub fn op_res_7_a(ctx: &mut CpuExec) { op_res_u3_r8(ctx, 7, Reg::A); }

/* SET */

pub fn op_set_u3_r8(ctx: &mut CpuExec, bit: u8, reg: Reg) {
    debug_assert!(ctx.cpu.step == 1);

    let val = ctx.cpu.get_reg(reg);
    let res = alu::alu_set(ctx.cpu, bit, val);
    ctx.cpu.set_reg(reg, res);

    op_fetch_next(ctx);
}

pub fn op_set_u3_hl(ctx: &mut CpuExec, bit: u8) {
    match ctx.cpu.step {
        1 => op_read_hl(ctx),
        2 => {
            let res = alu::alu_set(ctx.cpu, bit, ctx.cpu.temp);
            ctx.cpu.temp = res;

            op_write_hl(ctx); // ctx.cpu.step += 1;
        },
        3 => { op_fetch_next(ctx); },
        _ => unreachable!(),
    }
}

pub fn op_set_0_b(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 0, Reg::B); }
pub fn op_set_0_c(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 0, Reg::C); }
pub fn op_set_0_d(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 0, Reg::D); }
pub fn op_set_0_e(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 0, Reg::E); }
pub fn op_set_0_h(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 0, Reg::H); }
pub fn op_set_0_l(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 0, Reg::L); }
pub fn op_set_0_hl(ctx: &mut CpuExec) { op_set_u3_hl(ctx, 0); }
pub fn op_set_0_a(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 0, Reg::A); }

pub fn op_set_1_b(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 1, Reg::B); }
pub fn op_set_1_c(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 1, Reg::C); }
pub fn op_set_1_d(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 1, Reg::D); }
pub fn op_set_1_e(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 1, Reg::E); }
pub fn op_set_1_h(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 1, Reg::H); }
pub fn op_set_1_l(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 1, Reg::L); }
pub fn op_set_1_hl(ctx: &mut CpuExec) { op_set_u3_hl(ctx, 1); }
pub fn op_set_1_a(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 1, Reg::A); }

pub fn op_set_2_b(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 2, Reg::B); }
pub fn op_set_2_c(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 2, Reg::C); }
pub fn op_set_2_d(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 2, Reg::D); }
pub fn op_set_2_e(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 2, Reg::E); }
pub fn op_set_2_h(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 2, Reg::H); }
pub fn op_set_2_l(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 2, Reg::L); }
pub fn op_set_2_hl(ctx: &mut CpuExec) { op_set_u3_hl(ctx, 2); }
pub fn op_set_2_a(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 2, Reg::A); }

pub fn op_set_3_b(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 3, Reg::B); }
pub fn op_set_3_c(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 3, Reg::C); }
pub fn op_set_3_d(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 3, Reg::D); }
pub fn op_set_3_e(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 3, Reg::E); }
pub fn op_set_3_h(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 3, Reg::H); }
pub fn op_set_3_l(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 3, Reg::L); }
pub fn op_set_3_hl(ctx: &mut CpuExec) { op_set_u3_hl(ctx, 3); }
pub fn op_set_3_a(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 3, Reg::A); }

pub fn op_set_4_b(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 4, Reg::B); }
pub fn op_set_4_c(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 4, Reg::C); }
pub fn op_set_4_d(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 4, Reg::D); }
pub fn op_set_4_e(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 4, Reg::E); }
pub fn op_set_4_h(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 4, Reg::H); }
pub fn op_set_4_l(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 4, Reg::L); }
pub fn op_set_4_hl(ctx: &mut CpuExec) { op_set_u3_hl(ctx, 4); }
pub fn op_set_4_a(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 4, Reg::A); }

pub fn op_set_5_b(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 5, Reg::B); }
pub fn op_set_5_c(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 5, Reg::C); }
pub fn op_set_5_d(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 5, Reg::D); }
pub fn op_set_5_e(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 5, Reg::E); }
pub fn op_set_5_h(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 5, Reg::H); }
pub fn op_set_5_l(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 5, Reg::L); }
pub fn op_set_5_hl(ctx: &mut CpuExec) { op_set_u3_hl(ctx, 5); }
pub fn op_set_5_a(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 5, Reg::A); }

pub fn op_set_6_b(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 6, Reg::B); }
pub fn op_set_6_c(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 6, Reg::C); }
pub fn op_set_6_d(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 6, Reg::D); }
pub fn op_set_6_e(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 6, Reg::E); }
pub fn op_set_6_h(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 6, Reg::H); }
pub fn op_set_6_l(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 6, Reg::L); }
pub fn op_set_6_hl(ctx: &mut CpuExec) { op_set_u3_hl(ctx, 6); }
pub fn op_set_6_a(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 6, Reg::A); }

pub fn op_set_7_b(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 7, Reg::B); }
pub fn op_set_7_c(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 7, Reg::C); }
pub fn op_set_7_d(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 7, Reg::D); }
pub fn op_set_7_e(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 7, Reg::E); }
pub fn op_set_7_h(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 7, Reg::H); }
pub fn op_set_7_l(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 7, Reg::L); }
pub fn op_set_7_hl(ctx: &mut CpuExec) { op_set_u3_hl(ctx, 7); }
pub fn op_set_7_a(ctx: &mut CpuExec) { op_set_u3_r8(ctx, 7, Reg::A); }


