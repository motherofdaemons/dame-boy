#[derive(Default)]
pub struct Mem {
    rom: Rom,
    ram: Ram,
}

#[derive(Default)]
struct Ram {}

#[derive(Default)]
struct Rom {
    boot: Vec<u8>,
    cart: Vec<u8>,
}

impl Mem {
    pub fn new(boot: Vec<u8>) -> Self {
        Self {
            rom: Rom::new(boot),
            ..Default::default()
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        // TODO: check for where to read from either boot rom or cart.
        self.rom.boot[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        // TODO: check for where to read from either boot rom or cart.
        self.rom.boot[addr as usize] = value;
    }
}

impl Rom {
    pub fn new(boot: Vec<u8>) -> Self {
        Self {
            boot,
            ..Default::default()
        }
    }
}
