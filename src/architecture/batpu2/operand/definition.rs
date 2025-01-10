use std::fmt::Display;
use crate::architecture::batpu2::KEYWORDS;

#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
    pub name: String,
    pub value: Option<i16>
}

impl Definition {
    pub fn new_def(name: &str, value: i16) -> Option<Definition> {
        if name.is_empty() || KEYWORDS.contains(&name.to_lowercase().as_str()) {
            return None;
        }
        if value < -128 || value > 255 {
            None
        } else {
            Some(Definition{
                name: name.to_string(),
                value: Some(value)
            })
        }
    }

    pub fn new_opr(name: &str) -> Option<Definition> {
        if name.is_empty() || KEYWORDS.contains(&name.to_lowercase().as_str()) {
            None
        } else {
            Some(Definition {
                name: name.to_string(),
                value: None
            })
        }
    }
}

impl Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.value {
            Some(value) => write!(f, "{} ({})", self.name, value),
            None => write!(f, "{} (NULL)", self.name)
        }
    }
}