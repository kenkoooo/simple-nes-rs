use std::fs::File;
use std::io;
use std::io::prelude::*;

pub enum MapperType {
    NROM,
    SxROM,
    UxROM,
    CNROM,
}

impl MapperType {
    fn get(mapper_number: u8) -> Self {
        use self::MapperType::*;
        match mapper_number {
            0 => NROM,
            1 => SxROM,
            2 => UxROM,
            3 => CNROM,
            _ => unreachable!(),
        }
    }
}

pub struct Cartridge {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub name_table_mirroring: u8,
    pub mapper_type: MapperType,
    pub extended_ram: bool,
}

impl Cartridge {
    pub fn new(path: &str) -> io::Result<Self> {
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
        let name_table_mirroring = header[6] & 0x8;
        let mapper_number = ((header[6] >> 4) & 0xf) | (header[7] & 0xf0);
        let mapper_type = MapperType::get(mapper_number);
        let extended_ram = (header[6] & 0x2) != 0;
        assert!(header[6] & 0x4 == 0, "Trainer is not supported.");
        assert!(
            (header[0xA] & 0x3) != 0x2 && (header[0xA] & 0x1) == 0,
            "PAL ROM not supported."
        );
        let mut prg_rom = vec![0; 0x4000 * banks];
        f.read_exact(&mut prg_rom[..])?;

        let mut chr_rom = vec![];
        if v_banks != 0 {
            chr_rom.resize(0x2000 * v_banks, 0);
            f.read_exact(&mut chr_rom[..])?;
        }
        Ok(Cartridge {
            prg_rom,
            chr_rom,
            name_table_mirroring,
            mapper_type,
            extended_ram,
        })
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
                Cartridge::new(&path).expect(&path);
            });
    }
}
