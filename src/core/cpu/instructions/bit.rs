use super::{op_read_hl, op_write_hl};

use super::super::alu;
use super::super::registers::{Reg};
use super::super::CPU;


// these take n-1 cycles b/c cb prefix read occurs at decode stage

/* BIT */

pub fn op_bit_u3_r8(cpu: &mut CPU, bit: u8, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    alu::alu_bit(cpu, bit, val);

    op_fetch_next(cpu);
}

pub fn op_bit_u3_hl(cpu: &mut CPU, bit: u8) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            alu::alu_bit(cpu, bit, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_bit_0_b(cpu: &mut CPU) { op_bit_u3_r8(cpu, 0, Reg::B); }
pub fn op_bit_0_c(cpu: &mut CPU) { op_bit_u3_r8(cpu, 0, Reg::C); }
pub fn op_bit_0_d(cpu: &mut CPU) { op_bit_u3_r8(cpu, 0, Reg::D); }
pub fn op_bit_0_e(cpu: &mut CPU) { op_bit_u3_r8(cpu, 0, Reg::E); }
pub fn op_bit_0_h(cpu: &mut CPU) { op_bit_u3_r8(cpu, 0, Reg::H); }
pub fn op_bit_0_l(cpu: &mut CPU) { op_bit_u3_r8(cpu, 0, Reg::L); }
pub fn op_bit_0_hl(cpu: &mut CPU) { op_bit_u3_hl(cpu, 0); }
pub fn op_bit_0_a(cpu: &mut CPU) { op_bit_u3_r8(cpu, 0, Reg::A); }

pub fn op_bit_1_b(cpu: &mut CPU) { op_bit_u3_r8(cpu, 1, Reg::B); }
pub fn op_bit_1_c(cpu: &mut CPU) { op_bit_u3_r8(cpu, 1, Reg::C); }
pub fn op_bit_1_d(cpu: &mut CPU) { op_bit_u3_r8(cpu, 1, Reg::D); }
pub fn op_bit_1_e(cpu: &mut CPU) { op_bit_u3_r8(cpu, 1, Reg::E); }
pub fn op_bit_1_h(cpu: &mut CPU) { op_bit_u3_r8(cpu, 1, Reg::H); }
pub fn op_bit_1_l(cpu: &mut CPU) { op_bit_u3_r8(cpu, 1, Reg::L); }
pub fn op_bit_1_hl(cpu: &mut CPU) { op_bit_u3_hl(cpu, 1); }
pub fn op_bit_1_a(cpu: &mut CPU) { op_bit_u3_r8(cpu, 1, Reg::A); }

pub fn op_bit_2_b(cpu: &mut CPU) { op_bit_u3_r8(cpu, 2, Reg::B); }
pub fn op_bit_2_c(cpu: &mut CPU) { op_bit_u3_r8(cpu, 2, Reg::C); }
pub fn op_bit_2_d(cpu: &mut CPU) { op_bit_u3_r8(cpu, 2, Reg::D); }
pub fn op_bit_2_e(cpu: &mut CPU) { op_bit_u3_r8(cpu, 2, Reg::E); }
pub fn op_bit_2_h(cpu: &mut CPU) { op_bit_u3_r8(cpu, 2, Reg::H); }
pub fn op_bit_2_l(cpu: &mut CPU) { op_bit_u3_r8(cpu, 2, Reg::L); }
pub fn op_bit_2_hl(cpu: &mut CPU) { op_bit_u3_hl(cpu, 2); }
pub fn op_bit_2_a(cpu: &mut CPU) { op_bit_u3_r8(cpu, 2, Reg::A); }

pub fn op_bit_3_b(cpu: &mut CPU) { op_bit_u3_r8(cpu, 3, Reg::B); }
pub fn op_bit_3_c(cpu: &mut CPU) { op_bit_u3_r8(cpu, 3, Reg::C); }
pub fn op_bit_3_d(cpu: &mut CPU) { op_bit_u3_r8(cpu, 3, Reg::D); }
pub fn op_bit_3_e(cpu: &mut CPU) { op_bit_u3_r8(cpu, 3, Reg::E); }
pub fn op_bit_3_h(cpu: &mut CPU) { op_bit_u3_r8(cpu, 3, Reg::H); }
pub fn op_bit_3_l(cpu: &mut CPU) { op_bit_u3_r8(cpu, 3, Reg::L); }
pub fn op_bit_3_hl(cpu: &mut CPU) { op_bit_u3_hl(cpu, 3); }
pub fn op_bit_3_a(cpu: &mut CPU) { op_bit_u3_r8(cpu, 3, Reg::A); }

pub fn op_bit_4_b(cpu: &mut CPU) { op_bit_u3_r8(cpu, 4, Reg::B); }
pub fn op_bit_4_c(cpu: &mut CPU) { op_bit_u3_r8(cpu, 4, Reg::C); }
pub fn op_bit_4_d(cpu: &mut CPU) { op_bit_u3_r8(cpu, 4, Reg::D); }
pub fn op_bit_4_e(cpu: &mut CPU) { op_bit_u3_r8(cpu, 4, Reg::E); }
pub fn op_bit_4_h(cpu: &mut CPU) { op_bit_u3_r8(cpu, 4, Reg::H); }
pub fn op_bit_4_l(cpu: &mut CPU) { op_bit_u3_r8(cpu, 4, Reg::L); }
pub fn op_bit_4_hl(cpu: &mut CPU) { op_bit_u3_hl(cpu, 4); }
pub fn op_bit_4_a(cpu: &mut CPU) { op_bit_u3_r8(cpu, 4, Reg::A); }

pub fn op_bit_5_b(cpu: &mut CPU) { op_bit_u3_r8(cpu, 5, Reg::B); }
pub fn op_bit_5_c(cpu: &mut CPU) { op_bit_u3_r8(cpu, 5, Reg::C); }
pub fn op_bit_5_d(cpu: &mut CPU) { op_bit_u3_r8(cpu, 5, Reg::D); }
pub fn op_bit_5_e(cpu: &mut CPU) { op_bit_u3_r8(cpu, 5, Reg::E); }
pub fn op_bit_5_h(cpu: &mut CPU) { op_bit_u3_r8(cpu, 5, Reg::H); }
pub fn op_bit_5_l(cpu: &mut CPU) { op_bit_u3_r8(cpu, 5, Reg::L); }
pub fn op_bit_5_hl(cpu: &mut CPU) { op_bit_u3_hl(cpu, 5); }
pub fn op_bit_5_a(cpu: &mut CPU) { op_bit_u3_r8(cpu, 5, Reg::A); }

pub fn op_bit_6_b(cpu: &mut CPU) { op_bit_u3_r8(cpu, 6, Reg::B); }
pub fn op_bit_6_c(cpu: &mut CPU) { op_bit_u3_r8(cpu, 6, Reg::C); }
pub fn op_bit_6_d(cpu: &mut CPU) { op_bit_u3_r8(cpu, 6, Reg::D); }
pub fn op_bit_6_e(cpu: &mut CPU) { op_bit_u3_r8(cpu, 6, Reg::E); }
pub fn op_bit_6_h(cpu: &mut CPU) { op_bit_u3_r8(cpu, 6, Reg::H); }
pub fn op_bit_6_l(cpu: &mut CPU) { op_bit_u3_r8(cpu, 6, Reg::L); }
pub fn op_bit_6_hl(cpu: &mut CPU) { op_bit_u3_hl(cpu, 6); }
pub fn op_bit_6_a(cpu: &mut CPU) { op_bit_u3_r8(cpu, 6, Reg::A); }

pub fn op_bit_7_b(cpu: &mut CPU) { op_bit_u3_r8(cpu, 7, Reg::B); }
pub fn op_bit_7_c(cpu: &mut CPU) { op_bit_u3_r8(cpu, 7, Reg::C); }
pub fn op_bit_7_d(cpu: &mut CPU) { op_bit_u3_r8(cpu, 7, Reg::D); }
pub fn op_bit_7_e(cpu: &mut CPU) { op_bit_u3_r8(cpu, 7, Reg::E); }
pub fn op_bit_7_h(cpu: &mut CPU) { op_bit_u3_r8(cpu, 7, Reg::H); }
pub fn op_bit_7_l(cpu: &mut CPU) { op_bit_u3_r8(cpu, 7, Reg::L); }
pub fn op_bit_7_hl(cpu: &mut CPU) { op_bit_u3_hl(cpu, 7); }
pub fn op_bit_7_a(cpu: &mut CPU) { op_bit_u3_r8(cpu, 7, Reg::A); }

/* RES */

pub fn op_res_u3_r8(cpu: &mut CPU, bit: u8, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    let res = alu::alu_res(cpu, bit, val);
    cpu.set_reg(reg, res);

    op_fetch_next(cpu);
}

pub fn op_res_u3_hl(cpu: &mut CPU, bit: u8) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            let res = alu::alu_res(cpu, bit, cpu.temp);
            cpu.temp = res;
            
            op_write_hl(cpu); // cpu.step += 1;
        },
        3 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

pub fn op_res_0_b(cpu: &mut CPU) { op_res_u3_r8(cpu, 0, Reg::B); }
pub fn op_res_0_c(cpu: &mut CPU) { op_res_u3_r8(cpu, 0, Reg::C); }
pub fn op_res_0_d(cpu: &mut CPU) { op_res_u3_r8(cpu, 0, Reg::D); }
pub fn op_res_0_e(cpu: &mut CPU) { op_res_u3_r8(cpu, 0, Reg::E); }
pub fn op_res_0_h(cpu: &mut CPU) { op_res_u3_r8(cpu, 0, Reg::H); }
pub fn op_res_0_l(cpu: &mut CPU) { op_res_u3_r8(cpu, 0, Reg::L); }
pub fn op_res_0_hl(cpu: &mut CPU) { op_res_u3_hl(cpu, 0); }
pub fn op_res_0_a(cpu: &mut CPU) { op_res_u3_r8(cpu, 0, Reg::A); }

pub fn op_res_1_b(cpu: &mut CPU) { op_res_u3_r8(cpu, 1, Reg::B); }
pub fn op_res_1_c(cpu: &mut CPU) { op_res_u3_r8(cpu, 1, Reg::C); }
pub fn op_res_1_d(cpu: &mut CPU) { op_res_u3_r8(cpu, 1, Reg::D); }
pub fn op_res_1_e(cpu: &mut CPU) { op_res_u3_r8(cpu, 1, Reg::E); }
pub fn op_res_1_h(cpu: &mut CPU) { op_res_u3_r8(cpu, 1, Reg::H); }
pub fn op_res_1_l(cpu: &mut CPU) { op_res_u3_r8(cpu, 1, Reg::L); }
pub fn op_res_1_hl(cpu: &mut CPU) { op_res_u3_hl(cpu, 1); }
pub fn op_res_1_a(cpu: &mut CPU) { op_res_u3_r8(cpu, 1, Reg::A); }

pub fn op_res_2_b(cpu: &mut CPU) { op_res_u3_r8(cpu, 2, Reg::B); }
pub fn op_res_2_c(cpu: &mut CPU) { op_res_u3_r8(cpu, 2, Reg::C); }
pub fn op_res_2_d(cpu: &mut CPU) { op_res_u3_r8(cpu, 2, Reg::D); }
pub fn op_res_2_e(cpu: &mut CPU) { op_res_u3_r8(cpu, 2, Reg::E); }
pub fn op_res_2_h(cpu: &mut CPU) { op_res_u3_r8(cpu, 2, Reg::H); }
pub fn op_res_2_l(cpu: &mut CPU) { op_res_u3_r8(cpu, 2, Reg::L); }
pub fn op_res_2_hl(cpu: &mut CPU) { op_res_u3_hl(cpu, 2); }
pub fn op_res_2_a(cpu: &mut CPU) { op_res_u3_r8(cpu, 2, Reg::A); }

pub fn op_res_3_b(cpu: &mut CPU) { op_res_u3_r8(cpu, 3, Reg::B); }
pub fn op_res_3_c(cpu: &mut CPU) { op_res_u3_r8(cpu, 3, Reg::C); }
pub fn op_res_3_d(cpu: &mut CPU) { op_res_u3_r8(cpu, 3, Reg::D); }
pub fn op_res_3_e(cpu: &mut CPU) { op_res_u3_r8(cpu, 3, Reg::E); }
pub fn op_res_3_h(cpu: &mut CPU) { op_res_u3_r8(cpu, 3, Reg::H); }
pub fn op_res_3_l(cpu: &mut CPU) { op_res_u3_r8(cpu, 3, Reg::L); }
pub fn op_res_3_hl(cpu: &mut CPU) { op_res_u3_hl(cpu, 3); }
pub fn op_res_3_a(cpu: &mut CPU) { op_res_u3_r8(cpu, 3, Reg::A); }

pub fn op_res_4_b(cpu: &mut CPU) { op_res_u3_r8(cpu, 4, Reg::B); }
pub fn op_res_4_c(cpu: &mut CPU) { op_res_u3_r8(cpu, 4, Reg::C); }
pub fn op_res_4_d(cpu: &mut CPU) { op_res_u3_r8(cpu, 4, Reg::D); }
pub fn op_res_4_e(cpu: &mut CPU) { op_res_u3_r8(cpu, 4, Reg::E); }
pub fn op_res_4_h(cpu: &mut CPU) { op_res_u3_r8(cpu, 4, Reg::H); }
pub fn op_res_4_l(cpu: &mut CPU) { op_res_u3_r8(cpu, 4, Reg::L); }
pub fn op_res_4_hl(cpu: &mut CPU) { op_res_u3_hl(cpu, 4); }
pub fn op_res_4_a(cpu: &mut CPU) { op_res_u3_r8(cpu, 4, Reg::A); }

pub fn op_res_5_b(cpu: &mut CPU) { op_res_u3_r8(cpu, 5, Reg::B); }
pub fn op_res_5_c(cpu: &mut CPU) { op_res_u3_r8(cpu, 5, Reg::C); }
pub fn op_res_5_d(cpu: &mut CPU) { op_res_u3_r8(cpu, 5, Reg::D); }
pub fn op_res_5_e(cpu: &mut CPU) { op_res_u3_r8(cpu, 5, Reg::E); }
pub fn op_res_5_h(cpu: &mut CPU) { op_res_u3_r8(cpu, 5, Reg::H); }
pub fn op_res_5_l(cpu: &mut CPU) { op_res_u3_r8(cpu, 5, Reg::L); }
pub fn op_res_5_hl(cpu: &mut CPU) { op_res_u3_hl(cpu, 5); }
pub fn op_res_5_a(cpu: &mut CPU) { op_res_u3_r8(cpu, 5, Reg::A); }

pub fn op_res_6_b(cpu: &mut CPU) { op_res_u3_r8(cpu, 6, Reg::B); }
pub fn op_res_6_c(cpu: &mut CPU) { op_res_u3_r8(cpu, 6, Reg::C); }
pub fn op_res_6_d(cpu: &mut CPU) { op_res_u3_r8(cpu, 6, Reg::D); }
pub fn op_res_6_e(cpu: &mut CPU) { op_res_u3_r8(cpu, 6, Reg::E); }
pub fn op_res_6_h(cpu: &mut CPU) { op_res_u3_r8(cpu, 6, Reg::H); }
pub fn op_res_6_l(cpu: &mut CPU) { op_res_u3_r8(cpu, 6, Reg::L); }
pub fn op_res_6_hl(cpu: &mut CPU) { op_res_u3_hl(cpu, 6); }
pub fn op_res_6_a(cpu: &mut CPU) { op_res_u3_r8(cpu, 6, Reg::A); }

pub fn op_res_7_b(cpu: &mut CPU) { op_res_u3_r8(cpu, 7, Reg::B); }
pub fn op_res_7_c(cpu: &mut CPU) { op_res_u3_r8(cpu, 7, Reg::C); }
pub fn op_res_7_d(cpu: &mut CPU) { op_res_u3_r8(cpu, 7, Reg::D); }
pub fn op_res_7_e(cpu: &mut CPU) { op_res_u3_r8(cpu, 7, Reg::E); }
pub fn op_res_7_h(cpu: &mut CPU) { op_res_u3_r8(cpu, 7, Reg::H); }
pub fn op_res_7_l(cpu: &mut CPU) { op_res_u3_r8(cpu, 7, Reg::L); }
pub fn op_res_7_hl(cpu: &mut CPU) { op_res_u3_hl(cpu, 7); }
pub fn op_res_7_a(cpu: &mut CPU) { op_res_u3_r8(cpu, 7, Reg::A); }

/* SET */

pub fn op_set_u3_r8(cpu: &mut CPU, bit: u8, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    let res = alu::alu_set(cpu, bit, val);
    cpu.set_reg(reg, res);

    op_fetch_next(cpu);
}

pub fn op_set_u3_hl(cpu: &mut CPU, bit: u8) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            let res = alu::alu_set(cpu, bit, cpu.temp);
            cpu.temp = res;

            op_write_hl(cpu); // cpu.step += 1;
        },
        3 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

pub fn op_set_0_b(cpu: &mut CPU) { op_set_u3_r8(cpu, 0, Reg::B); }
pub fn op_set_0_c(cpu: &mut CPU) { op_set_u3_r8(cpu, 0, Reg::C); }
pub fn op_set_0_d(cpu: &mut CPU) { op_set_u3_r8(cpu, 0, Reg::D); }
pub fn op_set_0_e(cpu: &mut CPU) { op_set_u3_r8(cpu, 0, Reg::E); }
pub fn op_set_0_h(cpu: &mut CPU) { op_set_u3_r8(cpu, 0, Reg::H); }
pub fn op_set_0_l(cpu: &mut CPU) { op_set_u3_r8(cpu, 0, Reg::L); }
pub fn op_set_0_hl(cpu: &mut CPU) { op_set_u3_hl(cpu, 0); }
pub fn op_set_0_a(cpu: &mut CPU) { op_set_u3_r8(cpu, 0, Reg::A); }

pub fn op_set_1_b(cpu: &mut CPU) { op_set_u3_r8(cpu, 1, Reg::B); }
pub fn op_set_1_c(cpu: &mut CPU) { op_set_u3_r8(cpu, 1, Reg::C); }
pub fn op_set_1_d(cpu: &mut CPU) { op_set_u3_r8(cpu, 1, Reg::D); }
pub fn op_set_1_e(cpu: &mut CPU) { op_set_u3_r8(cpu, 1, Reg::E); }
pub fn op_set_1_h(cpu: &mut CPU) { op_set_u3_r8(cpu, 1, Reg::H); }
pub fn op_set_1_l(cpu: &mut CPU) { op_set_u3_r8(cpu, 1, Reg::L); }
pub fn op_set_1_hl(cpu: &mut CPU) { op_set_u3_hl(cpu, 1); }
pub fn op_set_1_a(cpu: &mut CPU) { op_set_u3_r8(cpu, 1, Reg::A); }

pub fn op_set_2_b(cpu: &mut CPU) { op_set_u3_r8(cpu, 2, Reg::B); }
pub fn op_set_2_c(cpu: &mut CPU) { op_set_u3_r8(cpu, 2, Reg::C); }
pub fn op_set_2_d(cpu: &mut CPU) { op_set_u3_r8(cpu, 2, Reg::D); }
pub fn op_set_2_e(cpu: &mut CPU) { op_set_u3_r8(cpu, 2, Reg::E); }
pub fn op_set_2_h(cpu: &mut CPU) { op_set_u3_r8(cpu, 2, Reg::H); }
pub fn op_set_2_l(cpu: &mut CPU) { op_set_u3_r8(cpu, 2, Reg::L); }
pub fn op_set_2_hl(cpu: &mut CPU) { op_set_u3_hl(cpu, 2); }
pub fn op_set_2_a(cpu: &mut CPU) { op_set_u3_r8(cpu, 2, Reg::A); }

pub fn op_set_3_b(cpu: &mut CPU) { op_set_u3_r8(cpu, 3, Reg::B); }
pub fn op_set_3_c(cpu: &mut CPU) { op_set_u3_r8(cpu, 3, Reg::C); }
pub fn op_set_3_d(cpu: &mut CPU) { op_set_u3_r8(cpu, 3, Reg::D); }
pub fn op_set_3_e(cpu: &mut CPU) { op_set_u3_r8(cpu, 3, Reg::E); }
pub fn op_set_3_h(cpu: &mut CPU) { op_set_u3_r8(cpu, 3, Reg::H); }
pub fn op_set_3_l(cpu: &mut CPU) { op_set_u3_r8(cpu, 3, Reg::L); }
pub fn op_set_3_hl(cpu: &mut CPU) { op_set_u3_hl(cpu, 3); }
pub fn op_set_3_a(cpu: &mut CPU) { op_set_u3_r8(cpu, 3, Reg::A); }

pub fn op_set_4_b(cpu: &mut CPU) { op_set_u3_r8(cpu, 4, Reg::B); }
pub fn op_set_4_c(cpu: &mut CPU) { op_set_u3_r8(cpu, 4, Reg::C); }
pub fn op_set_4_d(cpu: &mut CPU) { op_set_u3_r8(cpu, 4, Reg::D); }
pub fn op_set_4_e(cpu: &mut CPU) { op_set_u3_r8(cpu, 4, Reg::E); }
pub fn op_set_4_h(cpu: &mut CPU) { op_set_u3_r8(cpu, 4, Reg::H); }
pub fn op_set_4_l(cpu: &mut CPU) { op_set_u3_r8(cpu, 4, Reg::L); }
pub fn op_set_4_hl(cpu: &mut CPU) { op_set_u3_hl(cpu, 4); }
pub fn op_set_4_a(cpu: &mut CPU) { op_set_u3_r8(cpu, 4, Reg::A); }

pub fn op_set_5_b(cpu: &mut CPU) { op_set_u3_r8(cpu, 5, Reg::B); }
pub fn op_set_5_c(cpu: &mut CPU) { op_set_u3_r8(cpu, 5, Reg::C); }
pub fn op_set_5_d(cpu: &mut CPU) { op_set_u3_r8(cpu, 5, Reg::D); }
pub fn op_set_5_e(cpu: &mut CPU) { op_set_u3_r8(cpu, 5, Reg::E); }
pub fn op_set_5_h(cpu: &mut CPU) { op_set_u3_r8(cpu, 5, Reg::H); }
pub fn op_set_5_l(cpu: &mut CPU) { op_set_u3_r8(cpu, 5, Reg::L); }
pub fn op_set_5_hl(cpu: &mut CPU) { op_set_u3_hl(cpu, 5); }
pub fn op_set_5_a(cpu: &mut CPU) { op_set_u3_r8(cpu, 5, Reg::A); }

pub fn op_set_6_b(cpu: &mut CPU) { op_set_u3_r8(cpu, 6, Reg::B); }
pub fn op_set_6_c(cpu: &mut CPU) { op_set_u3_r8(cpu, 6, Reg::C); }
pub fn op_set_6_d(cpu: &mut CPU) { op_set_u3_r8(cpu, 6, Reg::D); }
pub fn op_set_6_e(cpu: &mut CPU) { op_set_u3_r8(cpu, 6, Reg::E); }
pub fn op_set_6_h(cpu: &mut CPU) { op_set_u3_r8(cpu, 6, Reg::H); }
pub fn op_set_6_l(cpu: &mut CPU) { op_set_u3_r8(cpu, 6, Reg::L); }
pub fn op_set_6_hl(cpu: &mut CPU) { op_set_u3_hl(cpu, 6); }
pub fn op_set_6_a(cpu: &mut CPU) { op_set_u3_r8(cpu, 6, Reg::A); }

pub fn op_set_7_b(cpu: &mut CPU) { op_set_u3_r8(cpu, 7, Reg::B); }
pub fn op_set_7_c(cpu: &mut CPU) { op_set_u3_r8(cpu, 7, Reg::C); }
pub fn op_set_7_d(cpu: &mut CPU) { op_set_u3_r8(cpu, 7, Reg::D); }
pub fn op_set_7_e(cpu: &mut CPU) { op_set_u3_r8(cpu, 7, Reg::E); }
pub fn op_set_7_h(cpu: &mut CPU) { op_set_u3_r8(cpu, 7, Reg::H); }
pub fn op_set_7_l(cpu: &mut CPU) { op_set_u3_r8(cpu, 7, Reg::L); }
pub fn op_set_7_hl(cpu: &mut CPU) { op_set_u3_hl(cpu, 7); }
pub fn op_set_7_a(cpu: &mut CPU) { op_set_u3_r8(cpu, 7, Reg::A); }


