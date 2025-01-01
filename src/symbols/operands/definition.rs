use std::fmt::Display;
use crate::symbols::operands::immediate::Immediate;

#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
    pub name: String,
    pub value: Option<Immediate>
}

impl Definition {
    pub fn new(name: &str) -> Definition {
        Definition {
            name: name.to_string(),
            value: None
        }
    }
}

impl Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.value.is_some() {
            write!(f, "{} ({})", self.name, self.value.unwrap())
        } else {
            write!(f, "{} (NULL)", self.name)
        }
    }
}