use super::{op_read_hl, op_write_hl};

use super::super::alu;
use super::super::registers::{Reg};
use super::super::CPU;

/* RL, RLA */

pub fn op_rl_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    let res = alu::alu_rl(cpu, val);
    cpu.set_reg(reg, res);

    op_fetch_next(cpu);
}

pub fn op_rl_b(cpu: &mut CPU) { op_rl_r8(cpu, Reg::B); }
pub fn op_rl_c(cpu: &mut CPU) { op_rl_r8(cpu, Reg::C); }
pub fn op_rl_d(cpu: &mut CPU) { op_rl_r8(cpu, Reg::D); }
pub fn op_rl_e(cpu: &mut CPU) { op_rl_r8(cpu, Reg::E); }
pub fn op_rl_h(cpu: &mut CPU) { op_rl_r8(cpu, Reg::H); }
pub fn op_rl_l(cpu: &mut CPU) { op_rl_r8(cpu, Reg::L); }
pub fn op_rl_a(cpu: &mut CPU) { op_rl_r8(cpu, Reg::A); }

pub fn op_rla(cpu: &mut CPU) { alu::alu_rla(cpu); }

pub fn op_rl_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            let res = alu::alu_rl(cpu, cpu.temp);
            cpu.temp = res;
            
            op_write_hl(cpu); // cpu.step += 1;
        },
        3 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* RLC, RLCA */

pub fn op_rlc_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    let res = alu::alu_rlc(cpu, val);
    cpu.set_reg(reg, res);

    op_fetch_next(cpu);
}

pub fn op_rlc_b(cpu: &mut CPU) { op_rlc_r8(cpu, Reg::B); }
pub fn op_rlc_c(cpu: &mut CPU) { op_rlc_r8(cpu, Reg::C); }
pub fn op_rlc_d(cpu: &mut CPU) { op_rlc_r8(cpu, Reg::D); }
pub fn op_rlc_e(cpu: &mut CPU) { op_rlc_r8(cpu, Reg::E); }
pub fn op_rlc_h(cpu: &mut CPU) { op_rlc_r8(cpu, Reg::H); }
pub fn op_rlc_l(cpu: &mut CPU) { op_rlc_r8(cpu, Reg::L); }
pub fn op_rlc_a(cpu: &mut CPU) { op_rlc_r8(cpu, Reg::A); }

pub fn op_rlca(cpu: &mut CPU) { alu::alu_rlca(cpu); }

pub fn op_rlc_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            let res = alu::alu_rlc(cpu, cpu.temp);
            cpu.temp = res;
            op_write_hl(cpu); // cpu.step += 1;
        },
        3 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* RR, RRA */

pub fn op_rr_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    let res = alu::alu_rr(cpu, val);
    cpu.set_reg(reg, res);

    op_fetch_next(cpu);
}

pub fn op_rr_b(cpu: &mut CPU) { op_rr_r8(cpu, Reg::B); }
pub fn op_rr_c(cpu: &mut CPU) { op_rr_r8(cpu, Reg::C); }
pub fn op_rr_d(cpu: &mut CPU) { op_rr_r8(cpu, Reg::D); }
pub fn op_rr_e(cpu: &mut CPU) { op_rr_r8(cpu, Reg::E); }
pub fn op_rr_h(cpu: &mut CPU) { op_rr_r8(cpu, Reg::H); }
pub fn op_rr_l(cpu: &mut CPU) { op_rr_r8(cpu, Reg::L); }
pub fn op_rr_a(cpu: &mut CPU) { op_rr_r8(cpu, Reg::A); }

pub fn op_rra(cpu: &mut CPU) { alu::alu_rra(cpu); }

pub fn op_rr_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            let res = alu::alu_rr(cpu, cpu.temp);
            cpu.temp = res;
            op_write_hl(cpu); // cpu.step += 1;
        },
        3 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* RRC, RRCA */

pub fn op_rrc_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    let res = alu::alu_rrc(cpu, val);
    cpu.set_reg(reg, res);

    op_fetch_next(cpu);
}

pub fn op_rrc_b(cpu: &mut CPU) { op_rrc_r8(cpu, Reg::B); }
pub fn op_rrc_c(cpu: &mut CPU) { op_rrc_r8(cpu, Reg::C); }
pub fn op_rrc_d(cpu: &mut CPU) { op_rrc_r8(cpu, Reg::D); }
pub fn op_rrc_e(cpu: &mut CPU) { op_rrc_r8(cpu, Reg::E); }
pub fn op_rrc_h(cpu: &mut CPU) { op_rrc_r8(cpu, Reg::H); }
pub fn op_rrc_l(cpu: &mut CPU) { op_rrc_r8(cpu, Reg::L); }
pub fn op_rrc_a(cpu: &mut CPU) { op_rrc_r8(cpu, Reg::A); }

pub fn op_rrca(cpu: &mut CPU) { alu::alu_rrca(cpu); }

pub fn op_rrc_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            let res = alu::alu_rrc(cpu, cpu.temp);
            cpu.temp = res;
            op_write_hl(cpu); // cpu.step += 1;
        },
        3 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* SLA */

pub fn op_sla_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    let res = alu::alu_sla(cpu, val);
    cpu.set_reg(reg, res);

    op_fetch_next(cpu);
}

pub fn op_sla_b(cpu: &mut CPU) { op_sla_r8(cpu, Reg::B); }
pub fn op_sla_c(cpu: &mut CPU) { op_sla_r8(cpu, Reg::C); }
pub fn op_sla_d(cpu: &mut CPU) { op_sla_r8(cpu, Reg::D); }
pub fn op_sla_e(cpu: &mut CPU) { op_sla_r8(cpu, Reg::E); }
pub fn op_sla_h(cpu: &mut CPU) { op_sla_r8(cpu, Reg::H); }
pub fn op_sla_l(cpu: &mut CPU) { op_sla_r8(cpu, Reg::L); }
pub fn op_sla_a(cpu: &mut CPU) { op_sla_r8(cpu, Reg::A); }

pub fn op_sla_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            let res = alu::alu_sla(cpu, cpu.temp);
            cpu.temp = res;

            op_write_hl(cpu); // cpu.step += 1;
        },
        3 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* SRA */

pub fn op_sra_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    let res = alu::alu_sra(cpu, val);
    cpu.set_reg(reg, res);

    op_fetch_next(cpu);
}

pub fn op_sra_b(cpu: &mut CPU) { op_sra_r8(cpu, Reg::B); }
pub fn op_sra_c(cpu: &mut CPU) { op_sra_r8(cpu, Reg::C); }
pub fn op_sra_d(cpu: &mut CPU) { op_sra_r8(cpu, Reg::D); }
pub fn op_sra_e(cpu: &mut CPU) { op_sra_r8(cpu, Reg::E); }
pub fn op_sra_h(cpu: &mut CPU) { op_sra_r8(cpu, Reg::H); }
pub fn op_sra_l(cpu: &mut CPU) { op_sra_r8(cpu, Reg::L); }
pub fn op_sra_a(cpu: &mut CPU) { op_sra_r8(cpu, Reg::A); }

pub fn op_sra_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            let res = alu::alu_sra(cpu, cpu.temp);
            cpu.temp = res;
            cpu.step += 1;
        },
        3 => op_write_hl(cpu),
        _ => unreachable!(),
    }
}

/* SRL */

pub fn op_srl_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    let res = alu::alu_srl(cpu, val);
    cpu.set_reg(reg, res);

    op_fetch_next(cpu);
}

pub fn op_srl_b(cpu: &mut CPU) { op_srl_r8(cpu, Reg::B); }
pub fn op_srl_c(cpu: &mut CPU) { op_srl_r8(cpu, Reg::C); }
pub fn op_srl_d(cpu: &mut CPU) { op_srl_r8(cpu, Reg::D); }
pub fn op_srl_e(cpu: &mut CPU) { op_srl_r8(cpu, Reg::E); }
pub fn op_srl_h(cpu: &mut CPU) { op_srl_r8(cpu, Reg::H); }
pub fn op_srl_l(cpu: &mut CPU) { op_srl_r8(cpu, Reg::L); }
pub fn op_srl_a(cpu: &mut CPU) { op_srl_r8(cpu, Reg::A); }

pub fn op_srl_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            let res = alu::alu_srl(cpu, cpu.temp);
            cpu.temp = res;
            op_write_hl(cpu); // cpu.step += 1;
        },
        3 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

/* SWAP */

pub fn op_swap_r8(cpu: &mut CPU, reg: Reg) {
    debug_assert!(cpu.step == 1);

    let val = cpu.get_reg(reg);
    let res = alu::alu_swap(cpu, val);
    cpu.set_reg(reg, res);

    op_fetch_next(cpu);
}

pub fn op_swap_b(cpu: &mut CPU) { op_swap_r8(cpu, Reg::B); }
pub fn op_swap_c(cpu: &mut CPU) { op_swap_r8(cpu, Reg::C); }
pub fn op_swap_d(cpu: &mut CPU) { op_swap_r8(cpu, Reg::D); }
pub fn op_swap_e(cpu: &mut CPU) { op_swap_r8(cpu, Reg::E); }
pub fn op_swap_h(cpu: &mut CPU) { op_swap_r8(cpu, Reg::H); }
pub fn op_swap_l(cpu: &mut CPU) { op_swap_r8(cpu, Reg::L); }
pub fn op_swap_a(cpu: &mut CPU) { op_swap_r8(cpu, Reg::A); }

pub fn op_swap_hl(cpu: &mut CPU) {
    match cpu.step {
        1 => op_read_hl(cpu),
        2 => {
            let res = alu::alu_swap(cpu, cpu.temp);
            cpu.temp = res;
            
            op_write_hl(cpu); // cpu.step += 1;
        },
        3 => { op_fetch_next(cpu); },
        _ => unreachable!(),
    }
}

