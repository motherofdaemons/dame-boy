use self::{
    instructions::{ArithmeticTarget, Instruction},
    registers::Registers,
};

mod instructions;
mod registers;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Cpu {
    registers: Registers,
    sp: u16,
}

impl Cpu {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Nop => self.nop(),
            Instruction::Add(target) => self.add(target),
        }
    }

    fn nop(&self) {}

    /// Take the value from `target` register and add it to A
    fn add(&mut self, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
        };

        let (result, carry) = self.registers.a.overflowing_add(value);
        self.registers.a = result;
        self.registers.f.set_zero(result == 0);
        self.registers.f.set_subtract(false);
        // check to see if we carried at the nibble
        self.registers.f.set_half_carry((result & 0x10) == 0x10);
        self.registers.f.set_carry(carry);
    }
}

#[cfg(test)]
mod tests {
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

        let expected_states = [
            Cpu {
                registers: Registers {
                    f: Flags(0b1000_0000),
                    ..cpu.registers
                },
                ..cpu
            },
            Cpu {
                registers: Registers {
                    a: 1,
                    ..cpu.registers
                },
                ..cpu
            },
            Cpu {
                registers: Registers {
                    a: 3,
                    ..cpu.registers
                },
                ..cpu
            },
            Cpu {
                registers: Registers {
                    a: 6,
                    ..cpu.registers
                },
                ..cpu
            },
            Cpu {
                registers: Registers {
                    a: 10,
                    ..cpu.registers
                },
                ..cpu
            },
            Cpu {
                registers: Registers {
                    a: 15,
                    ..cpu.registers
                },
                ..cpu
            },
            Cpu {
                registers: Registers {
                    a: 21,
                    f: Flags(0b0010_0000),
                    ..cpu.registers
                },
                ..cpu
            },
        ];

        let targets = [
            ArithmeticTarget::A,
            ArithmeticTarget::B,
            ArithmeticTarget::C,
            ArithmeticTarget::D,
            ArithmeticTarget::E,
            ArithmeticTarget::H,
            ArithmeticTarget::L,
        ];

        for (target, expected) in targets.into_iter().zip(expected_states.into_iter()) {
            cpu.execute(Instruction::Add(target));
            assert_eq!(cpu, expected);
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

        cpu.execute(Instruction::Add(ArithmeticTarget::B));
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
