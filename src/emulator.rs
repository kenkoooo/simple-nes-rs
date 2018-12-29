use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::system;
use sfml::window::{Event, Key, Style, VideoMode};
use time::{Duration, Tm};

use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::main_bus::MainBus;
use crate::mapper::Mapper;
use crate::picture_bus::PictureBus;
use crate::ppu;
use crate::ppu::PPU;
use crate::virtual_screen::VirtualScreen;

const NES_VIDEO_WIDTH: u32 = ppu::SCANLINE_VISIBLE_DOTS;
const NES_VIDEO_HEIGHT: u32 = ppu::VISIBLE_SCANLINES;
const SCREEN_SCALE: u32 = 2;

pub struct Emulator {
    pub cartridge: Rc<RefCell<Cartridge>>,
    pub picture_bus: Rc<RefCell<PictureBus>>,
    pub main_bus: Rc<RefCell<MainBus>>,
    pub cpu: CPU,
    pub ppu: PPU,
    pub window: Option<RenderWindow>,
    pub virtual_screen: Option<VirtualScreen>,
}

impl Emulator {
    pub fn run(&mut self, rom_path: &str) {
        let cpu_cycle_duration: Duration = Duration::nanoseconds(559);

        self.cartridge
            .borrow_mut()
            .load_from_file(rom_path)
            .unwrap();
        let mapper = Rc::new(RefCell::new(Mapper::new(
            self.cartridge.borrow().mapper_number,
            self.cartridge.clone(),
            self.picture_bus.clone(),
        )));
        self.main_bus.borrow_mut().set_mapper(mapper.clone());
        self.picture_bus.borrow_mut().set_mapper(mapper.clone());

        self.cpu.reset();

        let mut window = RenderWindow::new(
            VideoMode::new(
                NES_VIDEO_WIDTH * SCREEN_SCALE,
                NES_VIDEO_HEIGHT * SCREEN_SCALE,
                32,
            ),
            "simple_nes_rs",
            Style::TITLEBAR | Style::CLOSE,
            &Default::default(),
        );
        window.set_vertical_sync_enabled(true);

        let mut virtual_screen = VirtualScreen::new(
            NES_VIDEO_WIDTH,
            NES_VIDEO_HEIGHT,
            SCREEN_SCALE as f32,
            Color::WHITE,
        );

        let mut cycle_timer = time::now();
        let mut elapsed_time = cycle_timer - cycle_timer;

        let mut focus = true;
        let mut pause = false;
        while window.is_open() {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed => {
                        window.close();
                    }
                    Event::KeyPressed {
                        code,
                        alt: _,
                        ctrl: _,
                        shift: _,
                        system: _,
                    } => match code {
                        Key::Escape => {
                            window.close();
                        }
                        Key::F2 => {
                            pause = !pause;
                            if !pause {
                                cycle_timer = time::now();
                            }
                        }
                        _ => {}
                    },
                    Event::GainedFocus => {
                        focus = true;
                        cycle_timer = time::now();
                    }
                    Event::LostFocus => {
                        focus = false;
                    }
                    _ => {}
                }
            }

            if focus && !pause {
                elapsed_time = elapsed_time + (time::now() - cycle_timer);
                cycle_timer = time::now();

                while elapsed_time > cpu_cycle_duration {
                    self.ppu.step();
                    self.ppu.step();
                    self.ppu.step();

                    self.cpu.step();
                    elapsed_time = elapsed_time - cpu_cycle_duration;
                }

                window.draw(&virtual_screen);
                window.display();
            } else {
                system::sleep(system::Time::milliseconds(1000 / 60));
            }
        }
    }

    fn dma(&mut self, page: u8) {
        unimplemented!()
    }
}
