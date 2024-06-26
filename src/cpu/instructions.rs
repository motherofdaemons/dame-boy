pub enum Instruction {
    Nop,
    Add(ArithmeticTarget),
    AddCarry(ArithmeticTarget),
    Sub(ArithmeticTarget),
    SubCarry(ArithmeticTarget),
    And(ArithmeticTarget),
    Xor(ArithmeticTarget),
    Or(ArithmeticTarget),
    Cp(ArithmeticTarget),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    IndirectHL,
}

impl From<u8> for Instruction {
    fn from(opcode: u8) -> Self {
        match opcode {
            0x00 => Self::Nop,
            0x80 => Self::Add(ArithmeticTarget::B),
            0x81 => Self::Add(ArithmeticTarget::C),
            0x82 => Self::Add(ArithmeticTarget::D),
            0x83 => Self::Add(ArithmeticTarget::E),
            0x84 => Self::Add(ArithmeticTarget::H),
            0x85 => Self::Add(ArithmeticTarget::L),
            0x86 => Self::Add(ArithmeticTarget::IndirectHL),
            0x87 => Self::Add(ArithmeticTarget::A),
            0x88 => Self::AddCarry(ArithmeticTarget::B),
            0x89 => Self::AddCarry(ArithmeticTarget::C),
            0x8A => Self::AddCarry(ArithmeticTarget::D),
            0x8B => Self::AddCarry(ArithmeticTarget::E),
            0x8C => Self::AddCarry(ArithmeticTarget::H),
            0x8D => Self::AddCarry(ArithmeticTarget::L),
            0x8E => Self::AddCarry(ArithmeticTarget::IndirectHL),
            0x8F => Self::AddCarry(ArithmeticTarget::A),
            0x90 => Self::Sub(ArithmeticTarget::B),
            0x91 => Self::Sub(ArithmeticTarget::C),
            0x92 => Self::Sub(ArithmeticTarget::D),
            0x93 => Self::Sub(ArithmeticTarget::E),
            0x94 => Self::Sub(ArithmeticTarget::H),
            0x95 => Self::Sub(ArithmeticTarget::L),
            0x96 => Self::Sub(ArithmeticTarget::IndirectHL),
            0x97 => Self::Sub(ArithmeticTarget::A),
            0x98 => Self::SubCarry(ArithmeticTarget::B),
            0x99 => Self::SubCarry(ArithmeticTarget::C),
            0x9A => Self::SubCarry(ArithmeticTarget::D),
            0x9B => Self::SubCarry(ArithmeticTarget::E),
            0x9C => Self::SubCarry(ArithmeticTarget::H),
            0x9D => Self::SubCarry(ArithmeticTarget::L),
            0x9E => Self::SubCarry(ArithmeticTarget::IndirectHL),
            0x9F => Self::SubCarry(ArithmeticTarget::A),
            0xA0 => Self::And(ArithmeticTarget::B),
            0xA1 => Self::And(ArithmeticTarget::C),
            0xA2 => Self::And(ArithmeticTarget::D),
            0xA3 => Self::And(ArithmeticTarget::E),
            0xA4 => Self::And(ArithmeticTarget::H),
            0xA5 => Self::And(ArithmeticTarget::L),
            0xA6 => Self::And(ArithmeticTarget::IndirectHL),
            0xA7 => Self::And(ArithmeticTarget::A),
            0xA8 => Self::Xor(ArithmeticTarget::B),
            0xA9 => Self::Xor(ArithmeticTarget::C),
            0xAA => Self::Xor(ArithmeticTarget::D),
            0xAB => Self::Xor(ArithmeticTarget::E),
            0xAC => Self::Xor(ArithmeticTarget::H),
            0xAD => Self::Xor(ArithmeticTarget::L),
            0xAE => Self::Xor(ArithmeticTarget::IndirectHL),
            0xAF => Self::Xor(ArithmeticTarget::A),
            0xB0 => Self::Or(ArithmeticTarget::B),
            0xB1 => Self::Or(ArithmeticTarget::C),
            0xB2 => Self::Or(ArithmeticTarget::D),
            0xB3 => Self::Or(ArithmeticTarget::E),
            0xB4 => Self::Or(ArithmeticTarget::H),
            0xB5 => Self::Or(ArithmeticTarget::L),
            0xB6 => Self::Or(ArithmeticTarget::IndirectHL),
            0xB7 => Self::Or(ArithmeticTarget::A),
            0xB8 => Self::Cp(ArithmeticTarget::B),
            0xB9 => Self::Cp(ArithmeticTarget::C),
            0xBA => Self::Cp(ArithmeticTarget::D),
            0xBB => Self::Cp(ArithmeticTarget::E),
            0xBC => Self::Cp(ArithmeticTarget::H),
            0xBD => Self::Cp(ArithmeticTarget::L),
            0xBE => Self::Cp(ArithmeticTarget::IndirectHL),
            0xBF => Self::Cp(ArithmeticTarget::A),
            opcode => unimplemented!("{opcode:X} isn't implemented yet!"),
        }
    }
}
