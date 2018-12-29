use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::main_bus::MainBus;
use crate::mapper::Mapper;
use crate::picture_bus::PictureBus;
use crate::ppu::PPU;

pub struct Emulator {
    cartridge: Rc<RefCell<Cartridge>>,
    picture_bus: Rc<RefCell<PictureBus>>,
    main_bus: Rc<RefCell<MainBus>>,
    cpu: CPU,
    ppu: PPU,
}

impl Emulator {
    pub fn run(&mut self, rom_path: &str) -> io::Result<()> {
        self.cartridge.borrow_mut().load_from_file(rom_path)?;
        let mapper = Rc::new(RefCell::new(Mapper::new(
            self.cartridge.borrow().mapper_number,
            self.cartridge.clone(),
            self.picture_bus.clone(),
        )));
        self.main_bus.borrow_mut().set_mapper(mapper.clone());
        self.picture_bus.borrow_mut().set_mapper(mapper.clone());

        self.cpu.reset();
        self.ppu.reset();

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
