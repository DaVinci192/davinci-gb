#[derive(Copy, Clone, Debug)]
pub struct OpInfo {
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles_min: u8,
    pub cycles_max: u8,
}

impl OpInfo {
    pub fn format_mnemonic(&self, pc: u16, bytes: &[u8], is_cb_prefixed: bool) -> String {
        let mut s = self.mnemonic.to_string();

        let mut bytes_off = 0 as usize;
        if is_cb_prefixed {
            bytes_off += 1;
        }

        if s.contains("{n8}") {
            s = s.replace("{n8}", &format!("${:02X}", bytes[1 + bytes_off]));
        }

        if s.contains("{n16}") {
            let val = u16::from_le_bytes([bytes[1 + bytes_off], bytes[2 + bytes_off]]);
            s = s.replace("{n16}", &format!("${:04X}", val));
        }

        // no cb prefid opcode matches, so pc manipulation not needed here
        if s.contains("{e8}") {
            let off = bytes[1 + bytes_off] as i8;
            let target = pc.wrapping_add(2).wrapping_add_signed(off as i16);
            s = s.replace("{e8}", &format!("{:+} (${target:04X})", off));
        }

        s
    }
}


const fn op(mnemonic: &'static str, len: u8, cmin: u8, cmax: u8) -> OpInfo {
    OpInfo { mnemonic, len, cycles_min: cmin, cycles_max: cmax }
}

pub static OP_INFO: [OpInfo; 256] = [
    op("NOP",1,1,1), op("LD BC,{n16}",3,3,3), op("LD [BC],A",1,2,2), op("INC BC",1,2,2), op("INC B",1,1,1), op("DEC B",1,1,1), op("LD B,{n8}",2,2,2), op("RLCA",1,1,1),
    op("LD [{n16}],SP",3,5,5), op("ADD HL,BC",1,2,2), op("LD A,[BC]",1,2,2), op("DEC BC",1,2,2), op("INC C",1,1,1), op("DEC C",1,1,1), op("LD C,{n8}",2,2,2), op("RRCA",1,1,1),
    op("STOP",2,1,1), op("LD DE,{n16}",3,3,3), op("LD [DE],A",1,2,2), op("INC DE",1,2,2), op("INC D",1,1,1), op("DEC D",1,1,1), op("LD D,{n8}",2,2,2), op("RLA",1,1,1),
    op("JR {e8}",2,3,3), op("ADD HL,DE",1,2,2), op("LD A,[DE]",1,2,2), op("DEC DE",1,2,2), op("INC E",1,1,1), op("DEC E",1,1,1), op("LD E,{n8}",2,2,2), op("RRA",1,1,1),
    op("JR NZ,{e8}",2,2,3), op("LD HL,{n16}",3,3,3), op("LD [HL+],A",1,2,2), op("INC HL",1,2,2), op("INC H",1,1,1), op("DEC H",1,1,1), op("LD H,{n8}",2,2,2), op("DAA",1,1,1),
    op("JR Z,{e8}",2,2,3), op("ADD HL,HL",1,2,2), op("LD A,[HL+]",1,2,2), op("DEC HL",1,2,2), op("INC L",1,1,1), op("DEC L",1,1,1), op("LD L,{n8}",2,2,2), op("CPL",1,1,1),
    op("JR NC,{e8}",2,2,3), op("LD SP,{n16}",3,3,3), op("LD [HL-],A",1,2,2), op("INC SP",1,2,2), op("INC [HL]",1,3,3), op("DEC [HL]",1,3,3), op("LD [HL],{n8}",2,3,3), op("SCF",1,1,1),
    op("JR C,{e8}",2,2,3), op("ADD HL,SP",1,2,2), op("LD A,[HL-]",1,2,2), op("DEC SP",1,2,2), op("INC A",1,1,1), op("DEC A",1,1,1), op("LD A,{n8}",2,2,2), op("CCF",1,1,1),

    op("LD B,B",1,1,1), op("LD B,C",1,1,1), op("LD B,D",1,1,1), op("LD B,E",1,1,1), op("LD B,H",1,1,1), op("LD B,L",1,1,1), op("LD B,[HL]",1,2,2), op("LD B,A",1,1,1),
    op("LD C,B",1,1,1), op("LD C,C",1,1,1), op("LD C,D",1,1,1), op("LD C,E",1,1,1), op("LD C,H",1,1,1), op("LD C,L",1,1,1), op("LD C,[HL]",1,2,2), op("LD C,A",1,1,1),
    op("LD D,B",1,1,1), op("LD D,C",1,1,1), op("LD D,D",1,1,1), op("LD D,E",1,1,1), op("LD D,H",1,1,1), op("LD D,L",1,1,1), op("LD D,[HL]",1,2,2), op("LD D,A",1,1,1),
    op("LD E,B",1,1,1), op("LD E,C",1,1,1), op("LD E,D",1,1,1), op("LD E,E",1,1,1), op("LD E,H",1,1,1), op("LD E,L",1,1,1), op("LD E,[HL]",1,2,2), op("LD E,A",1,1,1),
    op("LD H,B",1,1,1), op("LD H,C",1,1,1), op("LD H,D",1,1,1), op("LD H,E",1,1,1), op("LD H,H",1,1,1), op("LD H,L",1,1,1), op("LD H,[HL]",1,2,2), op("LD H,A",1,1,1),
    op("LD L,B",1,1,1), op("LD L,C",1,1,1), op("LD L,D",1,1,1), op("LD L,E",1,1,1), op("LD L,H",1,1,1), op("LD L,L",1,1,1), op("LD L,[HL]",1,2,2), op("LD L,A",1,1,1),
    op("LD [HL],B",1,2,2), op("LD [HL],C",1,2,2), op("LD [HL],D",1,2,2), op("LD [HL],E",1,2,2), op("LD [HL],H",1,2,2), op("LD [HL],L",1,2,2), op("HALT",1,1,1), op("LD [HL],A",1,2,2),
    op("LD A,B",1,1,1), op("LD A,C",1,1,1), op("LD A,D",1,1,1), op("LD A,E",1,1,1), op("LD A,H",1,1,1), op("LD A,L",1,1,1), op("LD A,[HL]",1,2,2), op("LD A,A",1,1,1),

    op("ADD A,B",1,1,1), op("ADD A,C",1,1,1), op("ADD A,D",1,1,1), op("ADD A,E",1,1,1), op("ADD A,H",1,1,1), op("ADD A,L",1,1,1), op("ADD A,[HL]",1,2,2), op("ADD A,A",1,1,1),
    op("ADC A,B",1,1,1), op("ADC A,C",1,1,1), op("ADC A,D",1,1,1), op("ADC A,E",1,1,1), op("ADC A,H",1,1,1), op("ADC A,L",1,1,1), op("ADC A,[HL]",1,2,2), op("ADC A,A",1,1,1),
    op("SUB A,B",1,1,1), op("SUB A,C",1,1,1), op("SUB A,D",1,1,1), op("SUB A,E",1,1,1), op("SUB A,H",1,1,1), op("SUB A,L",1,1,1), op("SUB A,[HL]",1,2,2), op("SUB A,A",1,1,1),
    op("SBC A,B",1,1,1), op("SBC A,C",1,1,1), op("SBC A,D",1,1,1), op("SBC A,E",1,1,1), op("SBC A,H",1,1,1), op("SBC A,L",1,1,1), op("SBC A,[HL]",1,2,2), op("SBC A,A",1,1,1),
    op("AND A,B",1,1,1), op("AND A,C",1,1,1), op("AND A,D",1,1,1), op("AND A,E",1,1,1), op("AND A,H",1,1,1), op("AND A,L",1,1,1), op("AND A,[HL]",1,2,2), op("AND A,A",1,1,1),
    op("XOR A,B",1,1,1), op("XOR A,C",1,1,1), op("XOR A,D",1,1,1), op("XOR A,E",1,1,1), op("XOR A,H",1,1,1), op("XOR A,L",1,1,1), op("XOR A,[HL]",1,2,2), op("XOR A,A",1,1,1),
    op("OR A,B",1,1,1), op("OR A,C",1,1,1), op("OR A,D",1,1,1), op("OR A,E",1,1,1), op("OR A,H",1,1,1), op("OR A,L",1,1,1), op("OR A,[HL]",1,2,2), op("OR A,A",1,1,1),
    op("CP A,B",1,1,1), op("CP A,C",1,1,1), op("CP A,D",1,1,1), op("CP A,E",1,1,1), op("CP A,H",1,1,1), op("CP A,L",1,1,1), op("CP A,[HL]",1,2,2), op("CP A,A",1,1,1),

    op("RET NZ",1,2,5), op("POP BC",1,3,3), op("JP NZ,{n16}",3,3,4), op("JP {n16}",3,4,4), op("CALL NZ,{n16}",3,3,6), op("PUSH BC",1,4,4), op("ADD A,{n8}",2,2,2), op("RST $00",1,4,4),
    op("RET Z",1,2,5), op("RET",1,4,4), op("JP Z,{n16}",3,3,4), op("PREFIX CB",1,1,1), op("CALL Z,{n16}",3,3,6), op("CALL {n16}",3,6,6), op("ADC A,{n8}",2,2,2), op("RST $08",1,4,4),
    op("RET NC",1,2,5), op("POP DE",1,3,3), op("JP NC,{n16}",3,3,4), op("INVALID",1,1,1), op("CALL NC,{n16}",3,3,6), op("PUSH DE",1,4,4), op("SUB A,{n8}",2,2,2), op("RST $10",1,4,4),
    op("RET C",1,2,5), op("RETI",1,4,4), op("JP C,{n16}",3,3,4), op("INVALID",1,1,1), op("CALL C,{n16}",3,3,6), op("INVALID",1,1,1), op("SBC A,{n8}",2,2,2), op("RST $18",1,4,4),
    op("LDH [{a8}],A",2,3,3), op("POP HL",1,3,3), op("LDH [C],A",1,2,2), op("INVALID",1,1,1), op("INVALID",1,1,1), op("PUSH HL",1,4,4), op("AND A,{n8}",2,2,2), op("RST $20",1,4,4),
    op("ADD SP,{e8}",2,4,4), op("JP HL",1,1,1), op("LD [{n16}],A",3,4,4), op("INVALID",1,1,1), op("INVALID",1,1,1), op("INVALID",1,1,1), op("XOR A,{n8}",2,2,2), op("RST $28",1,4,4),
    op("LDH A,[{a8}]",2,3,3), op("POP AF",1,3,3), op("LD A,[C]",1,2,2), op("DI",1,1,1), op("INVALID",1,1,1), op("PUSH AF",1,4,4), op("OR A,{n8}",2,2,2), op("RST $30",1,4,4),
    op("LD HL,SP+{e8}",2,3,3), op("LD SP,HL",1,2,2), op("LD A,[{n16}]",3,4,4), op("EI",1,1,1), op("INVALID",1,1,1), op("INVALID",1,1,1), op("CP A,{n8}",2,2,2), op("RST $38",1,4,4),
];

pub static CB_OP_INFO: [OpInfo; 256] = [
    // RLC r
    op("RLC B",2,2,2), op("RLC C",2,2,2), op("RLC D",2,2,2), op("RLC E",2,2,2), op("RLC H",2,2,2), op("RLC L",2,2,2), op("RLC [HL]",2,4,4), op("RLC A",2,2,2),
    // RRC r
    op("RRC B",2,2,2), op("RRC C",2,2,2), op("RRC D",2,2,2), op("RRC E",2,2,2), op("RRC H",2,2,2), op("RRC L",2,2,2), op("RRC [HL]",2,4,4), op("RRC A",2,2,2),
    // RL r
    op("RL B",2,2,2), op("RL C",2,2,2), op("RL D",2,2,2), op("RL E",2,2,2), op("RL H",2,2,2), op("RL L",2,2,2), op("RL [HL]",2,4,4), op("RL A",2,2,2),
    // RR r
    op("RR B",2,2,2), op("RR C",2,2,2), op("RR D",2,2,2), op("RR E",2,2,2), op("RR H",2,2,2), op("RR L",2,2,2), op("RR [HL]",2,4,4), op("RR A",2,2,2),
    // SLA r
    op("SLA B",2,2,2), op("SLA C",2,2,2), op("SLA D",2,2,2), op("SLA E",2,2,2), op("SLA H",2,2,2), op("SLA L",2,2,2), op("SLA [HL]",2,4,4), op("SLA A",2,2,2),
    // SRA r
    op("SRA B",2,2,2), op("SRA C",2,2,2), op("SRA D",2,2,2), op("SRA E",2,2,2), op("SRA H",2,2,2), op("SRA L",2,2,2), op("SRA [HL]",2,4,4), op("SRA A",2,2,2),
    // SWAP r
    op("SWAP B",2,2,2), op("SWAP C",2,2,2), op("SWAP D",2,2,2), op("SWAP E",2,2,2), op("SWAP H",2,2,2), op("SWAP L",2,2,2), op("SWAP [HL]",2,4,4), op("SWAP A",2,2,2),
    // SRL r
    op("SRL B",2,2,2), op("SRL C",2,2,2), op("SRL D",2,2,2), op("SRL E",2,2,2), op("SRL H",2,2,2), op("SRL L",2,2,2), op("SRL [HL]",2,4,4), op("SRL A",2,2,2),

    // BIT 0-7
    op("BIT 0,B",2,2,2), op("BIT 0,C",2,2,2), op("BIT 0,D",2,2,2), op("BIT 0,E",2,2,2), op("BIT 0,H",2,2,2), op("BIT 0,L",2,2,2), op("BIT 0,[HL]",2,2,3), op("BIT 0,A",2,2,2),
    op("BIT 1,B",2,2,2), op("BIT 1,C",2,2,2), op("BIT 1,D",2,2,2), op("BIT 1,E",2,2,2), op("BIT 1,H",2,2,2), op("BIT 1,L",2,2,2), op("BIT 1,[HL]",2,2,3), op("BIT 1,A",2,2,2),
    op("BIT 2,B",2,2,2), op("BIT 2,C",2,2,2), op("BIT 2,D",2,2,2), op("BIT 2,E",2,2,2), op("BIT 2,H",2,2,2), op("BIT 2,L",2,2,2), op("BIT 2,[HL]",2,2,3), op("BIT 2,A",2,2,2),
    op("BIT 3,B",2,2,2), op("BIT 3,C",2,2,2), op("BIT 3,D",2,2,2), op("BIT 3,E",2,2,2), op("BIT 3,H",2,2,2), op("BIT 3,L",2,2,2), op("BIT 3,[HL]",2,2,3), op("BIT 3,A",2,2,2),
    op("BIT 4,B",2,2,2), op("BIT 4,C",2,2,2), op("BIT 4,D",2,2,2), op("BIT 4,E",2,2,2), op("BIT 4,H",2,2,2), op("BIT 4,L",2,2,2), op("BIT 4,[HL]",2,2,3), op("BIT 4,A",2,2,2),
    op("BIT 5,B",2,2,2), op("BIT 5,C",2,2,2), op("BIT 5,D",2,2,2), op("BIT 5,E",2,2,2), op("BIT 5,H",2,2,2), op("BIT 5,L",2,2,2), op("BIT 5,[HL]",2,2,3), op("BIT 5,A",2,2,2),
    op("BIT 6,B",2,2,2), op("BIT 6,C",2,2,2), op("BIT 6,D",2,2,2), op("BIT 6,E",2,2,2), op("BIT 6,H",2,2,2), op("BIT 6,L",2,2,2), op("BIT 6,[HL]",2,2,3), op("BIT 6,A",2,2,2),
    op("BIT 7,B",2,2,2), op("BIT 7,C",2,2,2), op("BIT 7,D",2,2,2), op("BIT 7,E",2,2,2), op("BIT 7,H",2,2,2), op("BIT 7,L",2,2,2), op("BIT 7,[HL]",2,2,3), op("BIT 7,A",2,2,2),

    // RES 0-7
    op("RES 0,B",2,2,2), op("RES 0,C",2,2,2), op("RES 0,D",2,2,2), op("RES 0,E",2,2,2), op("RES 0,H",2,2,2), op("RES 0,L",2,2,2), op("RES 0,[HL]",2,2,4), op("RES 0,A",2,2,2),
    op("RES 1,B",2,2,2), op("RES 1,C",2,2,2), op("RES 1,D",2,2,2), op("RES 1,E",2,2,2), op("RES 1,H",2,2,2), op("RES 1,L",2,2,2), op("RES 1,[HL]",2,2,4), op("RES 1,A",2,2,2),
    op("RES 2,B",2,2,2), op("RES 2,C",2,2,2), op("RES 2,D",2,2,2), op("RES 2,E",2,2,2), op("RES 2,H",2,2,2), op("RES 2,L",2,2,2), op("RES 2,[HL]",2,2,4), op("RES 2,A",2,2,2),
    op("RES 3,B",2,2,2), op("RES 3,C",2,2,2), op("RES 3,D",2,2,2), op("RES 3,E",2,2,2), op("RES 3,H",2,2,2), op("RES 3,L",2,2,2), op("RES 3,[HL]",2,2,4), op("RES 3,A",2,2,2),
    op("RES 4,B",2,2,2), op("RES 4,C",2,2,2), op("RES 4,D",2,2,2), op("RES 4,E",2,2,2), op("RES 4,H",2,2,2), op("RES 4,L",2,2,2), op("RES 4,[HL]",2,2,4), op("RES 4,A",2,2,2),
    op("RES 5,B",2,2,2), op("RES 5,C",2,2,2), op("RES 5,D",2,2,2), op("RES 5,E",2,2,2), op("RES 5,H",2,2,2), op("RES 5,L",2,2,2), op("RES 5,[HL]",2,2,4), op("RES 5,A",2,2,2),
    op("RES 6,B",2,2,2), op("RES 6,C",2,2,2), op("RES 6,D",2,2,2), op("RES 6,E",2,2,2), op("RES 6,H",2,2,2), op("RES 6,L",2,2,2), op("RES 6,[HL]",2,2,4), op("RES 6,A",2,2,2),
    op("RES 7,B",2,2,2), op("RES 7,C",2,2,2), op("RES 7,D",2,2,2), op("RES 7,E",2,2,2), op("RES 7,H",2,2,2), op("RES 7,L",2,2,2), op("RES 7,[HL]",2,2,4), op("RES 7,A",2,2,2),

    // SET 0-7
    op("SET 0,B",2,2,2), op("SET 0,C",2,2,2), op("SET 0,D",2,2,2), op("SET 0,E",2,2,2), op("SET 0,H",2,2,2), op("SET 0,L",2,2,2), op("SET 0,[HL]",2,2,4), op("SET 0,A",2,2,2),
    op("SET 1,B",2,2,2), op("SET 1,C",2,2,2), op("SET 1,D",2,2,2), op("SET 1,E",2,2,2), op("SET 1,H",2,2,2), op("SET 1,L",2,2,2), op("SET 1,[HL]",2,2,4), op("SET 1,A",2,2,2),
    op("SET 2,B",2,2,2), op("SET 2,C",2,2,2), op("SET 2,D",2,2,2), op("SET 2,E",2,2,2), op("SET 2,H",2,2,2), op("SET 2,L",2,2,2), op("SET 2,[HL]",2,2,4), op("SET 2,A",2,2,2),
    op("SET 3,B",2,2,2), op("SET 3,C",2,2,2), op("SET 3,D",2,2,2), op("SET 3,E",2,2,2), op("SET 3,H",2,2,2), op("SET 3,L",2,2,2), op("SET 3,[HL]",2,2,4), op("SET 3,A",2,2,2),
    op("SET 4,B",2,2,2), op("SET 4,C",2,2,2), op("SET 4,D",2,2,2), op("SET 4,E",2,2,2), op("SET 4,H",2,2,2), op("SET 4,L",2,2,2), op("SET 4,[HL]",2,2,4), op("SET 4,A",2,2,2),
    op("SET 5,B",2,2,2), op("SET 5,C",2,2,2), op("SET 5,D",2,2,2), op("SET 5,E",2,2,2), op("SET 5,H",2,2,2), op("SET 5,L",2,2,2), op("SET 5,[HL]",2,2,4), op("SET 5,A",2,2,2),
    op("SET 6,B",2,2,2), op("SET 6,C",2,2,2), op("SET 6,D",2,2,2), op("SET 6,E",2,2,2), op("SET 6,H",2,2,2), op("SET 6,L",2,2,2), op("SET 6,[HL]",2,2,4), op("SET 6,A",2,2,2),
    op("SET 7,B",2,2,2), op("SET 7,C",2,2,2), op("SET 7,D",2,2,2), op("SET 7,E",2,2,2), op("SET 7,H",2,2,2), op("SET 7,L",2,2,2), op("SET 7,[HL]",2,2,4), op("SET 7,A",2,2,2),
];