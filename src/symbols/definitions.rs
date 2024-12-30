use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Port {
    PixelX = 240,
    PixelY,
    DrawPixel,
    ClearPixel,
    LoadPixel,
    BufferScreen,
    ClearScreenBuffer,
    WriteChar,
    BufferChars,
    ClearCharsBuffer,
    ShowNumber,
    ClearNumber,
    SignedMode,
    UnsignedMode,
    RNG,
    ControllerInput
}

#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
    pub name: String,
    pub value: Option<i16>
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

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Port::PixelX => "PIXEL_X",
            Port::PixelY => "PIXEL_Y",
            Port::DrawPixel => "DRAW_PIXEL",
            Port::ClearPixel => "CLEAR_PIXEL",
            Port::LoadPixel => "LOAD_PIXEL",
            Port::BufferScreen => "BUFFER_SCREEN",
            Port::ClearScreenBuffer => "CLEAR_SCREEN_BUFFER",
            Port::WriteChar => "WRITE_CHAR",
            Port::BufferChars => "BUFFER_CHARS",
            Port::ClearCharsBuffer => "CLEAR_CHARS_BUFFER",
            Port::ShowNumber => "SHOW_NUMBER",
            Port::ClearNumber => "CLEAR_NUMBER",
            Port::SignedMode => "SIGNED_MODE",
            Port::UnsignedMode => "UNSIGNED_MODE",
            Port::RNG => "RNG",
            Port::ControllerInput => "CONTROLLER_INPUT"
        };
        write!(f, "{} (0x{:02X})", text, self.clone() as u8)
    }
}