use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Immediate(i16);

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

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}