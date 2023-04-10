pub struct Serial {
    data: u8,
    control: u8,
    pub interrupt: u8,
}

impl Serial {
    pub fn new() -> Serial {
        Serial {
            data: 0,
            control: 0,
            interrupt: 0,
        }
    }

    pub fn wb(&mut self, a: u16, v: u8) {
        match a {
            0xFF01 => self.data = v,
            0xFF02 => {
                self.control = v;

                if v == 0x81 {
                    // TODO: Send data somewhere?
                }
            }
            _ => panic!("Serial does not handle address {:4X} (write)", a),
        };
    }

    pub fn rb(&self, a: u16) -> u8 {
        match a {
            0xFF01 => self.data,
            0xFF02 => self.control,
            _ => panic!("Serial does not handle address {:4X} (read)", a),
        }
    }
}
