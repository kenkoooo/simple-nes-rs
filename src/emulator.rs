use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use crate::cartridge::Cartridge;

pub struct Emulator {
    cartridge: Rc<RefCell<Cartridge>>,
}

impl Emulator {
    pub fn run(&mut self, rom_path: &str) -> io::Result<()> {
        self.cartridge.borrow_mut().load_from_file(rom_path)?;

        unimplemented!()
    }
    pub fn set_video_width(&mut self, width: usize) {
        unimplemented!()
    }
    pub fn set_video_height(&mut self, height: usize) {
        unimplemented!()
    }

    fn dma(&mut self, page: u8) {
        unimplemented!()
    }
}
