pub enum Instruction {
    Nop,
    Add(ArithmeticTarget),
    AddCarry(ArithmeticTarget),
    Sub(ArithmeticTarget),
    SubCarry(ArithmeticTarget),
    And(ArithmeticTarget),
    Xor(ArithmeticTarget),
    Or(ArithmeticTarget),
    Compare(ArithmeticTarget),
    Load { dst: LoadTarget, src: LoadTarget },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WideRegister {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticTarget {
    Register(Register),
    IndirectHl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadTarget {
    Register(Register),
    WideRegister(WideRegister),
    Immediate8,
    Immediate16,
    IndirectWideRegister(WideRegister),
    IndirectHlInc,
    IndirectHlDec,
}

impl From<u8> for Instruction {
    fn from(opcode: u8) -> Self {
        match opcode {
            0x00 => Self::Nop,
            0x01 => Self::Load {
                dst: LoadTarget::WideRegister(WideRegister::BC),
                src: LoadTarget::Immediate16,
            },
            0x11 => Self::Load {
                dst: LoadTarget::WideRegister(WideRegister::DE),
                src: LoadTarget::Immediate16,
            },
            0x21 => Self::Load {
                dst: LoadTarget::WideRegister(WideRegister::HL),
                src: LoadTarget::Immediate16,
            },
            0x31 => Self::Load {
                dst: LoadTarget::WideRegister(WideRegister::SP),
                src: LoadTarget::Immediate16,
            },
            0x02 => Self::Load {
                dst: LoadTarget::IndirectWideRegister(WideRegister::BC),
                src: LoadTarget::Immediate16,
            },
            0x12 => Self::Load {
                dst: LoadTarget::IndirectWideRegister(WideRegister::DE),
                src: LoadTarget::Immediate16,
            },
            0x22 => Self::Load {
                dst: LoadTarget::IndirectHlInc,
                src: LoadTarget::Immediate16,
            },
            0x32 => Self::Load {
                dst: LoadTarget::IndirectHlDec,
                src: LoadTarget::Immediate16,
            },
            0x06 => Self::Load {
                dst: LoadTarget::Register(Register::B),
                src: LoadTarget::Immediate8,
            },
            0x16 => Self::Load {
                dst: LoadTarget::Register(Register::D),
                src: LoadTarget::Immediate8,
            },
            0x26 => Self::Load {
                dst: LoadTarget::Register(Register::H),
                src: LoadTarget::Immediate8,
            },
            0x36 => Self::Load {
                dst: LoadTarget::IndirectWideRegister(WideRegister::HL),
                src: LoadTarget::Immediate8,
            },
            0xA0 => Self::Load {
                dst: LoadTarget::Register(Register::A),
                src: LoadTarget::IndirectWideRegister(WideRegister::BC),
            },
            0xA1 => Self::Load {
                dst: LoadTarget::Register(Register::A),
                src: LoadTarget::IndirectWideRegister(WideRegister::DE),
            },
            0x80 => Self::Add(ArithmeticTarget::Register(Register::B)),
            0x81 => Self::Add(ArithmeticTarget::Register(Register::C)),
            0x82 => Self::Add(ArithmeticTarget::Register(Register::D)),
            0x83 => Self::Add(ArithmeticTarget::Register(Register::E)),
            0x84 => Self::Add(ArithmeticTarget::Register(Register::H)),
            0x85 => Self::Add(ArithmeticTarget::Register(Register::L)),
            0x86 => Self::Add(ArithmeticTarget::IndirectHl),
            0x87 => Self::Add(ArithmeticTarget::Register(Register::A)),
            0x88 => Self::AddCarry(ArithmeticTarget::Register(Register::B)),
            0x89 => Self::AddCarry(ArithmeticTarget::Register(Register::C)),
            0x8A => Self::AddCarry(ArithmeticTarget::Register(Register::D)),
            0x8B => Self::AddCarry(ArithmeticTarget::Register(Register::E)),
            0x8C => Self::AddCarry(ArithmeticTarget::Register(Register::H)),
            0x8D => Self::AddCarry(ArithmeticTarget::Register(Register::L)),
            0x8E => Self::AddCarry(ArithmeticTarget::IndirectHl),
            0x8F => Self::AddCarry(ArithmeticTarget::Register(Register::A)),
            0x90 => Self::Sub(ArithmeticTarget::Register(Register::B)),
            0x91 => Self::Sub(ArithmeticTarget::Register(Register::C)),
            0x92 => Self::Sub(ArithmeticTarget::Register(Register::D)),
            0x93 => Self::Sub(ArithmeticTarget::Register(Register::E)),
            0x94 => Self::Sub(ArithmeticTarget::Register(Register::H)),
            0x95 => Self::Sub(ArithmeticTarget::Register(Register::L)),
            0x96 => Self::Sub(ArithmeticTarget::IndirectHl),
            0x97 => Self::Sub(ArithmeticTarget::Register(Register::A)),
            0x98 => Self::SubCarry(ArithmeticTarget::Register(Register::B)),
            0x99 => Self::SubCarry(ArithmeticTarget::Register(Register::C)),
            0x9A => Self::SubCarry(ArithmeticTarget::Register(Register::D)),
            0x9B => Self::SubCarry(ArithmeticTarget::Register(Register::E)),
            0x9C => Self::SubCarry(ArithmeticTarget::Register(Register::H)),
            0x9D => Self::SubCarry(ArithmeticTarget::Register(Register::L)),
            0x9E => Self::SubCarry(ArithmeticTarget::IndirectHl),
            0x9F => Self::SubCarry(ArithmeticTarget::Register(Register::A)),
            0xA0 => Self::And(ArithmeticTarget::Register(Register::B)),
            0xA1 => Self::And(ArithmeticTarget::Register(Register::C)),
            0xA2 => Self::And(ArithmeticTarget::Register(Register::D)),
            0xA3 => Self::And(ArithmeticTarget::Register(Register::E)),
            0xA4 => Self::And(ArithmeticTarget::Register(Register::H)),
            0xA5 => Self::And(ArithmeticTarget::Register(Register::L)),
            0xA6 => Self::And(ArithmeticTarget::IndirectHl),
            0xA7 => Self::And(ArithmeticTarget::Register(Register::A)),
            0xA8 => Self::Xor(ArithmeticTarget::Register(Register::B)),
            0xA9 => Self::Xor(ArithmeticTarget::Register(Register::C)),
            0xAA => Self::Xor(ArithmeticTarget::Register(Register::D)),
            0xAB => Self::Xor(ArithmeticTarget::Register(Register::E)),
            0xAC => Self::Xor(ArithmeticTarget::Register(Register::H)),
            0xAD => Self::Xor(ArithmeticTarget::Register(Register::L)),
            0xAE => Self::Xor(ArithmeticTarget::IndirectHl),
            0xAF => Self::Xor(ArithmeticTarget::Register(Register::A)),
            0xB0 => Self::Or(ArithmeticTarget::Register(Register::B)),
            0xB1 => Self::Or(ArithmeticTarget::Register(Register::C)),
            0xB2 => Self::Or(ArithmeticTarget::Register(Register::D)),
            0xB3 => Self::Or(ArithmeticTarget::Register(Register::E)),
            0xB4 => Self::Or(ArithmeticTarget::Register(Register::H)),
            0xB5 => Self::Or(ArithmeticTarget::Register(Register::L)),
            0xB6 => Self::Or(ArithmeticTarget::IndirectHl),
            0xB7 => Self::Or(ArithmeticTarget::Register(Register::A)),
            0xB8 => Self::Compare(ArithmeticTarget::Register(Register::B)),
            0xB9 => Self::Compare(ArithmeticTarget::Register(Register::C)),
            0xBA => Self::Compare(ArithmeticTarget::Register(Register::D)),
            0xBB => Self::Compare(ArithmeticTarget::Register(Register::E)),
            0xBC => Self::Compare(ArithmeticTarget::Register(Register::H)),
            0xBD => Self::Compare(ArithmeticTarget::Register(Register::L)),
            0xBE => Self::Compare(ArithmeticTarget::IndirectHl),
            0xBF => Self::Compare(ArithmeticTarget::Register(Register::A)),
            opcode => unimplemented!("0x{opcode:02X} isn't implemented yet!"),
        }
    }
}
