use std::cell::RefCell;
use std::rc::Rc;
use crate::mapper::Mapper;

struct PictureBus<F> {
    ram: Vec<u8>,
    name_table0: usize,
    name_table1: usize,
    name_table2: usize,
    palette: Vec<u8>,
    mapper: Rc<RefCell<Mapper<F>>>,
}

impl<F> PictureBus<F> {
    fn new(mapper: Rc<RefCell<Mapper<F>>>) -> Self {
        unimplemented!()
    }

    fn read(&self, address: u16) -> u8 { unimplemented!() }
    fn write(&mut self, address: u16) { unimplemented!() }
    fn read_palette(&self, palette_address: u8) -> u8 { unimplemented!() }
    fn update_mirroring(&mut self) {}
}