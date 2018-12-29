use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;

pub struct MapperCNROM {
    one_bank: bool,
}

impl MapperCNROM {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        let one_bank = cartridge.borrow().prg_rom.len() == 0x4000;
        MapperCNROM { one_bank: one_bank }
    }
}
