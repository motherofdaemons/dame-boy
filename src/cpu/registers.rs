#[derive(Debug, Default, PartialEq, Eq)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

macro_rules! combined_regs {
    ($reg1:ident, $reg2:ident, $get:ident, $set:ident) => {
        pub fn $get(&self) -> u16 {
            ((self.$reg1 as u16) << 8) | self.$reg2 as u16
        }

        pub fn $set(&mut self, value: u16) {
            self.$reg1 = ((value & 0xFF00) >> 8) as u8;
            self.$reg2 = (value & 0x00FF) as u8;
        }
    };
}

macro_rules! reg {
    ($reg:ident, $get:ident, $set:ident) => {
        pub fn $get(&self) -> u8 {
            self.$reg
        }

        pub fn $set(&mut self, value: u8) {
            self.$reg = value;
        }
    };
}

impl Registers {
    combined_regs!(a, f, af, set_af);
    combined_regs!(b, c, bc, set_bc);
    combined_regs!(d, e, de, set_de);
    combined_regs!(h, l, hl, set_hl);

    pub fn flags(&self) -> Flags {
        self.f.into()
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.f = flags.into()
    }
}

#[derive(Debug, PartialEq)]
pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl From<Flags> for u8 {
    fn from(flags: Flags) -> Self {
        (flags.zero as u8) << ZERO_FLAG_BYTE_POSITION
            | (flags.subtract as u8) << SUBTRACT_FLAG_BYTE_POSITION
            | (flags.half_carry as u8) << HALF_CARRY_FLAG_BYTE_POSITION
            | (flags.carry as u8) << CARRY_FLAG_BYTE_POSITION
    }
}

impl From<u8> for Flags {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        Self {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}
