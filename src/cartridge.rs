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
        assert_eq!(
            String::from_utf8_lossy(&header[0..4]),
            "NES\x1A",
            "Invalid magic number"
        );

        let banks = header[4] as usize;
        assert!(banks != 0, "ROM has no PRG-ROM banks.");
        let v_banks = header[5] as usize;
        self.name_table_mirroring = header[6] & 0x8;
        self.mapper_number = ((header[6] >> 4) & 0xf) | (header[7] & 0xf0);
        self.extended_ram = (header[6] & 0x2) != 0;
        assert!(header[6] & 0x4 == 0, "Trainer is not supported.");
        assert!(
            (header[0xA] & 0x3) != 0x2 && (header[0xA] & 0x1) == 0,
            "PAL ROM not supported."
        );
        self.prg_rom.resize(0x4000 * banks, 0);
        f.read_exact(&mut self.prg_rom[..])?;

        if v_banks != 0 {
            self.chr_rom.resize(0x2000 * v_banks, 0);
            f.read_exact(&mut self.chr_rom[..])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn load_cartridge_test() {
        fs::read_dir("./roms/")
            .unwrap()
            .map(|f| f.unwrap().path().display().to_string())
            .for_each(|path| {
                let mut cartridge = Cartridge::new();
                cartridge.load_from_file(&path).expect(&path);
            });
    }
}
