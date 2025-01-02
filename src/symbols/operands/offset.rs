use std::fmt::Display;

/// A 4 bit signed number
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Nybble(i8);

impl Nybble {
    pub fn new(value: i8) -> Option<Self> {
        if value < -8 || value > 7 {
            None
        } else {
            Some(Nybble(value))
        }
    }

    pub fn value(&self) -> u8 {
        (self.0 & 0x0f) as u8
    }
}

impl Display for Nybble {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i8> for Nybble {
    fn from(value: i8) -> Self {
        Nybble(value)
    }
}

impl From<Nybble> for i8 {
    fn from(value: Nybble) -> Self {
        value.0
    }
}

impl From<Nybble> for i16 {
    fn from(value: Nybble) -> Self {
        value.0 as i16
    }
}