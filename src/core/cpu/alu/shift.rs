use super::super::CPU;

pub fn alu_rl(cpu: &mut CPU, val: u8) -> u8 {
    let c = if cpu.f.c { 0x01u8 } else {0x00u8 };

    let result = (val << 1) | c;

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = 0x80 & val != 0;

    result
}

pub fn alu_rla(cpu: &mut CPU) {
    let a = cpu.a;
    let c = if cpu.f.c { 0x01u8 } else {0x00u8 };

    let result = (a << 1) | c;

    cpu.f.z = false;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = 0x80 & a != 0;

    cpu.a = result;
}

pub fn alu_rlc(cpu: &mut CPU, val: u8) -> u8 {
    let result = val.rotate_left(1);

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = 0x80 & val != 0;

    result
}

pub fn alu_rlca(cpu: &mut CPU) {
    let a = cpu.a;

    let result = a.rotate_left(1);

    cpu.f.z = false;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = 0x80 & a != 0;

    cpu.a = result;
}

pub fn alu_rr(cpu: &mut CPU, val: u8) -> u8 {
    // 0bc0000000
    let c = if cpu.f.c { 0x80u8 } else { 0x00u8 }; 

    let result = (val >> 1) | c;

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = 0x01 & val != 0;

    result
}


pub fn alu_rra(cpu: &mut CPU) {
    // 0bc0000000
    let c = if cpu.f.c { 0x80u8 } else { 0x00u8 }; 
    let a = cpu.a;

    let result = (a >> 1) | c;

    cpu.f.z = false;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = 0x01 & a != 0;

    cpu.a = result;
}

pub fn alu_rrc(cpu: &mut CPU, val: u8) -> u8 {
    let result = val.rotate_right(1);

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = 0x01 & val != 0;

    result
}

pub fn alu_rrca(cpu: &mut CPU) {
    let a = cpu.a;
    let result = a.rotate_right(1);

    cpu.f.z = false;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = 0x01 & a != 0;

    cpu.a = result;
}

pub fn alu_sla(cpu: &mut CPU, val: u8) -> u8 {
    let result = val << 1;

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = (0x80 & val) != 0;

    result
}

pub fn alu_sra(cpu: &mut CPU, val: u8) -> u8 {
    let result = (val >> 1) | (val & 0x80);

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = 0x01 & val != 0;

    result
}

pub fn alu_srl(cpu: &mut CPU, val: u8) -> u8 {
    let result = val >> 1;

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = 0x01 & val != 0;

    result
}

pub fn alu_swap(cpu: &mut CPU, val: u8) -> u8 {
    let result = (val << 4) | (val >> 4);

    cpu.f.z = result == 0;
    cpu.f.n = false;
    cpu.f.h = false;
    cpu.f.c = false;

    result
}