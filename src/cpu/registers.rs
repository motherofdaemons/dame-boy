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

impl Registers {
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f.0 as u16
    }
    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = transmute!((value & 0x00FF) as u8);
    }
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
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
