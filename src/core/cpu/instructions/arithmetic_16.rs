use crate::core::cpu::registers::Reg16;

use super::super::alu;
use super::super::CPU;


// 16-bit ops are 2 M-cycles
// this is artificially modelled by using the 
// first M-cycle to load the operand into temp16

/* ADD16 */

pub fn op_add_hl_r16(cpu: &mut CPU, reg: Reg16) {
    match cpu.step {
        1 => {
            cpu.temp16 = cpu.get_reg16(reg);
            cpu.step += 1;
        },
        2 => {
            alu::alu_add16(cpu, cpu.temp16);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_add_hl_bc(cpu: &mut CPU) { op_add_hl_r16(cpu, Reg16::BC) }
pub fn op_add_hl_de(cpu: &mut CPU) { op_add_hl_r16(cpu, Reg16::DE) }
pub fn op_add_hl_hl(cpu: &mut CPU) { op_add_hl_r16(cpu, Reg16::HL) }

/* DEC16 */

pub fn op_dec_r16(cpu: &mut CPU, reg: Reg16) {
    match cpu.step {
        1 => {
            cpu.temp16 = cpu.get_reg16(reg);
            cpu.step += 1;
        },
        2 => {
            let res = alu::alu_dec16(cpu.temp16);
            cpu.set_reg16(reg, res);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_dec_bc(cpu: &mut CPU) { op_dec_r16(cpu, Reg16::BC) }
pub fn op_dec_de(cpu: &mut CPU) { op_dec_r16(cpu, Reg16::DE) }
pub fn op_dec_hl(cpu: &mut CPU) { op_dec_r16(cpu, Reg16::HL) }

/* INC16 */

pub fn op_inc_r16(cpu: &mut CPU, reg: Reg16) {
    match cpu.step {
        1 => {
            cpu.temp16 = cpu.get_reg16(reg);
            cpu.step += 1;
        },
        2 => {
            let res = alu::alu_inc16(cpu.temp16);
            cpu.set_reg16(reg, res);
            op_fetch_next(cpu);
        },
        _ => unreachable!(),
    }
}

pub fn op_inc_bc(cpu: &mut CPU) { op_inc_r16(cpu, Reg16::BC) }
pub fn op_inc_de(cpu: &mut CPU) { op_inc_r16(cpu, Reg16::DE) }
pub fn op_inc_hl(cpu: &mut CPU) { op_inc_r16(cpu, Reg16::HL) }
