use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Cartridge {}

impl Cartridge {
    fn new(path: &str) -> io::Result<Self> {
        let mut f = File::open(path)?;
        let mut header = [0; 16];
        f.read_exact(&mut header)?;
        assert_eq!(&header[0..4], b"NES\x1A");

        let prg_rom_size = header[4] as usize;
        let chr_rom_size = header[5] as usize;
        let flags_6 = header[6];

        let mirroring = (flags_6 & 1) != 0;
        let battery = (flags_6 & (1 << 1)) != 0;
        let trainer = (flags_6 & (1 << 2)) != 0;

        let flags_7 = header[7];
        let flags_8 = header[8];
        let flags_9 = header[9];
        let flags_10 = header[10];

        unimplemented!()
    }
}
