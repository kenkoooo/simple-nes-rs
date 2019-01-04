use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Cartridge {
    pub is_horizontal_mirror: bool,
    pub character_rom: Vec<u8>,
    pub program_rom: Vec<u8>,
    pub mapper: u8,
}

impl Cartridge {
    pub fn new(path: &str) -> io::Result<Self> {
        let mut f = File::open(path)?;
        let mut header = [0; 16];
        f.read_exact(&mut header)?;
        assert_eq!(&header[0..4], b"NES\x1A");

        let program_rom_size = header[4] as usize;
        let character_rom_size = header[5] as usize;

        let flag_6 = header[6];
        let is_horizontal_mirror = (flag_6 & 1) == 0;

        let flag_7 = header[7];
        let mapper = (flag_6 >> 4) | ((flag_7 >> 4) << 4);

        let mut program_rom = vec![0; 0x4000 * program_rom_size];
        let mut character_rom = vec![0; 0x2000 * character_rom_size];

        f.read_exact(&mut program_rom[..])?;
        f.read_exact(&mut character_rom[..])?;
        Ok(Cartridge {
            is_horizontal_mirror,
            program_rom,
            character_rom,
            mapper,
        })
    }
}
