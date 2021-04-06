/// A color with 8 bit components.
///
/// Stored internally as a little endian integer.
pub struct Color(u32);

impl Color {
    pub const RED: Color = Color::rgb(u8::MAX, u8::MIN, u8::MIN);
    pub const GREEN: Color = Color::rgb(u8::MIN, u8::MAX, u8::MIN);
    pub const BLUE: Color = Color::rgb(u8::MIN, u8::MIN, u8::MAX);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, u8::MAX)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(
            (r as u32) << 24 |
            (g as u32) << 16 |
            (b as u32) << 8  |
            (a as u32)
        )
    }

    pub fn red(&self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub fn green(&self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub fn blue(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn alpha(&self) -> u8 {
        self.0 as u8
    }
}
