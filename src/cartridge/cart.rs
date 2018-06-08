struct Cart {
    pub header: CartHeader,
    pub prg_rom: ProgramRom,
    pub chr_rom: CharacterRom,
}

impl Cart {
    pub fn new(bytes: impl AsRef<[u8]>) -> Self {

        // Grab bytes reference
        let bytes_ref = bytes.as_ref();

        // Extract header bytes
        let header_bytes = array_ref!(bytes_ref, 0, 16);

        // Transmute the fuck out of the header
        let header: CartHeader = unsafe {
            ::std::mem::transmute_copy(header_bytes)
        };

        println!("{:?}", header);

        // Catch nasty errors
        if header.prg_rom_banks == 0 {
            panic!("Invalid program ROM size!");
        }

        // Get sizes and stuff
        let prg_rom_size = header.prg_rom_banks as usize * 0x4000_usize;
        let chr_rom_size = header.chr_rom_banks as usize * 0x2000_usize;
        let prg_rom_end = 16_usize + prg_rom_size;

        // Grab raw bytes
        let prg_rom_bytes = &bytes_ref[16 .. prg_rom_end];
        let chr_rom_bytes = &bytes_ref[prg_rom_end .. chr_rom_size + prg_rom_end];

        // Waste some memory
        let prg_rom_vec = Vec::from(prg_rom_bytes);
        let chr_rom_vec = {
            if header.chr_rom_banks == 0 { vec![0; 0x2000] }
            else { Vec::from(chr_rom_bytes) }
        };

        // Initialize the cart
        Cart {
            header: header,
            prg_rom: ProgramRom(prg_rom_size, prg_rom_vec),
            chr_rom: CharacterRom(chr_rom_size, chr_rom_vec),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
struct CartHeader {
    /** b"NES\r" */
    nes: [u8; 4],
    prg_rom_banks: u8,
    chr_rom_banks: u8,
    flags_6: u8,
    flags_7: u8,
    prg_ram_banks: u8,
    flags_9: u8,
    flags_10: u8,
    /** Unused */
    zero: [u8; 5],
}

impl CartHeader {
    pub fn mapper(&self) -> u8 {
        (self.flags_6 & 0xF0) >> 4 | (self.flags_7 & 0xF0)
    }
}

struct ProgramRom(usize, Vec<u8>);

struct CharacterRom(usize, Vec<u8>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn detect_corrupt_program_rom() {
        let fake_header = b"NES\r\0\0\0\0\0\0\0\0\0\0\0\0";
        let expected = b"NES\r";
        let cart = Cart::new(fake_header);
    }

    #[test]
    fn transmute_works() {
        let mut fake_header: Vec<u8> = vec![b'N', b'E', b'S', 0x1A, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        fake_header.extend(vec![0u8; 0x4000]);
        let expected = b"NES\x1A";
        let cart = Cart::new(fake_header);
        assert_eq!(expected, &cart.header.nes);
    }
}