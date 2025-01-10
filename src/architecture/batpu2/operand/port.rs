use std::fmt::Display;
use std::str::FromStr;

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
        write!(f, "{} (0x{:02X})", text, *self as u8)
    }
}

impl FromStr for Port {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pixel_x" => Ok(Port::PixelX),
            "pixel_y" => Ok(Port::PixelY),
            "draw_pixel" => Ok(Port::DrawPixel),
            "clear_pixel" => Ok(Port::ClearPixel),
            "load_pixel" => Ok(Port::LoadPixel),
            "buffer_screen" => Ok(Port::BufferScreen),
            "clear_screen_buffer" => Ok(Port::ClearScreenBuffer),
            "write_char" => Ok(Port::WriteChar),
            "buffer_chars" => Ok(Port::BufferChars),
            "clear_chars_buffer" => Ok(Port::ClearCharsBuffer),
            "show_number" => Ok(Port::ShowNumber),
            "clear_number" => Ok(Port::ClearNumber),
            "signed_mode" => Ok(Port::SignedMode),
            "unsigned_mode" => Ok(Port::UnsignedMode),
            "rng" => Ok(Port::RNG),
            "controller_input" => Ok(Port::ControllerInput),
            _ => Err(())
        }
    }
}