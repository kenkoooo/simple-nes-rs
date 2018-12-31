use crate::cartridge::{Cartridge, MapperType};

use std::cell::RefCell;
use std::rc::Rc;

type Address = u16;
type Byte = u8;

pub enum NameTableMirroring {
    Horizontal,
    Vertical,
    FourScreen,
    OneScreenLower,
    OneScreenHigher,
}

impl NameTableMirroring {
    fn get(id: u8) -> Self {
        use self::NameTableMirroring::*;
        match id {
            0 => Horizontal,
            1 => Vertical,
            8 => FourScreen,
            9 => OneScreenLower,
            10 => OneScreenHigher,
            _ => unreachable!(),
        }
    }
}

pub struct Mapper {
    mapper: MapperKind,
    mirroring: NameTableMirroring,
}

impl Mapper {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        let mapper = match cartridge.borrow().mapper_type {
            MapperType::CNROM => MapperKind::CNROM(MapperCNROM::new(cartridge.clone())),
            _ => unimplemented!(),
        };
        let mirroring = match cartridge.borrow().mapper_type {
            MapperType::SxROM => NameTableMirroring::Horizontal,
            _ => NameTableMirroring::get(cartridge.borrow().name_table_mirroring),
        };
        Mapper { mapper, mirroring }
    }
}

enum MapperKind {
    CNROM(MapperCNROM),
    UxROM,
    SxROM,
    NROM,
}

pub trait MapperTrait {
    fn read_prg(&self, addr: Address) -> Byte;
    fn write_prg(&mut self, addr: Address, value: Byte);
    fn read_chr(&self, addr: Address) -> Byte;
    fn write_chr(&mut self, addr: Address, value: Byte);
}

struct MapperCNROM {
    one_bank: bool,
    select_chr: Address,
    cartridge: Rc<RefCell<Cartridge>>,
}

impl MapperCNROM {
    fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        let one_bank = cartridge.borrow().prg_rom.len() == 0x4000;
        MapperCNROM {
            one_bank,
            select_chr: 0,
            cartridge,
        }
    }
}

impl MapperTrait for MapperCNROM {
    fn read_prg(&self, addr: Address) -> Byte {
        let addr = addr as usize;
        if !self.one_bank {
            self.cartridge.borrow().prg_rom[addr - 0x8000]
        } else {
            self.cartridge.borrow().prg_rom[(addr - 0x8000) & 0x3fff]
        }
    }

    fn write_prg(&mut self, addr: Address, value: Byte) {
        self.select_chr = (value & 0x3) as Address;
    }

    fn read_chr(&self, addr: Address) -> Byte {
        let address = (addr | (self.select_chr << 13)) as usize;
        self.cartridge.borrow().prg_rom[address]
    }
    fn write_chr(&mut self, addr: Address, _: Byte) {
        eprintln!("Read-only CHR memory write attempt at {}", addr);
    }
}

struct MapperNROM {
    one_bank: bool,
    uses_character_ram: bool,
    character_ram: Vec<Byte>,
}

impl MapperNROM {
    fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        let one_bank = cartridge.borrow().prg_rom.len() == 0x4000;
        let uses_character_ram = cartridge.borrow().chr_rom.is_empty();
        let mut character_ram = vec![];
        if uses_character_ram {
            character_ram.resize(0x2000, 0);
        }
        MapperNROM {
            one_bank,
            uses_character_ram,
            character_ram,
        }
    }
}

impl MapperTrait for MapperNROM {
    fn read_prg(&self, addr: Address) -> Byte {
        unimplemented!()
    }
    fn write_prg(&mut self, addr: Address, value: Byte) {
        unimplemented!()
    }
    fn read_chr(&self, addr: Address) -> Byte {
        unimplemented!()
    }
    fn write_chr(&mut self, addr: Address, value: Byte) {
        unimplemented!()
    }
}
