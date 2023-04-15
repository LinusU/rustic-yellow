use std::sync::mpsc::Receiver;

pub struct Keypad {
    row0: u8,
    row1: u8,
    data: u8,
    events: Receiver<KeypadEvent>,
}

#[derive(Copy, Clone)]
pub enum KeypadKey {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

#[derive(Copy, Clone)]
pub enum KeypadEvent {
    Down(KeypadKey),
    Up(KeypadKey),
}

impl Keypad {
    pub fn new(events: Receiver<KeypadEvent>) -> Keypad {
        Keypad {
            row0: 0x0F,
            row1: 0x0F,
            data: 0xFF,
            events,
        }
    }

    pub fn rb(&mut self) -> u8 {
        self.update();
        self.data
    }

    pub fn wb(&mut self, value: u8) {
        self.data = (self.data & 0xCF) | (value & 0x30);
    }

    fn update(&mut self) {
        loop {
            match self.events.try_recv() {
                Ok(KeypadEvent::Down(key)) => self.keydown(key),
                Ok(KeypadEvent::Up(key)) => self.keyup(key),
                Err(_) => break,
            }
        }

        let mut new_values = 0xF;

        if self.data & 0x10 == 0x00 {
            new_values &= self.row0;
        }
        if self.data & 0x20 == 0x00 {
            new_values &= self.row1;
        }

        self.data = (self.data & 0xF0) | new_values;
    }

    fn keydown(&mut self, key: KeypadKey) {
        match key {
            KeypadKey::Right => self.row0 &= !(1 << 0),
            KeypadKey::Left => self.row0 &= !(1 << 1),
            KeypadKey::Up => self.row0 &= !(1 << 2),
            KeypadKey::Down => self.row0 &= !(1 << 3),
            KeypadKey::A => self.row1 &= !(1 << 0),
            KeypadKey::B => self.row1 &= !(1 << 1),
            KeypadKey::Select => self.row1 &= !(1 << 2),
            KeypadKey::Start => self.row1 &= !(1 << 3),
        }
    }

    fn keyup(&mut self, key: KeypadKey) {
        match key {
            KeypadKey::Right => self.row0 |= 1 << 0,
            KeypadKey::Left => self.row0 |= 1 << 1,
            KeypadKey::Up => self.row0 |= 1 << 2,
            KeypadKey::Down => self.row0 |= 1 << 3,
            KeypadKey::A => self.row1 |= 1 << 0,
            KeypadKey::B => self.row1 |= 1 << 1,
            KeypadKey::Select => self.row1 |= 1 << 2,
            KeypadKey::Start => self.row1 |= 1 << 3,
        }
    }
}
