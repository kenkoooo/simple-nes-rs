use std::cell::RefCell;
use std::rc::Rc;

use sfml::graphics::Color;

use crate::picture_bus::PictureBus;

type Byte = u8;
type Address = u16;

const SCANLINE_CYCLE_LENGTH: u32 = 341;
const SCANLINE_END_CYCLE: u32 = 340;
pub const VISIBLE_SCANLINES: u32 = 240;
pub const SCANLINE_VISIBLE_DOTS: u32 = 256;
const FRAME_END_SCANLINE: u32 = 261;
const COLORS: [u32; 64] = [
    0x666666ff, 0x002a88ff, 0x1412a7ff, 0x3b00a4ff, 0x5c007eff, 0x6e0040ff, 0x6c0600ff, 0x561d00ff,
    0x333500ff, 0x0b4800ff, 0x005200ff, 0x004f08ff, 0x00404dff, 0x000000ff, 0x000000ff, 0x000000ff,
    0xadadadff, 0x155fd9ff, 0x4240ffff, 0x7527feff, 0xa01accff, 0xb71e7bff, 0xb53120ff, 0x994e00ff,
    0x6b6d00ff, 0x388700ff, 0x0c9300ff, 0x008f32ff, 0x007c8dff, 0x000000ff, 0x000000ff, 0x000000ff,
    0xfffeffff, 0x64b0ffff, 0x9290ffff, 0xc676ffff, 0xf36affff, 0xfe6eccff, 0xfe8170ff, 0xea9e22ff,
    0xbcbe00ff, 0x88d800ff, 0x5ce430ff, 0x45e082ff, 0x48cddeff, 0x4f4f4fff, 0x000000ff, 0x000000ff,
    0xfffeffff, 0xc0dfffff, 0xd3d2ffff, 0xe8c8ffff, 0xfbc2ffff, 0xfec4eaff, 0xfeccc5ff, 0xf7d8a5ff,
    0xe4e594ff, 0xcfef96ff, 0xbdf4abff, 0xb3f3ccff, 0xb5ebf2ff, 0xb8b8b8ff, 0x000000ff, 0x000000ff,
];

const AttributeOffset: u32 = 0x3C0;

enum State {
    PreRender,
    Render,
    PostRender,
    VerticalBlank,
}

#[derive(PartialEq)]
enum CharacterPage {
    Low,
    High,
}

impl CharacterPage {
    fn value(&self) -> usize {
        match self {
            CharacterPage::Low => 0,
            CharacterPage::High => 1,
        }
    }
}

pub struct PPU {
    picture_bus: Rc<RefCell<PictureBus>>,
    m_spriteMemory: [u8; 64 * 4],
    m_scanlineSprites: Vec<Byte>,

    m_pipelineState: State,

    m_cycle: u32,
    m_scanline: u32,
    m_evenFrame: bool,

    m_vblank: bool,
    m_sprZeroHit: bool,

    // Registers
    m_dataAddress: Address,
    m_tempAddress: Address,
    m_fineXScroll: Byte,
    m_firstWrite: bool,
    m_dataBuffer: Byte,

    m_spriteDataAddress: Byte,

    // Setup flags and variables
    m_longSprites: bool,
    m_generateInterrupt: bool,

    m_greyscaleMode: bool,
    m_showSprites: bool,
    m_showBackground: bool,
    m_hideEdgeSprites: bool,
    m_hideEdgeBackground: bool,

    m_bgPage: CharacterPage,
    m_sprPage: CharacterPage,

    m_dataAddrIncrement: Address,
    m_pictureBuffer: Vec<Vec<Color>>,
}
impl PPU {
    pub fn new(picture_bus: Rc<RefCell<PictureBus>>) -> Self {
        PPU {
            picture_bus: picture_bus,
            m_spriteMemory: [0; 64 * 4],
            m_scanlineSprites: vec![],

            m_pipelineState: State::PreRender,

            m_cycle: 0,
            m_scanline: 0,
            m_evenFrame: true,

            m_vblank: false,
            m_sprZeroHit: false,

            // Registers
            m_dataAddress: 0,
            m_tempAddress: 0,
            m_fineXScroll: 0,
            m_firstWrite: true,
            m_dataBuffer: 0,

            m_spriteDataAddress: 0,

            // Setup flags and variables
            m_longSprites: false,
            m_generateInterrupt: false,

            m_greyscaleMode: false,
            m_showSprites: true,
            m_showBackground: true,
            m_hideEdgeSprites: false,
            m_hideEdgeBackground: false,

            m_bgPage: CharacterPage::Low,
            m_sprPage: CharacterPage::Low,

            m_dataAddrIncrement: 1,
            m_pictureBuffer: vec![
                vec![Color::MAGENTA; VISIBLE_SCANLINES as usize];
                SCANLINE_VISIBLE_DOTS as usize
            ],
        }
    }

    pub fn get_data(&self) -> u8 {
        unimplemented!()
    }

    pub fn get_status(&self) -> u8 {
        unimplemented!()
    }

    pub fn get_oam_data(&mut self) -> u8 {
        unimplemented!()
    }

    pub fn step(&mut self) {
        match self.m_pipelineState {
            State::PreRender => {
                if self.m_cycle == 1 {
                    self.m_vblank = false;
                    self.m_sprZeroHit = false;
                } else if self.m_cycle == SCANLINE_VISIBLE_DOTS + 2
                    && self.m_showBackground
                    && self.m_showSprites
                {
                    // Set bits related to horizontal position
                    self.m_dataAddress &= !0x41f; // Unset horizontal bits
                    self.m_dataAddress |= self.m_tempAddress & 0x41f; // Copy
                } else if self.m_cycle > 280
                    && self.m_cycle <= 304
                    && self.m_showBackground
                    && self.m_showSprites
                {
                    // Set vertical bits
                    self.m_dataAddress &= !0x7be0; // Unset bits related to horizontal
                    self.m_dataAddress |= self.m_tempAddress & 0x7be0; // Copy
                }
                if self.m_cycle
                    >= SCANLINE_END_CYCLE
                        - if !self.m_evenFrame && self.m_showBackground && self.m_showSprites {
                            1
                        } else {
                            0
                        }
                {
                    self.m_pipelineState = State::Render;
                    self.m_cycle = 0;
                    self.m_scanline = 0;
                }
            }
            State::Render => self.render(),
            State::PostRender => {}
            State::VerticalBlank => {}
        }
        self.m_cycle += 1;
    }

    fn render(&mut self) {
        if self.m_cycle > 0 && self.m_cycle <= SCANLINE_VISIBLE_DOTS {
            let mut bgColor = 0;
            let mut sprColor = 0;

            let mut bgOpaque = false;
            let mut sprOpaque = true;
            let mut spriteForeground = false;

            let mut x = self.m_cycle - 1;
            let mut y = self.m_scanline;

            if self.m_showBackground {
                let x_fine: u32 = (self.m_fineXScroll as u32 + x) % 8;
                if !self.m_hideEdgeBackground || x >= 8 {
                    // fetch tile
                    let mut addr = 0x2000 | (self.m_dataAddress & 0x0FFF); // mask off fine y
                                                                           // auto addr = 0x2000 + x / 8 + (y / 8) * (ScanlineVisibleDots / 8);
                    let tile = self.read(addr) as u16;

                    // fetch pattern
                    // Each pattern occupies 16 bytes, so multiply by 16
                    addr = (tile * 16) + ((self.m_dataAddress >> 12/*y % 8*/) & 0x7); // Add fine y
                    addr |= (self.m_bgPage.value() << 12) as u16; // set whether the pattern is in the high or low page
                                                                  // Get the corresponding bit determined by (8 - x_fine) from the right
                    bgColor = (self.read(addr) >> (7 ^ x_fine)) & 1; // bit 0 of palette entry
                    bgColor |= ((self.read(addr + 8) >> (7 ^ x_fine)) & 1) << 1; // bit 1

                    bgOpaque = bgColor != 0; // flag used to calculate final pixel with the
                                             // sprite pixel

                    // fetch attribute and calculate higher two bits of palette
                    addr = 0x23C0
                        | (self.m_dataAddress & 0x0C00)
                        | ((self.m_dataAddress >> 4) & 0x38)
                        | ((self.m_dataAddress >> 2) & 0x07);
                    let attribute = self.read(addr);
                    let shift = ((self.m_dataAddress >> 4) & 4) | (self.m_dataAddress & 2);
                    // Extract and set the upper two bits for the color
                    bgColor |= ((attribute >> shift) & 0x3) << 2;
                }
                // Increment/wrap coarse X
                if x_fine == 7 {
                    if (self.m_dataAddress & 0x001F) == 31
                    // if coarse X == 31
                    {
                        self.m_dataAddress &= !0x001F; // coarse X = 0
                        self.m_dataAddress ^= 0x0400; // switch horizontal nametable
                    } else {
                        self.m_dataAddress += 1; // increment coarse X
                    }
                }
            }

            if self.m_showSprites && (!self.m_hideEdgeSprites || x >= 8) {
                let sprites = self.m_scanlineSprites.clone();
                for &i in sprites.iter() {
                    let i = i as usize;
                    let spr_x = self.m_spriteMemory[i * 4 + 3] as u32;

                    if 0 > x - spr_x || x - spr_x >= 8 {
                        continue;
                    }

                    let spr_y = (self.m_spriteMemory[i * 4 + 0] + 1) as u32;
                    let tile = (self.m_spriteMemory[i * 4 + 1]) as u16;
                    let attribute = self.m_spriteMemory[i * 4 + 2];

                    let length: u16 = if self.m_longSprites { 16 } else { 8 };

                    let mut x_shift = (x - spr_x) % 8;
                    let mut y_offset = ((y - spr_y) as u16) % length;

                    if (attribute & 0x40) == 0
                    // If NOT flipping horizontally
                    {
                        x_shift ^= 7;
                    }
                    if (attribute & 0x80) != 0
                    // IF flipping vertically
                    {
                        y_offset ^= length - 1;
                    }
                    let mut addr = 0;

                    if !self.m_longSprites {
                        addr = tile * 16 + y_offset;
                        if self.m_sprPage == CharacterPage::High {
                            addr += 0x1000;
                        }
                    } else
                    // 8x16 sprites
                    {
                        // bit-3 is one if it is the bottom tile of the sprite, multiply by
                        // two to get the next pattern
                        y_offset = (y_offset & 7) | ((y_offset & 8) << 1);
                        addr = (tile >> 1) * 32 + y_offset;
                        addr |= (tile & 1) << 12; // Bank 0x1000 if bit-0 is high
                    }

                    sprColor |= (self.read(addr) >> (x_shift)) & 1; // bit 0 of palette entry
                    sprColor |= ((self.read(addr + 8) >> (x_shift)) & 1) << 1; // bit 1

                    sprOpaque = sprColor != 0;
                    if sprOpaque {
                        sprColor = 0;
                        continue;
                    }

                    sprColor |= 0x10; // Select sprite palette
                    sprColor |= (attribute & 0x3) << 2; // bits 2-3

                    spriteForeground = (attribute & 0x20) == 0;

                    // Sprite-0 hit detection
                    if !self.m_sprZeroHit
                        && self.m_showBackground
                        && i == 0
                        && sprOpaque
                        && bgOpaque
                    {
                        self.m_sprZeroHit = true;
                    }

                    break; // Exit the loop now since we've found the highest priority
                           // sprite
                }
            }

            let mut paletteAddr = bgColor;

            if (!bgOpaque && sprOpaque) || (bgOpaque && sprOpaque && spriteForeground) {
                paletteAddr = sprColor;
            } else if !bgOpaque && !sprOpaque {
                paletteAddr = 0;
            } // else bgColor

            //                     m_screen.setPixel(x, y,
            //                     sf::Color(colors[m_bus.readPalette(paletteAddr)]));
            let palette_index = self.picture_bus.borrow().read_palette(paletteAddr as u16);
            self.m_pictureBuffer[x as usize][y as usize] =
                Color::from(COLORS[palette_index as usize]);
        } else if self.m_cycle == SCANLINE_VISIBLE_DOTS + 1 && self.m_showBackground {
            // Shamelessly copied from nesdev wiki
            if (self.m_dataAddress & 0x7000) != 0x7000 {
                // if fine Y < 7
                self.m_dataAddress += 0x1000; // increment fine Y
            } else {
                self.m_dataAddress &= !0x7000; // fine Y = 0
                let mut y = (self.m_dataAddress & 0x03E0) >> 5; // let y = coarse Y
                if y == 29 {
                    y = 0; // coarse Y = 0
                    self.m_dataAddress ^= 0x0800; // switch vertical nametable
                } else if (y == 31) {
                    y = 0; // coarse Y = 0, nametable not switched
                } else {
                    y += 1; // increment coarse Y
                }
                self.m_dataAddress = (self.m_dataAddress & !0x03E0) | (y << 5);
                // put coarse Y back into m_dataAddress
            }
        } else if self.m_cycle == SCANLINE_VISIBLE_DOTS + 2
            && self.m_showBackground
            && self.m_showSprites
        {
            // Copy bits related to horizontal position
            self.m_dataAddress &= !0x41f;
            self.m_dataAddress |= self.m_tempAddress & 0x41f;
        }

        //   //                 if (m_cycle > 257 && m_cycle < 320)
        //   //                     m_spriteDataAddress = 0;

        if self.m_cycle >= SCANLINE_END_CYCLE {
            // Find and index sprites that are on the next Scanline
            // This isn't where/when this indexing, actually copying in 2C02 is done
            // but (I think) it shouldn't hurt any games if this is done here

            self.m_scanlineSprites.resize(0, 0);

            let mut range = 8;
            if self.m_longSprites {
                range = 16;
            }
            let mut j = 0;
            for i in (self.m_spriteDataAddress / 4)..64 {
                let diff =
                    (self.m_scanline as i32) - (self.m_spriteMemory[(i * 4) as usize] as i32);
                if 0 <= diff && diff < range {
                    self.m_scanlineSprites.push(i);
                    j += 1;
                    if j >= 8 {
                        break;
                    }
                }
            }

            self.m_scanline += 1;
            self.m_cycle = 0;
        }

        if self.m_scanline >= VISIBLE_SCANLINES {
            self.m_pipelineState = State::PostRender;
        }
    }

    fn read(&mut self, addr: Address) -> Byte {
        self.picture_bus.borrow_mut().read(addr)
    }
}
