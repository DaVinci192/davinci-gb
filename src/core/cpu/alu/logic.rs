use super::super::CPU;

pub fn alu_and(cpu: &mut CPU, rhs: u8) {
    let a = cpu.a;

    let result = a & rhs;

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = true;
    cpu.f.c = false;

    cpu.a = result;
}

pub fn alu_cpl(cpu: &mut CPU) {
    let a = cpu.a;

    let result = !a;

    cpu.f.n = true;
    cpu.f.h = true;

    cpu.a = result;
}

pub fn alu_or(cpu: &mut CPU, rhs: u8) {
    let a = cpu.a;

    let result = a | rhs;

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = false;

    cpu.a = result;
}

pub fn alu_xor(cpu: &mut CPU, rhs: u8) {
    let a = cpu.a;

    let result = a ^ rhs;

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = false;

    cpu.a = result;
}