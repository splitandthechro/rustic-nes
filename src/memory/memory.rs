use ::cartridge::ProgramRom;

pub struct Memory {
    pub bytes: [u8; 0x800],
    mapper: u8,
}

impl Memory {
    pub fn new(mapper: u8) -> Self {
        Memory {
            bytes: [0u8; 0x800],
            mapper: mapper,
        }
    }

    #[inline]
    pub fn read8(&self, addr: u16, prg_rom: &ProgramRom) -> u8 {
        match addr {
            0x0000...0x2000 => self.bytes[addr as usize % 0x800],
            0x8000...0xFFFF if self.mapper == 0 => prg_rom[addr as usize - 0x8000],
            0x8000...0xFFFF if self.mapper == 1 => prg_rom[addr as usize],
            0x8000...0xFFFF => panic!("Unimplemented mapper: {}", self.mapper),
            _ => panic!("Invalid address: 0x{:X}", addr),
        }
    }

    #[inline]
    pub fn write8(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000...0x2000 => self.bytes[addr as usize % 0x800] = val,
            0x4017 => (), // Ignore write to PPU or CPU register
            _ => panic!("Cannot write at address: 0x{:X}", addr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_correctly() {
        Memory::new(0);
    }

    #[test]
    fn reads_ram_without_error() {
        let mem = Memory::new(0);
        let prg_rom = ProgramRom::default();
        (0_u16..0x2000_u16).for_each(|i| {
            let b = mem.read8(i, &prg_rom);
            assert_eq!(0_u8, b);
        });
    }

    #[test]
    fn writes_ram_without_error() {
        let mut mem = Memory::new(0);
        let prg_rom = ProgramRom::default();
        (0_u16..0x2000_u16).for_each(|i| {
            let wb = (i % 255) as u8;
            mem.write8(i, wb);
            let rb = mem.read8(i, &prg_rom);
            assert_eq!(wb, rb);
        });
    }

    #[test]
    fn wraps_correctly() {
        let mut mem = Memory::new(0);
        let prg_rom = ProgramRom::default();
        (0x000_u16..0x0800_u16).for_each(|i| assert_eq!(0_u8, mem.read8(i, &prg_rom)));
        (0x800_u16..0x1000_u16).for_each(|i| mem.write8(i, 1_u8));
        (0x000_u16..0x0800_u16).for_each(|i| assert_eq!(1_u8, mem.read8(i, &prg_rom)));
    }
}