use super::super::CPU;

pub fn alu_add_sp_e8(cpu: &mut CPU, val: u8) {
    let offset = val as i8 as i16;
    let result = cpu.sp.wrapping_add(offset as u16);

    let h = ((cpu.sp & 0xF) + (offset as u16 & 0xF)) > 0xF;
    let c = ((cpu.sp & 0xFF) + (offset as u16 & 0xFF)) > 0xFF;

    cpu.f.z = false;
    cpu.f.n = false;
    cpu.f.h = h;
    cpu.f.c = c;

    cpu.sp = result;
}