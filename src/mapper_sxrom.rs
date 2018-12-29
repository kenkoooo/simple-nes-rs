use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::mapper::NameTableMirroring;
use crate::picture_bus::PictureBus;

pub struct MapperSxROM {
    cartridge: Rc<RefCell<Cartridge>>,
    mirroring: NameTableMirroring,
    uses_character_ram: bool,
    character_ram: Vec<u8>,
    picture_bus: Rc<RefCell<PictureBus>>,

    first_bank_prg_idx: usize,
    second_bank_prg_idx: usize,
}

impl MapperSxROM {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>, picture_bus: Rc<RefCell<PictureBus>>) -> Self {
        let uses_character_ram = cartridge.borrow().chr_rom.is_empty();
        let mut character_ram = vec![];
        if uses_character_ram {
            character_ram.resize(0x2000, 0);
        }
        let second_bank_prg_idx = cartridge.borrow().prg_rom.len() - 0x4000;

        MapperSxROM {
            cartridge: cartridge,
            picture_bus: picture_bus,
            mirroring: NameTableMirroring::Horizontal,
            uses_character_ram: uses_character_ram,
            character_ram: character_ram,
            first_bank_prg_idx: 0,
            second_bank_prg_idx: second_bank_prg_idx,
        }
    }

    pub fn read_prg(&self, address: u16) -> u8 {
        let addr = address as usize;
        let idx = if address < 0xc000 {
            self.first_bank_prg_idx + (addr & 0x3fff)
        } else {
            self.second_bank_prg_idx + (addr & 0x3fff)
        };
        self.cartridge.borrow().prg_rom[idx]
    }
}
