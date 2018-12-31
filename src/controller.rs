use sfml::window::Key;

pub enum Buttons {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

impl Buttons {
    fn id(&self) -> usize {
        match self {
            Buttons::A => 0,
            Buttons::B => 1,
            Buttons::Select => 2,
            Buttons::Start => 3,
            Buttons::Up => 4,
            Buttons::Down => 5,
            Buttons::Left => 6,
            Buttons::Right => 7,
        }
    }
}

pub struct Controller {
    strobe: bool,
    key_state: u32,
    key_bindings: [Key; 8],
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            strobe: false,
            key_state: 0,
            key_bindings: [
                Key::J,
                Key::K,
                Key::RShift,
                Key::Return,
                Key::W,
                Key::S,
                Key::A,
                Key::D,
            ],
        }
    }

    pub fn read(&mut self) -> u8 {
        let result = if self.strobe {
            if self.key_bindings[Buttons::A.id()].is_pressed() {
                1
            } else {
                0
            }
        } else {
            (self.key_state & 1) as u8
        };
        self.key_state >>= 1;
        result | 0x40
    }

    pub fn set_strobe(&mut self, b: u8) {
        self.strobe = (b & 1) != 0;
        if !self.strobe {
            self.key_state = 0;
            self.key_state = 0;
            for (i, key) in self.key_bindings.iter().enumerate() {
                let is_pressed = if key.is_pressed() { 1 } else { 0 };
                self.key_state |= is_pressed << i;
            }
        }
    }
}
