use bitfield::bitfield;
use zerocopy::{transmute, AsBytes, FromBytes, FromZeroes};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8,
}

macro_rules! wide_register {
    ($hi:ident, $lo:ident, $getter:ident, $setter:ident) => {
        pub fn $getter(&self) -> u16 {
            let lo: u8 = transmute!(self.$lo);
            ((self.$hi as u16) << 8) | lo as u16
        }
        pub fn $setter(&mut self, value: u16) {
            self.$hi = ((value & 0xFF00) >> 8) as u8;
            self.$lo = transmute!((value & 0x00FF) as u8);
        }
    };
}

impl Registers {
    wide_register!(a, f, af, set_af);
    wide_register!(b, c, bc, set_bc);
    wide_register!(d, e, de, set_de);
    wide_register!(h, l, hl, set_hl);
}

bitfield! {
    #[derive(Clone, Copy, Default, PartialEq, Eq, AsBytes, FromBytes, FromZeroes)]
    #[repr(transparent)]
    pub struct Flags(u8);
    impl Debug;
    pub zero, set_zero: 7;
    pub subtract, set_subtract: 6;
    pub half_carry, set_half_carry: 5;
    pub carry, set_carry: 4;
}
