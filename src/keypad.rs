use std::sync::mpsc::Receiver;

#[derive(Debug, Copy, Clone)]
pub enum TextEvent {
    Append(char),
    Delete,
    Submit,
    Cancel,
}

#[derive(Debug, Copy, Clone)]
pub enum KeyboardKey {
    Escape,
    Left,
    Up,
    Right,
    Down,
    Backspace,
    Return,
    Space,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl KeyboardKey {
    fn into_keypad_key(self) -> Option<KeypadKey> {
        match self {
            KeyboardKey::Right | KeyboardKey::D => Some(KeypadKey::Right),
            KeyboardKey::Left | KeyboardKey::A => Some(KeypadKey::Left),
            KeyboardKey::Up | KeyboardKey::W => Some(KeypadKey::Up),
            KeyboardKey::Down | KeyboardKey::S => Some(KeypadKey::Down),

            KeyboardKey::Z | KeyboardKey::N => Some(KeypadKey::A),
            KeyboardKey::X | KeyboardKey::M => Some(KeypadKey::B),

            KeyboardKey::Return => Some(KeypadKey::Start),
            KeyboardKey::Space => Some(KeypadKey::Select),

            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum KeyboardEvent {
    Down { key: KeyboardKey, shift: bool },
    Up { key: KeyboardKey },
}

impl KeyboardEvent {
    fn into_keypad_event(self) -> Option<KeypadEvent> {
        match self {
            KeyboardEvent::Down { key, .. } => key.into_keypad_key().map(KeypadEvent::Down),
            KeyboardEvent::Up { key } => key.into_keypad_key().map(KeypadEvent::Up),
        }
    }

    #[rustfmt::skip]
    fn into_text_event(self) -> Option<TextEvent> {
        match self {
            KeyboardEvent::Down { key: KeyboardKey::Escape, .. } => Some(TextEvent::Cancel),
            KeyboardEvent::Down { key: KeyboardKey::Left, .. } => None,
            KeyboardEvent::Down { key: KeyboardKey::Up, .. } => None,
            KeyboardEvent::Down { key: KeyboardKey::Right, .. } => None,
            KeyboardEvent::Down { key: KeyboardKey::Down, .. } => None,
            KeyboardEvent::Down { key: KeyboardKey::Backspace, .. } => Some(TextEvent::Delete),
            KeyboardEvent::Down { key: KeyboardKey::Return, .. } => Some(TextEvent::Submit),
            KeyboardEvent::Down { key: KeyboardKey::Space, .. } => Some(TextEvent::Append(' ')),
            KeyboardEvent::Down { key: KeyboardKey::A, shift } => Some(TextEvent::Append(if shift { 'A' } else { 'a' })),
            KeyboardEvent::Down { key: KeyboardKey::B, shift } => Some(TextEvent::Append(if shift { 'B' } else { 'b' })),
            KeyboardEvent::Down { key: KeyboardKey::C, shift } => Some(TextEvent::Append(if shift { 'C' } else { 'c' })),
            KeyboardEvent::Down { key: KeyboardKey::D, shift } => Some(TextEvent::Append(if shift { 'D' } else { 'd' })),
            KeyboardEvent::Down { key: KeyboardKey::E, shift } => Some(TextEvent::Append(if shift { 'E' } else { 'e' })),
            KeyboardEvent::Down { key: KeyboardKey::F, shift } => Some(TextEvent::Append(if shift { 'F' } else { 'f' })),
            KeyboardEvent::Down { key: KeyboardKey::G, shift } => Some(TextEvent::Append(if shift { 'G' } else { 'g' })),
            KeyboardEvent::Down { key: KeyboardKey::H, shift } => Some(TextEvent::Append(if shift { 'H' } else { 'h' })),
            KeyboardEvent::Down { key: KeyboardKey::I, shift } => Some(TextEvent::Append(if shift { 'I' } else { 'i' })),
            KeyboardEvent::Down { key: KeyboardKey::J, shift } => Some(TextEvent::Append(if shift { 'J' } else { 'j' })),
            KeyboardEvent::Down { key: KeyboardKey::K, shift } => Some(TextEvent::Append(if shift { 'K' } else { 'k' })),
            KeyboardEvent::Down { key: KeyboardKey::L, shift } => Some(TextEvent::Append(if shift { 'L' } else { 'l' })),
            KeyboardEvent::Down { key: KeyboardKey::M, shift } => Some(TextEvent::Append(if shift { 'M' } else { 'm' })),
            KeyboardEvent::Down { key: KeyboardKey::N, shift } => Some(TextEvent::Append(if shift { 'N' } else { 'n' })),
            KeyboardEvent::Down { key: KeyboardKey::O, shift } => Some(TextEvent::Append(if shift { 'O' } else { 'o' })),
            KeyboardEvent::Down { key: KeyboardKey::P, shift } => Some(TextEvent::Append(if shift { 'P' } else { 'p' })),
            KeyboardEvent::Down { key: KeyboardKey::Q, shift } => Some(TextEvent::Append(if shift { 'Q' } else { 'q' })),
            KeyboardEvent::Down { key: KeyboardKey::R, shift } => Some(TextEvent::Append(if shift { 'R' } else { 'r' })),
            KeyboardEvent::Down { key: KeyboardKey::S, shift } => Some(TextEvent::Append(if shift { 'S' } else { 's' })),
            KeyboardEvent::Down { key: KeyboardKey::T, shift } => Some(TextEvent::Append(if shift { 'T' } else { 't' })),
            KeyboardEvent::Down { key: KeyboardKey::U, shift } => Some(TextEvent::Append(if shift { 'U' } else { 'u' })),
            KeyboardEvent::Down { key: KeyboardKey::V, shift } => Some(TextEvent::Append(if shift { 'V' } else { 'v' })),
            KeyboardEvent::Down { key: KeyboardKey::W, shift } => Some(TextEvent::Append(if shift { 'W' } else { 'w' })),
            KeyboardEvent::Down { key: KeyboardKey::X, shift } => Some(TextEvent::Append(if shift { 'X' } else { 'x' })),
            KeyboardEvent::Down { key: KeyboardKey::Y, shift } => Some(TextEvent::Append(if shift { 'Y' } else { 'y' })),
            KeyboardEvent::Down { key: KeyboardKey::Z, shift } => Some(TextEvent::Append(if shift { 'Z' } else { 'z' })),

            KeyboardEvent::Up { .. } => None,
        }
    }
}

pub struct Keypad {
    row0: u8,
    row1: u8,
    data: u8,
    events: Receiver<KeyboardEvent>,
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
    pub fn new(events: Receiver<KeyboardEvent>) -> Keypad {
        Keypad {
            row0: 0x0F,
            row1: 0x0F,
            data: 0xFF,
            events,
        }
    }

    pub fn wait(&mut self) -> KeypadKey {
        loop {
            match self.events.recv().map(|e| e.into_keypad_event()) {
                Ok(Some(KeypadEvent::Down(key))) => {
                    self.keydown(key);
                    return key;
                }
                Ok(Some(KeypadEvent::Up(key))) => self.keyup(key),
                Ok(None) => {}
                Err(_) => panic!("Keypad event channel closed"),
            }
        }
    }

    pub fn text(&mut self) -> TextEvent {
        loop {
            match self.events.recv().map(|e| e.into_text_event()) {
                Ok(Some(event)) => return event,
                Ok(None) => {}
                Err(_) => panic!("Keypad event channel closed"),
            }
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
            match self.events.try_recv().map(|e| e.into_keypad_event()) {
                Ok(Some(KeypadEvent::Down(key))) => self.keydown(key),
                Ok(Some(KeypadEvent::Up(key))) => self.keyup(key),
                Ok(None) => {}
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
