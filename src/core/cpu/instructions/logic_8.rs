use super::{op_read_hl, op_read_n8};

use super::super::alu;
use super::super::registers::{Reg};
use super::super::CPU;


/* AND */

pub fn op_and_a_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    alu::alu_and(cpu, val);

    op_fetch_next(cpu);
}

pub fn op_and_a_b(cpu: &mut CPU) { op_and_a_r8(cpu, Reg::B); }
pub fn op_and_a_c(cpu: &mut CPU) { op_and_a_r8(cpu, Reg::C); }
pub fn op_and_a_d(cpu: &mut CPU) { op_and_a_r8(cpu, Reg::D); }
pub fn op_and_a_e(cpu: &mut CPU) { op_and_a_r8(cpu, Reg::E); }
pub fn op_and_a_h(cpu: &mut CPU) { op_and_a_r8(cpu, Reg::H); }
pub fn op_and_a_l(cpu: &mut CPU) { op_and_a_r8(cpu, Reg::L); }
pub fn op_and_a_a(cpu: &mut CPU) { op_and_a_r8(cpu, Reg::A); }

pub fn op_and_a_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            alu::alu_and(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_and_a_n8(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_n8(cpu),
        2 => {
            alu::alu_and(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* CPL */

pub fn op_cpl(cpu: &mut CPU) {
    alu::alu_cpl(cpu);
}

/* OR */

pub fn op_or_a_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    alu::alu_or(cpu, val);

    op_fetch_next(cpu);
}

pub fn op_or_a_b(cpu: &mut CPU) { op_or_a_r8(cpu, Reg::B); }
pub fn op_or_a_c(cpu: &mut CPU) { op_or_a_r8(cpu, Reg::C); }
pub fn op_or_a_d(cpu: &mut CPU) { op_or_a_r8(cpu, Reg::D); }
pub fn op_or_a_e(cpu: &mut CPU) { op_or_a_r8(cpu, Reg::E); }
pub fn op_or_a_h(cpu: &mut CPU) { op_or_a_r8(cpu, Reg::H); }
pub fn op_or_a_l(cpu: &mut CPU) { op_or_a_r8(cpu, Reg::L); }
pub fn op_or_a_a(cpu: &mut CPU) { op_or_a_r8(cpu, Reg::A); }

pub fn op_or_a_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            alu::alu_or(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_or_a_n8(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_n8(cpu),
        2 => {
            alu::alu_or(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* XOR */

pub fn op_xor_a_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    alu::alu_xor(cpu, val);

    op_fetch_next(cpu);
}

pub fn op_xor_a_b(cpu: &mut CPU) { op_xor_a_r8(cpu, Reg::B); }
pub fn op_xor_a_c(cpu: &mut CPU) { op_xor_a_r8(cpu, Reg::C); }
pub fn op_xor_a_d(cpu: &mut CPU) { op_xor_a_r8(cpu, Reg::D); }
pub fn op_xor_a_e(cpu: &mut CPU) { op_xor_a_r8(cpu, Reg::E); }
pub fn op_xor_a_h(cpu: &mut CPU) { op_xor_a_r8(cpu, Reg::H); }
pub fn op_xor_a_l(cpu: &mut CPU) { op_xor_a_r8(cpu, Reg::L); }
pub fn op_xor_a_a(cpu: &mut CPU) { op_xor_a_r8(cpu, Reg::A); }

pub fn op_xor_a_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            alu::alu_xor(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_xor_a_n8(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_n8(cpu),
        2 => {
            alu::alu_xor(cpu, cpu.temp);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

/* CCF */

pub fn op_ccf(cpu: &mut CPU) {
    debug_assert!(cpu.step == 1);

    alu::alu_ccf(cpu);

    op_fetch_next(cpu);
}

/* SCF */

pub fn op_scf(cpu: &mut CPU) {
    debug_assert!(cpu.step == 1);

    alu::alu_scf(cpu);

    op_fetch_next(cpu);
}

