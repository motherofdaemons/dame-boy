pub enum Instruction {
    Nop,
    Add(ArithmeticTarget),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl From<u8> for Instruction {
    fn from(opcode: u8) -> Self {
        match opcode {
            0x00 => Self::Nop,
            _ => unimplemented!(),
        }
    }
}
