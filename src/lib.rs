// For now we allow dead code because I am lazy and don't want to see errors everywhere
#![allow(dead_code)]

use std::{fs::read, path::Path};

use cpu::{instructions::Instruction, Cpu};
use mem::Mem;
use ppu::Ppu;

mod cpu;
mod mem;
mod ppu;

#[derive(Default)]
pub struct Emu {
    cpu: Cpu,
    mem: Mem,
    ppu: Ppu,
}

impl Emu {
    pub fn new(boot_rom_file: &Path) -> Self {
        let boot = read(boot_rom_file).expect("We must have the boot rom to boot");
        Self {
            mem: Mem::new(boot),
            ..Default::default()
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            let instruction = self.fetch_instruction();
            self.cpu.execute(instruction);
        }
    }

    fn fetch_instruction(&self) -> Instruction {
        // TODO: handle extended opcode table
        let addr = self.cpu.sp();
        let opcode = self.mem.read(addr.into());
        opcode.into()
    }
}
