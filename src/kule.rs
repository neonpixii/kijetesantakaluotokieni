pub enum Formats {
    Plain,
    Bright,
    Dim,
    Italic,
    Blink,
}

// outputs a code to reset all formatting. equivalent to Formats::Plain.escape(false)
pub fn reset() -> String {
    "\x1b[0m".to_string()
}

impl Formats {
    fn escape_code(&self, disable: bool) -> u8 {
        let mut code: u8 = match self {
            Formats::Plain => 0,
            Formats::Bright => 1,
            Formats::Dim => 2,
            Formats::Italic => 3,
            Formats::Blink => 5,
        };
        if code == 0 {
            // you can't disable plain
            return code;
        } else if disable {
            code += 20;
        }
        code
    }
    pub fn escape(&self, disable: bool) -> String {
        format!("\x1b[{}m", self.escape_code(disable))
    }
}
pub enum FourBit {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl FourBit {
    fn escape_code(&self, background: bool) -> u8 {
        let mut code: u8 = match self {
            FourBit::Black => 30,
            FourBit::Red => 31,
            FourBit::Green => 32,
            FourBit::Yellow => 33,
            FourBit::Blue => 34,
            FourBit::Magenta => 35,
            FourBit::Cyan => 36,
            FourBit::White => 37,
            FourBit::BrightBlack => 90,
            FourBit::BrightRed => 91,
            FourBit::BrightGreen => 92,
            FourBit::BrightYellow => 93,
            FourBit::BrightBlue => 94,
            FourBit::BrightMagenta => 95,
            FourBit::BrightCyan => 96,
            FourBit::BrightWhite => 97,
        };
        if background {
            code += 10;
        }
        code
    }
    pub fn escape(&self, background: bool) -> String {
        format!("\x1b[{}m", self.escape_code(background))
    }
}
