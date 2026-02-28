use anyhow::Result;

pub struct RGB {
    r: u8,
    g: u8, 
    b: u8,
}

impl RGB {
    pub fn hex_to_rgb(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches("#");

        let num = u32::from_str_radix(hex, 16)?;

        let r = ((num >> 16) & 0xFF) as u8;
        let g = ((num >> 8) & 0xFF) as u8;
        let b = (num & 0xFF) as u8;

        Ok(Self { r, g, b })
    }

    pub fn r(&self) -> u8 { self.r }
    pub fn g(&self) -> u8 { self.g }
    pub fn b(&self) -> u8 { self.b }

    pub fn header_color() -> owo_colors::Rgb {
        owo_colors::Rgb(196, 167, 231)
    }
}

