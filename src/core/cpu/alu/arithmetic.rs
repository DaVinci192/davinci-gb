use super::super::CPU;
use super::super::registers::Reg16;

pub fn alu_adc(cpu: &mut CPU, rhs: u8) {
    let a = cpu.a;
    let c = cpu.f.c as u8;

    let sum = a as u16 + rhs as u16 + c as u16;
    let result = sum as u8;

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = ((a & 0xF) + (rhs & 0xF) + c) > 0xF;
    cpu.f.c = sum > 0xFF;

    cpu.a = result;
}

pub fn alu_add(cpu: &mut CPU, rhs: u8) {
    let a = cpu.a;
    
    let sum = a as u16 + rhs as u16;
    let result = sum as u8;

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = ((a & 0xF) + (rhs & 0xF)) > 0xF;
    cpu.f.c = sum > 0xFF;

    cpu.a = result;
}

pub fn alu_cp(cpu: &mut CPU, rhs: u8) {
    let a = cpu.a;

    let result = a.wrapping_sub(rhs);

    cpu.f.z = result == 0;
    cpu.f.n = true;
    cpu.f.h = (a & 0xF) < (rhs & 0xF);
    cpu.f.c = rhs > a;
}

pub fn alu_dec(cpu: &mut CPU, val: u8) -> u8 {
    let result = val.wrapping_sub(1);

    cpu.f.z = result == 0;
    cpu.f.n = true;
    cpu.f.h = val & 0x0F == 0;

    result
}

pub fn alu_inc(cpu: &mut CPU, val: u8) -> u8 {
    let result = val.wrapping_add(1);

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = (val & 0xF) + 1 > 0xF;

    result
}

pub fn alu_sbc(cpu: &mut CPU, rhs: u8) {
    let a = cpu.a;
    let c = cpu.f.c as u8;

    let borrow = rhs as u16 + c as u16;
    let result = (a as u16).wrapping_sub(borrow);

    cpu.f.z = result as u8 == 0;
    cpu.f.n = true;
    cpu.f.h = (a & 0xF) < ((rhs & 0xF) + c);
    cpu.f.c = (a as u16) < borrow;

    cpu.a = result as u8;
}

pub fn alu_sub(cpu: &mut CPU, rhs: u8) {
    let a = cpu.a;

    let result = a.wrapping_sub(rhs);

    cpu.f.z = result == 0;
    cpu.f.n = true;
    cpu.f.h = (a & 0xF) < (rhs & 0xF);
    cpu.f.c = rhs > a;

    cpu.a = result;
}

// 16 BIT ARITHMETIC INSTRUCTIONS

pub fn alu_add16(cpu: &mut CPU, rhs: u16) {
    let hl = cpu.get_reg16(Reg16::HL);

    let sum = hl as u32 + rhs as u32;
    let result = sum as u16;

    cpu.f.n = false;
    cpu.f.h = ((hl & 0xFFF) + (rhs & 0xFFF)) > 0xFFF;
    cpu.f.c = sum > 0xFFFF;

    cpu.set_reg16(Reg16::HL, result);
}

pub fn alu_dec16(val: u16) -> u16 {
    val.wrapping_sub(1)
}

pub fn alu_inc16(val: u16) -> u16 {
    val.wrapping_add(1)
}


