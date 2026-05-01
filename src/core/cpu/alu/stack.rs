use super::super::CPU;

pub fn alu_add_sp_e8(cpu: &mut CPU, val: u8) -> u16 {
    let sp = cpu.sp;
    let offset = val as i8 as i16;
    
    let result = sp.wrapping_add(offset as u16);

    cpu.f.z = false;
    cpu.f.n = false;
    cpu.f.h = ((sp ^ (offset as u16) ^ result) & 0x10) != 0;
    cpu.f.c = ((sp ^ (offset as u16) ^ result) & 0x100) != 0;

    result
}