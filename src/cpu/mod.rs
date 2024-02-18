use self::{
    instructions::{ArithmeticTarget, Instruction},
    registers::{Flags, Registers},
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
        if let Some(flags) = match instruction {
            Instruction::Nop => self.nop(),
            Instruction::Add(target) => self.add(target),
        } {
            self.registers.set_flags(flags);
        }
    }

    fn nop(&self) -> Option<Flags> {
        None
    }

    /// Take the value from `target` register and add it to A
    fn add(&mut self, target: ArithmeticTarget) -> Option<Flags> {
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
        // check to see if we carried at the nibble
        Some(Flags {
            zero: result == 0,
            subtract: false,
            half_carry: (result & 0x10) == 0x10,
            carry,
        })
    }
}

#[cfg(test)]
mod tests {
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
                    a: 0,
                    f: Flags {
                        zero: true,
                        subtract: false,
                        half_carry: false,
                        carry: false,
                    }
                    .into(),
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
                    f: Flags {
                        zero: false,
                        subtract: false,
                        half_carry: true,
                        carry: false,
                    }
                    .into(),
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
                f: Flags {
                    zero: true,
                    subtract: false,
                    half_carry: false,
                    carry: true,
                }
                .into(),
                ..cpu.registers
            },
            ..cpu
        };

        assert_eq!(cpu, expected);
    }
}
