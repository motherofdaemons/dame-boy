use std::path::Path;

use dame_boy::Emu;

fn main() -> ! {
    let boot_rom_file = Path::new("./roms/dmg_rom.bin");
    let mut emu = Emu::new(boot_rom_file);
    emu.run()
}
