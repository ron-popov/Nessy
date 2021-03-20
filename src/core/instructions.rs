use std::collections::HashMap;

#[derive(Clone)]
pub struct Instruction {
    pub opcode: u8,
    pub name: String,
    pub bytes: usize,
    pub mode: String
}

pub fn get_unknown_instruction() -> Instruction {
    Instruction{opcode:0xFF, name:"Unknown".to_string(), bytes: 1, mode:"Unknown".to_string()}
}

pub fn get_instruction_set() -> HashMap<u8, Instruction> {
    let mut map: HashMap<u8, Instruction> = HashMap::new();

    map.insert(0x69, Instruction{opcode: 0x69, name: "ADC".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0x65, Instruction{opcode: 0x65, name: "ADC".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x75, Instruction{opcode: 0x75, name: "ADC".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x6D, Instruction{opcode: 0x6D, name: "ADC".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x7D, Instruction{opcode: 0x7D, name: "ADC".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0x79, Instruction{opcode: 0x79, name: "ADC".to_string(), bytes:3, mode:"Absolute,Y".to_string()});
    map.insert(0x61, Instruction{opcode: 0x61, name: "ADC".to_string(), bytes:2, mode:"(Indirect,X)".to_string()});
    map.insert(0x71, Instruction{opcode: 0x71, name: "ADC".to_string(), bytes:2, mode:"(Indirect),Y".to_string()});
    map.insert(0x29, Instruction{opcode: 0x29, name: "AND".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0x25, Instruction{opcode: 0x25, name: "AND".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x35, Instruction{opcode: 0x35, name: "AND".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x2D, Instruction{opcode: 0x2D, name: "AND".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x3D, Instruction{opcode: 0x3D, name: "AND".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0x39, Instruction{opcode: 0x39, name: "AND".to_string(), bytes:3, mode:"Absolute,Y".to_string()});
    map.insert(0x21, Instruction{opcode: 0x21, name: "AND".to_string(), bytes:2, mode:"(Indirect,X)".to_string()});
    map.insert(0x31, Instruction{opcode: 0x31, name: "AND".to_string(), bytes:2, mode:"(Indirect),Y".to_string()});
    map.insert(0x0A, Instruction{opcode: 0x0A, name: "ASL".to_string(), bytes:1, mode:"Accumulator".to_string()});
    map.insert(0x06, Instruction{opcode: 0x06, name: "ASL".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x16, Instruction{opcode: 0x16, name: "ASL".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x0E, Instruction{opcode: 0x0E, name: "ASL".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x1E, Instruction{opcode: 0x1E, name: "ASL".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0x90, Instruction{opcode: 0x90, name: "BCC".to_string(), bytes:2, mode:"Relative".to_string()});
    map.insert(0xB0, Instruction{opcode: 0xB0, name: "BCS".to_string(), bytes:2, mode:"Relative".to_string()});
    map.insert(0xF0, Instruction{opcode: 0xF0, name: "BEQ".to_string(), bytes:2, mode:"Relative".to_string()});
    map.insert(0x24, Instruction{opcode: 0x24, name: "BIT".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x2C, Instruction{opcode: 0x2C, name: "BIT".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x30, Instruction{opcode: 0x30, name: "BMI".to_string(), bytes:2, mode:"Relative".to_string()});
    map.insert(0xD0, Instruction{opcode: 0xD0, name: "BNE".to_string(), bytes:2, mode:"Relative".to_string()});
    map.insert(0x10, Instruction{opcode: 0x10, name: "BPL".to_string(), bytes:2, mode:"Relative".to_string()});
    map.insert(0x00, Instruction{opcode: 0x00, name: "BRK".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x50, Instruction{opcode: 0x50, name: "BVC".to_string(), bytes:2, mode:"Relative".to_string()});
    map.insert(0x70, Instruction{opcode: 0x70, name: "BVS".to_string(), bytes:2, mode:"Relative".to_string()});
    map.insert(0x18, Instruction{opcode: 0x18, name: "CLC".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0xD8, Instruction{opcode: 0xD8, name: "CLD".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x58, Instruction{opcode: 0x58, name: "CLI".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0xB8, Instruction{opcode: 0xB8, name: "CLV".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0xC9, Instruction{opcode: 0xC9, name: "CMP".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0xC5, Instruction{opcode: 0xC5, name: "CMP".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0xD5, Instruction{opcode: 0xD5, name: "CMP".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0xCD, Instruction{opcode: 0xCD, name: "CMP".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0xDD, Instruction{opcode: 0xDD, name: "CMP".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0xD9, Instruction{opcode: 0xD9, name: "CMP".to_string(), bytes:3, mode:"Absolute,Y".to_string()});
    map.insert(0xC1, Instruction{opcode: 0xC1, name: "CMP".to_string(), bytes:2, mode:"(Indirect,X)".to_string()});
    map.insert(0xD1, Instruction{opcode: 0xD1, name: "CMP".to_string(), bytes:2, mode:"(Indirect),Y".to_string()});
    map.insert(0xE0, Instruction{opcode: 0xE0, name: "CPX".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0xE4, Instruction{opcode: 0xE4, name: "CPX".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0xEC, Instruction{opcode: 0xEC, name: "CPX".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0xC0, Instruction{opcode: 0xC0, name: "CPY".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0xC4, Instruction{opcode: 0xC4, name: "CPY".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0xCC, Instruction{opcode: 0xCC, name: "CPY".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0xC6, Instruction{opcode: 0xC6, name: "DEC".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0xD6, Instruction{opcode: 0xD6, name: "DEC".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0xCE, Instruction{opcode: 0xCE, name: "DEC".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0xDE, Instruction{opcode: 0xDE, name: "DEC".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0xCA, Instruction{opcode: 0xCA, name: "DEX".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x88, Instruction{opcode: 0x88, name: "DEY".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x49, Instruction{opcode: 0x49, name: "EOR".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0x45, Instruction{opcode: 0x45, name: "EOR".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x55, Instruction{opcode: 0x55, name: "EOR".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x4D, Instruction{opcode: 0x4D, name: "EOR".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x5D, Instruction{opcode: 0x5D, name: "EOR".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0x59, Instruction{opcode: 0x59, name: "EOR".to_string(), bytes:3, mode:"Absolute,Y".to_string()});
    map.insert(0x41, Instruction{opcode: 0x41, name: "EOR".to_string(), bytes:2, mode:"(Indirect,X)".to_string()});
    map.insert(0x51, Instruction{opcode: 0x51, name: "EOR".to_string(), bytes:2, mode:"(Indirect),Y".to_string()});
    map.insert(0xE6, Instruction{opcode: 0xE6, name: "INC".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0xF6, Instruction{opcode: 0xF6, name: "INC".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0xEE, Instruction{opcode: 0xEE, name: "INC".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0xFE, Instruction{opcode: 0xFE, name: "INC".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0xE8, Instruction{opcode: 0xE8, name: "INX".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0xC8, Instruction{opcode: 0xC8, name: "INY".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x4C, Instruction{opcode: 0x4C, name: "JMP".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x6C, Instruction{opcode: 0x6C, name: "JMP".to_string(), bytes:3, mode:"Indirect ".to_string()});
    map.insert(0x20, Instruction{opcode: 0x20, name: "JSR".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0xA9, Instruction{opcode: 0xA9, name: "LDA".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0xA5, Instruction{opcode: 0xA5, name: "LDA".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0xB5, Instruction{opcode: 0xB5, name: "LDA".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0xAD, Instruction{opcode: 0xAD, name: "LDA".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0xBD, Instruction{opcode: 0xBD, name: "LDA".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0xB9, Instruction{opcode: 0xB9, name: "LDA".to_string(), bytes:3, mode:"Absolute,Y".to_string()});
    map.insert(0xA1, Instruction{opcode: 0xA1, name: "LDA".to_string(), bytes:2, mode:"(Indirect,X)".to_string()});
    map.insert(0xB1, Instruction{opcode: 0xB1, name: "LDA".to_string(), bytes:2, mode:"(Indirect),Y".to_string()});
    map.insert(0xA2, Instruction{opcode: 0xA2, name: "LDX".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0xA6, Instruction{opcode: 0xA6, name: "LDX".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0xB6, Instruction{opcode: 0xB6, name: "LDX".to_string(), bytes:2, mode:"ZeroPage,Y".to_string()});
    map.insert(0xAE, Instruction{opcode: 0xAE, name: "LDX".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0xBE, Instruction{opcode: 0xBE, name: "LDX".to_string(), bytes:3, mode:"Absolute,Y".to_string()});
    map.insert(0xA0, Instruction{opcode: 0xA0, name: "LDY".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0xA4, Instruction{opcode: 0xA4, name: "LDY".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0xB4, Instruction{opcode: 0xB4, name: "LDY".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0xAC, Instruction{opcode: 0xAC, name: "LDY".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0xBC, Instruction{opcode: 0xBC, name: "LDY".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0x4A, Instruction{opcode: 0x4A, name: "LSR".to_string(), bytes:1, mode:"Accumulator".to_string()});
    map.insert(0x46, Instruction{opcode: 0x46, name: "LSR".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x56, Instruction{opcode: 0x56, name: "LSR".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x4E, Instruction{opcode: 0x4E, name: "LSR".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x5E, Instruction{opcode: 0x5E, name: "LSR".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0xEA, Instruction{opcode: 0xEA, name: "NOP".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x09, Instruction{opcode: 0x09, name: "ORA".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0x05, Instruction{opcode: 0x05, name: "ORA".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x15, Instruction{opcode: 0x15, name: "ORA".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x0D, Instruction{opcode: 0x0D, name: "ORA".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x1D, Instruction{opcode: 0x1D, name: "ORA".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0x19, Instruction{opcode: 0x19, name: "ORA".to_string(), bytes:3, mode:"Absolute,Y".to_string()});
    map.insert(0x01, Instruction{opcode: 0x01, name: "ORA".to_string(), bytes:2, mode:"(Indirect,X)".to_string()});
    map.insert(0x11, Instruction{opcode: 0x11, name: "ORA".to_string(), bytes:2, mode:"(Indirect),Y".to_string()});
    map.insert(0x48, Instruction{opcode: 0x48, name: "PHA".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x08, Instruction{opcode: 0x08, name: "PHP".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x68, Instruction{opcode: 0x68, name: "PLA".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x28, Instruction{opcode: 0x28, name: "PLP".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x2A, Instruction{opcode: 0x2A, name: "ROL".to_string(), bytes:1, mode:"Accumulator".to_string()});
    map.insert(0x26, Instruction{opcode: 0x26, name: "ROL".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x36, Instruction{opcode: 0x36, name: "ROL".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x2E, Instruction{opcode: 0x2E, name: "ROL".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x3E, Instruction{opcode: 0x3E, name: "ROL".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0x6A, Instruction{opcode: 0x6A, name: "ROR".to_string(), bytes:1, mode:"Accumulator".to_string()});
    map.insert(0x66, Instruction{opcode: 0x66, name: "ROR".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x76, Instruction{opcode: 0x76, name: "ROR".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x6E, Instruction{opcode: 0x6E, name: "ROR".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x7E, Instruction{opcode: 0x7E, name: "ROR".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0x40, Instruction{opcode: 0x40, name: "RTI".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x60, Instruction{opcode: 0x60, name: "RTS".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0xE9, Instruction{opcode: 0xE9, name: "SBC".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0xE5, Instruction{opcode: 0xE5, name: "SBC".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0xF5, Instruction{opcode: 0xF5, name: "SBC".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0xED, Instruction{opcode: 0xED, name: "SBC".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0xFD, Instruction{opcode: 0xFD, name: "SBC".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0xF9, Instruction{opcode: 0xF9, name: "SBC".to_string(), bytes:3, mode:"Absolute,Y".to_string()});
    map.insert(0xE1, Instruction{opcode: 0xE1, name: "SBC".to_string(), bytes:2, mode:"(Indirect,X)".to_string()});
    map.insert(0xF1, Instruction{opcode: 0xF1, name: "SBC".to_string(), bytes:2, mode:"(Indirect),Y".to_string()});
    map.insert(0x38, Instruction{opcode: 0x38, name: "SEC".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0xF8, Instruction{opcode: 0xF8, name: "SED".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x78, Instruction{opcode: 0x78, name: "SEI".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x85, Instruction{opcode: 0x85, name: "STA".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x95, Instruction{opcode: 0x95, name: "STA".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x8D, Instruction{opcode: 0x8D, name: "STA".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x9D, Instruction{opcode: 0x9D, name: "STA".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0x99, Instruction{opcode: 0x99, name: "STA".to_string(), bytes:3, mode:"Absolute,Y".to_string()});
    map.insert(0x81, Instruction{opcode: 0x81, name: "STA".to_string(), bytes:2, mode:"(Indirect,X)".to_string()});
    map.insert(0x91, Instruction{opcode: 0x91, name: "STA".to_string(), bytes:2, mode:"(Indirect),Y".to_string()});
    map.insert(0x86, Instruction{opcode: 0x86, name: "STX".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x96, Instruction{opcode: 0x96, name: "STX".to_string(), bytes:2, mode:"ZeroPage,Y".to_string()});
    map.insert(0x8E, Instruction{opcode: 0x8E, name: "STX".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x84, Instruction{opcode: 0x84, name: "STY".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x94, Instruction{opcode: 0x94, name: "STY".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x8C, Instruction{opcode: 0x8C, name: "STY".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0xAA, Instruction{opcode: 0xAA, name: "TAX".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0xA8, Instruction{opcode: 0xA8, name: "TAY".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0xBA, Instruction{opcode: 0xBA, name: "TSX".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x8A, Instruction{opcode: 0x8A, name: "TXA".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x9A, Instruction{opcode: 0x9A, name: "TXS".to_string(), bytes:1, mode:"Implied".to_string()});
    map.insert(0x98, Instruction{opcode: 0x98, name: "TYA".to_string(), bytes:1, mode:"Implied".to_string()});

    // Unofficial
    // Docs : https://wiki.nesdev.com/w/index.php/Programming_with_unofficial_opcodes
    map.insert(0x1A, Instruction{opcode: 0x1A, name: "NOP".to_string(), bytes:1, mode:"Unofficial".to_string()});
    map.insert(0x3A, Instruction{opcode: 0x3A, name: "NOP".to_string(), bytes:1, mode:"Unofficial".to_string()});
    map.insert(0x5A, Instruction{opcode: 0x5A, name: "NOP".to_string(), bytes:1, mode:"Unofficial".to_string()});
    map.insert(0x7A, Instruction{opcode: 0x7A, name: "NOP".to_string(), bytes:1, mode:"Unofficial".to_string()});
    map.insert(0xDA, Instruction{opcode: 0xDA, name: "NOP".to_string(), bytes:1, mode:"Unofficial".to_string()});
    map.insert(0xFA, Instruction{opcode: 0xFA, name: "NOP".to_string(), bytes:1, mode:"Unofficial".to_string()});
    
    map.insert(0x80, Instruction{opcode: 0x80, name: "IGN".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0x82, Instruction{opcode: 0x82, name: "IGN".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0x89, Instruction{opcode: 0x89, name: "IGN".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0xC2, Instruction{opcode: 0xC2, name: "IGN".to_string(), bytes:2, mode:"Immediate".to_string()});
    map.insert(0xE2, Instruction{opcode: 0xE2, name: "IGN".to_string(), bytes:2, mode:"Immediate".to_string()});

    map.insert(0x0C, Instruction{opcode: 0x0C, name: "IGN".to_string(), bytes:3, mode:"Absolute".to_string()});
    
    map.insert(0x1C, Instruction{opcode: 0x1C, name: "IGN".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0x3C, Instruction{opcode: 0x3C, name: "IGN".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0x5C, Instruction{opcode: 0x5C, name: "IGN".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0x7C, Instruction{opcode: 0x7C, name: "IGN".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0xDC, Instruction{opcode: 0xDC, name: "IGN".to_string(), bytes:3, mode:"Absolute,X".to_string()});
    map.insert(0xFC, Instruction{opcode: 0xFC, name: "IGN".to_string(), bytes:3, mode:"Absolute,X".to_string()});

    map.insert(0x04, Instruction{opcode: 0x04, name: "IGN".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x44, Instruction{opcode: 0x44, name: "IGN".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x64, Instruction{opcode: 0x64, name: "IGN".to_string(), bytes:2, mode:"ZeroPage".to_string()});

    map.insert(0x14, Instruction{opcode: 0x14, name: "IGN".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x34, Instruction{opcode: 0x34, name: "IGN".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x54, Instruction{opcode: 0x54, name: "IGN".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0x74, Instruction{opcode: 0x74, name: "IGN".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0xD4, Instruction{opcode: 0xD4, name: "IGN".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});
    map.insert(0xF4, Instruction{opcode: 0xF4, name: "IGN".to_string(), bytes:2, mode:"ZeroPage,X".to_string()});

    map.insert(0xA3, Instruction{opcode: 0xA3, name: "LAX".to_string(), bytes:3, mode:"(Indirect,X)".to_string()});
    map.insert(0xA7, Instruction{opcode: 0xA7, name: "LAX".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0xAF, Instruction{opcode: 0xAF, name: "LAX".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0xB3, Instruction{opcode: 0xB3, name: "LAX".to_string(), bytes:3, mode:"(Indirect),Y".to_string()});
    map.insert(0xB7, Instruction{opcode: 0xB7, name: "LAX".to_string(), bytes:2, mode:"ZeroPage,Y".to_string()});
    map.insert(0xBF, Instruction{opcode: 0xBF, name: "LAX".to_string(), bytes:3, mode:"Absolute,Y".to_string()});

    map.insert(0x83, Instruction{opcode: 0x83, name: "SAX".to_string(), bytes:3, mode:"(Indirect,X)".to_string()});
    map.insert(0x87, Instruction{opcode: 0x87, name: "SAX".to_string(), bytes:2, mode:"ZeroPage".to_string()});
    map.insert(0x8F, Instruction{opcode: 0x8F, name: "SAX".to_string(), bytes:3, mode:"Absolute".to_string()});
    map.insert(0x97, Instruction{opcode: 0x97, name: "SAX".to_string(), bytes:2, mode:"ZeroPage,Y".to_string()});

    map.insert(0xEB, Instruction{opcode: 0xEB, name: "SBC".to_string(), bytes:2, mode:"Immediate".to_string()});

    return map;
}