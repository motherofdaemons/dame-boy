use instructions::{LoadTarget, Register};

use self::{
    instructions::{ArithmeticTarget, Instruction},
    registers::Registers,
};

pub mod instructions;
mod registers;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Cpu {
    registers: Registers,
    sp: u16,
}

impl Cpu {
    pub fn sp(&self) -> u16 {
        self.sp
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Nop => self.nop(),
            Instruction::Add(target) => self.add(target, false),
            Instruction::AddCarry(target) => self.add(target, true),
            Instruction::Sub(target) => self.sub(target, false),
            Instruction::SubCarry(target) => self.sub(target, true),
            Instruction::And(target) => self.and(target),
            Instruction::Xor(target) => self.xor(target),
            Instruction::Or(target) => self.or(target),
            Instruction::Compare(target) => self.compare(target),
            Instruction::Load { dst, src } => self.load(dst, src),
        }
    }

    fn nop(&self) {}

    /// Take the value from `target` register and add it to A.
    ///
    /// - `carry` will use the carrybit in the addition.
    fn add(&mut self, target: ArithmeticTarget, carry: bool) {
        let value = match target {
            ArithmeticTarget::Register(Register::A) => self.registers.a,
            ArithmeticTarget::Register(Register::B) => self.registers.b,
            ArithmeticTarget::Register(Register::C) => self.registers.c,
            ArithmeticTarget::Register(Register::D) => self.registers.d,
            ArithmeticTarget::Register(Register::E) => self.registers.e,
            ArithmeticTarget::Register(Register::H) => self.registers.h,
            ArithmeticTarget::Register(Register::L) => self.registers.l,
            ArithmeticTarget::IndirectHl => {
                todo!("Need to figured out how I want to structure rom")
            }
        };

        let carry = if carry {
            self.registers.f.carry().into()
        } else {
            0
        };

        let (result, carry) = self.registers.a.overflowing_add(value + carry);
        self.registers.a = result;
        self.registers.f.set_zero(result == 0);
        self.registers.f.set_subtract(false);
        // check to see if we carried at the nibble
        self.registers.f.set_half_carry((result & 0x10) == 0x10);
        self.registers.f.set_carry(carry);
    }

    /// Take the value from `target` register and sub it to from A.
    ///
    /// - `carry` will use the carrybit in the subtraction.
    fn sub(&mut self, target: ArithmeticTarget, carry: bool) {
        let value = match target {
            ArithmeticTarget::Register(Register::A) => self.registers.a,
            ArithmeticTarget::Register(Register::B) => self.registers.b,
            ArithmeticTarget::Register(Register::C) => self.registers.c,
            ArithmeticTarget::Register(Register::D) => self.registers.d,
            ArithmeticTarget::Register(Register::E) => self.registers.e,
            ArithmeticTarget::Register(Register::H) => self.registers.h,
            ArithmeticTarget::Register(Register::L) => self.registers.l,
            ArithmeticTarget::IndirectHl => {
                todo!("Need to figured out how I want to structure rom")
            }
        };

        let carry = if carry {
            self.registers.f.carry().into()
        } else {
            0
        };

        let (result, carry) = self.registers.a.overflowing_sub(value + carry);
        self.registers.a = result;
        self.registers.f.set_zero(result == 0);
        self.registers.f.set_subtract(false);
        self.registers
            .f
            .set_half_carry(check_for_half_carry(result));
        self.registers.f.set_carry(carry);
    }

    fn and(&mut self, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::Register(Register::A) => self.registers.a,
            ArithmeticTarget::Register(Register::B) => self.registers.b,
            ArithmeticTarget::Register(Register::C) => self.registers.c,
            ArithmeticTarget::Register(Register::D) => self.registers.d,
            ArithmeticTarget::Register(Register::E) => self.registers.e,
            ArithmeticTarget::Register(Register::H) => self.registers.h,
            ArithmeticTarget::Register(Register::L) => self.registers.l,
            ArithmeticTarget::IndirectHl => {
                todo!("Need to figured out how I want to structure rom")
            }
        };

        self.registers.a &= value;

        self.registers.f.set_zero(self.registers.a == 0);
        self.registers.f.set_subtract(false);
        self.registers.f.set_half_carry(true);
        self.registers.f.set_carry(false);
    }

    fn xor(&mut self, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::Register(Register::A) => self.registers.a,
            ArithmeticTarget::Register(Register::B) => self.registers.b,
            ArithmeticTarget::Register(Register::C) => self.registers.c,
            ArithmeticTarget::Register(Register::D) => self.registers.d,
            ArithmeticTarget::Register(Register::E) => self.registers.e,
            ArithmeticTarget::Register(Register::H) => self.registers.h,
            ArithmeticTarget::Register(Register::L) => self.registers.l,
            ArithmeticTarget::IndirectHl => {
                todo!("Need to figured out how I want to structure rom")
            }
        };

        self.registers.a ^= value;

        self.registers.f.set_zero(self.registers.a == 0);
        self.registers.f.set_subtract(false);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_carry(false);
    }

    fn or(&mut self, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::Register(Register::A) => self.registers.a,
            ArithmeticTarget::Register(Register::B) => self.registers.b,
            ArithmeticTarget::Register(Register::C) => self.registers.c,
            ArithmeticTarget::Register(Register::D) => self.registers.d,
            ArithmeticTarget::Register(Register::E) => self.registers.e,
            ArithmeticTarget::Register(Register::H) => self.registers.h,
            ArithmeticTarget::Register(Register::L) => self.registers.l,
            ArithmeticTarget::IndirectHl => {
                todo!("Need to figured out how I want to structure rom")
            }
        };

        self.registers.a |= value;

        self.registers.f.set_zero(self.registers.a == 0);
        self.registers.f.set_subtract(false);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_carry(false);
    }

    fn compare(&mut self, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::Register(Register::A) => self.registers.a,
            ArithmeticTarget::Register(Register::B) => self.registers.b,
            ArithmeticTarget::Register(Register::C) => self.registers.c,
            ArithmeticTarget::Register(Register::D) => self.registers.d,
            ArithmeticTarget::Register(Register::E) => self.registers.e,
            ArithmeticTarget::Register(Register::H) => self.registers.h,
            ArithmeticTarget::Register(Register::L) => self.registers.l,
            ArithmeticTarget::IndirectHl => {
                todo!("Need to figured out how I want to structure rom")
            }
        };

        let (result, carry) = self.registers.a.overflowing_sub(value);

        self.registers.f.set_zero(result == 0);
        self.registers.f.set_subtract(false);
        // check to see if we carried at the nibble
        self.registers
            .f
            .set_half_carry(check_for_half_carry(result));
        self.registers.f.set_carry(carry);
    }

    fn load(&mut self, dst: LoadTarget, src: LoadTarget) {
        todo!()
    }
}

/// Check to see if we carried at the nibble
fn check_for_half_carry(value: u8) -> bool {
    (value & 0x10) == 0x10
}

#[cfg(test)]
mod tests {
    use std::array::from_fn;

    use self::registers::Flags;

    use super::*;

    #[test]
    fn add() {
        let mut cpu = Cpu {
            registers: Registers {
                b: 1,
                c: 2,
                d: 3,
                e: 4,
                h: 5,
                l: 6,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut a = 0;
        let expected_states: [Cpu; 7] = from_fn(|i| {
            a += i as u8;
            let mut f = Flags::default();
            if a > 0b1111 {
                f.set_half_carry(true);
            } else if a == 0 {
                f.set_zero(true);
            }
            Cpu {
                registers: Registers {
                    a,
                    f,
                    ..cpu.registers
                },
                ..cpu
            }
        });

        let targets = [
            ArithmeticTarget::Register(Register::A),
            ArithmeticTarget::Register(Register::B),
            ArithmeticTarget::Register(Register::C),
            ArithmeticTarget::Register(Register::D),
            ArithmeticTarget::Register(Register::E),
            ArithmeticTarget::Register(Register::H),
            ArithmeticTarget::Register(Register::L),
        ];

        for (target, expected) in targets.into_iter().zip(expected_states.into_iter()) {
            cpu.execute(Instruction::Add(target));
            assert_eq!(cpu, expected, "Failed to add {:?}", target);
        }
    }

    #[test]
    fn add_overflow() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 255,
                b: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        cpu.execute(Instruction::Add(ArithmeticTarget::Register(Register::B)));
        let expected = Cpu {
            registers: Registers {
                a: 0,
                f: Flags(0b1001_0000),
                ..cpu.registers
            },
            ..cpu
        };

        assert_eq!(cpu, expected);
    }
}
