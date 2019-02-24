#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Rgb24 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb24 {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn to_f32_rgb(self) -> [f32; 3] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
        ]
    }
    pub fn to_f32_rgba(self, opacity: f32) -> [f32; 4] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
            opacity,
        ]
    }
}

pub const fn rgb24(r: u8, g: u8, b: u8) -> Rgb24 {
    Rgb24::new(r, g, b)
}
