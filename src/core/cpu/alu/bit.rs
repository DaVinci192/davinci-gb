use super::super::CPU;

pub fn alu_bit(cpu: &mut CPU, bit: u8, val: u8) {
    let result = (0x01u8 << bit) & val == 0;

    cpu.f.z = result;
    cpu.f.n = false;
    cpu.f.h = true;
}

pub fn alu_res(_cpu: &mut CPU, bit: u8, val: u8) -> u8 {
    val & !(1u8 << bit)
}

pub fn alu_set(_cpu: &mut CPU, bit: u8, val: u8) -> u8 {
    val | (1u8 << bit)
}

pub fn alu_ccf(cpu: &mut CPU) {
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = !cpu.f.c;
}

pub fn alu_scf(cpu: &mut CPU) {
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = true;
}