// For now we allow dead code because I am lazy and don't want to see errors everywhere
#![allow(dead_code)]

use cpu::Cpu;
use ppu::Ppu;
use rom::Rom;

mod cpu;
mod ppu;
mod rom;

struct Emu {
    cpu: Cpu,
    rom: Rom,
    ppu: Ppu,
}
