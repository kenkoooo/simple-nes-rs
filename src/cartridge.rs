use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Cartridge {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    name_table_mirroring: u8,
    mapper_number: u8,
    extended_ram: bool,
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge {
            prg_rom: vec![],
            chr_rom: vec![],
            name_table_mirroring: 0,
            mapper_number: 0,
            extended_ram: false,
        }
    }

    pub fn load_from_file(&mut self, path: &str) -> io::Result<()> {
        let mut f = File::open(path)?;

        let mut header = [0; 0x10];
        f.read_exact(&mut header)?;

        unimplemented!()
    }
}
