use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Immediate(i16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Address(u16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Offset(i8);

impl Immediate {
    pub fn new(value: i16) -> Option<Self> {
        if value < -128 || value > 255 {
            None
        } else {
            Some(Immediate(value))
        }
    }

    pub fn value(&self) -> u8 {
        (self.0 & 0xff) as u8
    }
}

impl Address {
    pub fn new(value: u16) -> Option<Address> {
        if value & 1 != 0 {
            warn!("Address 0x{value:04X} is not aligned");
        }

        // All instructions are 2 bytes long, so we should only address even addresses
        // Therefore, we can safely ignore the least significant bit
        let value = value >> 1;

        if value > 0x03ff { // 10 bits
            None
        } else {
            Some(Address(value))
        }
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

impl Offset {
    pub fn new(value: i8) -> Option<Self> {
        if value < -8 || value > 7 {
            None
        } else {
            Some(Offset(value))
        }
    }

    pub fn value(&self) -> i8 {
        self.0
    }

    pub fn encode(&self) -> u8 {
        (self.0 & 0x0f) as u8
    }
}

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:04X}", self.0 << 1) // Shift back to the original value
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}