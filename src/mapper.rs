use crate::cartridge::{Cartridge, MapperType};

use std::cell::RefCell;
use std::rc::Rc;

type Address = u16;
type Byte = u8;

#[derive(Clone, Copy)]
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

pub struct Mapper<F> {
    mapper: MapperKind<F>,
}

impl<F> Mapper<F>
where
    F: FnMut() -> (),
{
    pub fn new(cartridge: Rc<RefCell<Cartridge>>, callback: F) -> Self {
        let mapper = match cartridge.borrow().mapper_type {
            MapperType::CNROM => MapperKind::CNROM(MapperCNROM::new(cartridge.clone())),
            MapperType::NROM => MapperKind::NROM(MapperNROM::new(cartridge.clone())),
            MapperType::UxROM => MapperKind::UxROM(MapperUxROM::new(cartridge.clone())),
            MapperType::SxROM => MapperKind::SxROM(MapperSxROM::new(cartridge.clone(), callback)),
        };
        Mapper { mapper }
    }
}

enum MapperKind<F> {
    CNROM(MapperCNROM),
    UxROM(MapperUxROM),
    SxROM(MapperSxROM<F>),
    NROM(MapperNROM),
}

pub trait MapperTrait {
    fn read_prg(&self, addr: Address) -> Byte;
    fn write_prg(&mut self, addr: Address, value: Byte);
    fn read_chr(&self, addr: Address) -> Byte;
    fn write_chr(&mut self, addr: Address, value: Byte);
    fn get_mirroring(&self) -> NameTableMirroring;
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

    fn get_mirroring(&self) -> NameTableMirroring {
        NameTableMirroring::get(self.cartridge.borrow().name_table_mirroring)
    }
}

struct MapperNROM {
    one_bank: bool,
    uses_character_ram: bool,
    character_ram: Vec<Byte>,
    cartridge: Rc<RefCell<Cartridge>>,
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
            cartridge,
        }
    }
}

impl MapperTrait for MapperNROM {
    fn read_prg(&self, addr: Address) -> Byte {
        let addr = addr as usize;
        if !self.one_bank {
            self.cartridge.borrow().prg_rom[addr - 0x8000]
        } else {
            self.cartridge.borrow().prg_rom[(addr - 0x8000) & 0x3fff]
        }
    }
    fn write_prg(&mut self, addr: Address, value: Byte) {
        eprintln!("ROM memory write attempt at {}", addr);
    }
    fn read_chr(&self, addr: Address) -> Byte {
        let addr = addr as usize;
        if self.uses_character_ram {
            self.character_ram[addr]
        } else {
            self.cartridge.borrow().chr_rom[addr]
        }
    }
    fn write_chr(&mut self, addr: Address, value: Byte) {
        if self.uses_character_ram {
            self.character_ram[addr as usize] = value;
        } else {
            eprintln!("Read-only CHR memory write attempt at {}", addr);
        }
    }

    fn get_mirroring(&self) -> NameTableMirroring {
        NameTableMirroring::get(self.cartridge.borrow().name_table_mirroring)
    }
}

struct MapperSxROM<F> {
    mirroring: NameTableMirroring,
    callback: F,

    uses_character_ram: bool,
    mode_chr: i32,
    mode_prg: i32,
    temp_register: Byte,
    write_counter: usize,

    reg_prg: Byte,
    reg_chr0: Byte,
    reg_chr1: Byte,

    first_bank_prg: usize,
    second_bank_prg: usize,

    first_bank_chr: usize,
    second_bank_chr: usize,

    character_ram: Vec<Byte>,
    cartridge: Rc<RefCell<Cartridge>>,
}

impl<F> MapperSxROM<F>
where
    F: FnMut() -> (),
{
    fn new(cartridge: Rc<RefCell<Cartridge>>, f: F) -> Self {
        let uses_character_ram = cartridge.borrow().chr_rom.is_empty();
        let mut character_ram = vec![];
        if uses_character_ram {
            character_ram.resize(0x2000, 0);
        }

        let first_bank_prg = 0;
        let second_bank_prg = cartridge.borrow().prg_rom.len() - 0x4000;

        MapperSxROM {
            mirroring: NameTableMirroring::Horizontal,
            callback: f,
            uses_character_ram,
            mode_chr: 0,
            mode_prg: 3,
            temp_register: 0,
            write_counter: 0,
            reg_prg: 0,
            reg_chr0: 0,
            reg_chr1: 0,
            first_bank_prg,
            second_bank_prg,
            first_bank_chr: 0,
            second_bank_chr: 0,
            character_ram,
            cartridge,
        }
    }
    fn calculate_prg_pointers(&mut self) {
        if self.mode_prg <= 1 {
            // 32KB changeable
            // equivalent to multiplying 0x8000 * (m_regPRG >> 1)
            self.first_bank_prg = 0x4000 * ((self.reg_prg as usize) & !1);
            self.second_bank_prg = self.first_bank_prg + 0x4000; // add 16KB
        } else if self.mode_prg == 2 {
            // fix first switch second
            self.first_bank_prg = 0;
            self.second_bank_prg = self.first_bank_prg + 0x4000 * (self.reg_prg as usize);
        } else {
            // switch first fix second
            self.first_bank_prg = 0x4000 * self.reg_prg as usize;
            self.second_bank_prg = self.cartridge.borrow().prg_rom.len() - 0x4000;
        }
    }
}

impl<F> MapperTrait for MapperSxROM<F>
where
    F: FnMut() -> (),
{
    fn read_prg(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        if addr < 0xc000 {
            self.cartridge.borrow().prg_rom[self.first_bank_prg + (addr & 0x3fff)]
        } else {
            self.cartridge.borrow().prg_rom[self.second_bank_prg + (addr & 0x3fff)]
        }
    }

    fn write_prg(&mut self, addr: u16, value: u8) {
        let reset_bit = value & 0x80;
        if reset_bit == 0 {
            self.temp_register = (self.temp_register >> 1) | ((value & 1) << 4);
            self.write_counter += 1;
            if self.write_counter == 5 {
                if addr <= 0x9fff {
                    match self.temp_register & 0x3 {
                        0 => self.mirroring = NameTableMirroring::OneScreenLower,
                        1 => self.mirroring = NameTableMirroring::OneScreenHigher,
                        2 => self.mirroring = NameTableMirroring::Vertical,
                        3 => self.mirroring = NameTableMirroring::Horizontal,
                        _ => unreachable!(),
                    }
                    (self.callback)();

                    self.mode_chr = ((self.temp_register as i32) & 0x10) >> 4;
                    self.mode_prg = ((self.temp_register as i32) & 0xc) >> 2;
                    self.calculate_prg_pointers();

                    // Recalculate CHR pointers
                    if self.mode_chr == 0 {
                        // one 8KB bank
                        self.first_bank_chr = 0x1000 * (self.reg_chr0 as usize | 1); // ignore last bit
                        self.second_bank_chr = self.first_bank_chr + 0x1000;
                    } else {
                        // two 4KB banks
                        self.first_bank_chr = 0x1000 * self.reg_chr0 as usize;
                        self.second_bank_chr = 0x1000 * self.reg_chr1 as usize;
                    }
                } else if addr <= 0xbfff {
                    // CHR Reg 0
                    self.reg_chr0 = self.temp_register;
                    self.first_bank_chr =
                        0x1000 * ((self.temp_register as i32) | (1 - self.mode_chr)) as usize; // OR 1 if 8KB mode
                    if self.mode_chr == 0 {
                        self.second_bank_chr = self.first_bank_chr + 0x1000;
                    }
                } else if addr <= 0xdfff {
                    self.reg_chr1 = self.temp_register;
                    if self.mode_chr == 1 {
                        self.second_bank_chr = 0x1000 * (self.temp_register as usize);
                    }
                } else {
                    if self.temp_register & 0x10 == 0x10 {
                        eprintln!("PRG-RAM activated");
                    }

                    self.temp_register &= 0xf;
                    self.reg_prg = self.temp_register;
                    self.calculate_prg_pointers();
                }

                self.temp_register += 1;
                self.write_counter += 1;
            }
        } else {
            // reset
            self.temp_register = 0;
            self.write_counter = 0;
            self.mode_prg = 3;
            self.calculate_prg_pointers();
        }
    }

    fn read_chr(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        if self.uses_character_ram {
            return self.character_ram[addr];
        } else if addr < 0x1000 {
            return self.cartridge.borrow().chr_rom[self.first_bank_chr + addr];
        } else {
            return self.cartridge.borrow().chr_rom[self.second_bank_chr + (addr & 0xfff)];
        }
    }

    fn write_chr(&mut self, addr: u16, value: u8) {
        if self.uses_character_ram {
            self.character_ram[addr as usize] = value;
        } else {
            eprintln!("Read-only CHR memory write attempt at {}", addr);
        }
    }

    fn get_mirroring(&self) -> NameTableMirroring {
        self.mirroring
    }
}

struct MapperUxROM {
    cartridge: Rc<RefCell<Cartridge>>,
    uses_character_ram: bool,
    select_prg: Address,
    character_ram: Vec<Byte>,
    last_bank_ptr: usize,
}

impl MapperUxROM {
    fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        let uses_character_ram = cartridge.borrow().chr_rom.is_empty();
        let mut character_ram = vec![];
        if uses_character_ram {
            character_ram.resize(0x2000, 0);
        }
        let last_bank_ptr = cartridge.borrow().prg_rom.len() - 0x4000;
        MapperUxROM {
            cartridge,
            uses_character_ram,
            character_ram,
            select_prg: 0,
            last_bank_ptr,
        }
    }
}

impl MapperTrait for MapperUxROM {
    fn read_prg(&self, addr: u16) -> u8 {
        if addr < 0xc000 {
            let addr = ((addr - 0x8000) & 0x3fff) | (self.select_prg << 14);
            self.cartridge.borrow().prg_rom[addr as usize]
        } else {
            self.cartridge.borrow().prg_rom[self.last_bank_ptr + ((addr as usize) & 0x3fff)]
        }
    }

    fn write_prg(&mut self, addr: u16, value: u8) {
        self.select_prg = value as u16;
    }

    fn read_chr(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        if self.uses_character_ram {
            self.character_ram[addr]
        } else {
            self.cartridge.borrow().chr_rom[addr]
        }
    }

    fn write_chr(&mut self, addr: u16, value: u8) {
        if self.uses_character_ram {
            self.character_ram[addr as usize] = value;
        } else {
            eprintln!("Read-only CHR memory write attempt at {}", addr);
        }
    }

    fn get_mirroring(&self) -> NameTableMirroring {
        NameTableMirroring::get(self.cartridge.borrow().name_table_mirroring)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_mapper() {
        fs::read_dir("./roms/")
            .unwrap()
            .map(|f| f.unwrap().path().display().to_string())
            .for_each(|path| {
                let cartridge = Cartridge::new(&path).expect(&path);
                let cartridge = Rc::new(RefCell::new(cartridge));
                let mut x = 0;
                Mapper::new(cartridge, || {
                    x += 1;
                });
            });
    }
}
