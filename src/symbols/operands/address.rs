use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Address(u16);

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

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:04X}", self.0 << 1) // Shift back to the original value
    }
}