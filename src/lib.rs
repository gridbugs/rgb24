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
    pub const fn new_grey(c: u8) -> Self {
        Self::new(c, c, c)
    }
    pub fn floor(self, min: u8) -> Self {
        Self {
            r: self.r.max(min),
            g: self.g.max(min),
            b: self.b.max(min),
        }
    }
    pub fn ceil(self, max: u8) -> Self {
        Self {
            r: self.r.min(max),
            g: self.g.min(max),
            b: self.b.min(max),
        }
    }
    pub fn to_f32_rgb(self) -> [f32; 3] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
        ]
    }
    pub fn to_f32_rgba(self, alpha: f32) -> [f32; 4] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
            alpha,
        ]
    }
    pub fn saturating_add(self, other: Self) -> Self {
        Self {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }
    pub fn saturating_sub(self, other: Self) -> Self {
        Self {
            r: self.r.saturating_sub(other.r),
            g: self.g.saturating_sub(other.g),
            b: self.b.saturating_sub(other.b),
        }
    }
    pub fn saturating_scalar_mul(self, scalar: u32) -> Self {
        fn single_channel(channel: u8, scalar: u32) -> u8 {
            let as_u32 = channel as u32 * scalar;
            as_u32.min(::std::u8::MAX as u32) as u8
        }
        Self {
            r: single_channel(self.r, scalar),
            g: single_channel(self.g, scalar),
            b: single_channel(self.b, scalar),
        }
    }
    pub fn scalar_div(self, scalar: u32) -> Self {
        fn single_channel(channel: u8, scalar: u32) -> u8 {
            let as_u32 = channel as u32 / scalar;
            as_u32.min(::std::u8::MAX as u32) as u8
        }
        Self {
            r: single_channel(self.r, scalar),
            g: single_channel(self.g, scalar),
            b: single_channel(self.b, scalar),
        }
    }
    pub fn saturating_scalar_mul_div(self, numerator: u32, denominator: u32) -> Self {
        fn single_channel(channel: u8, numerator: u32, denominator: u32) -> u8 {
            let as_u32 = ((channel as u32) * (numerator)) / denominator;
            as_u32.min(::std::u8::MAX as u32) as u8
        }
        Self {
            r: single_channel(self.r, numerator, denominator),
            g: single_channel(self.g, numerator, denominator),
            b: single_channel(self.b, numerator, denominator),
        }
    }
    pub fn normalised_mul(self, other: Self) -> Self {
        fn single_channel(a: u8, b: u8) -> u8 {
            ((a as u32 * b as u32) / 255) as u8
        }
        Self {
            r: single_channel(self.r, other.r),
            g: single_channel(self.g, other.g),
            b: single_channel(self.b, other.b),
        }
    }
    pub fn normalised_scalar_mul(self, scalar: u8) -> Self {
        fn single_channel(c: u8, scalar: u8) -> u8 {
            ((c as u32 * scalar as u32) / 255) as u8
        }
        Self {
            r: single_channel(self.r, scalar),
            g: single_channel(self.g, scalar),
            b: single_channel(self.b, scalar),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let a = Rgb24::new(255, 0, 200);
        let b = Rgb24::new(0, 255, 200);
        let c = a.saturating_add(b);
        assert_eq!(c, Rgb24::new(255, 255, 255));
    }

    #[test]
    fn sub() {
        let a = Rgb24::new(255, 0, 200);
        let b = Rgb24::new(0, 255, 200);
        let c = a.saturating_sub(b);
        assert_eq!(c, Rgb24::new(255, 0, 0));
    }

    #[test]
    fn mul_div() {
        assert_eq!(
            Rgb24::new(1, 2, 3).saturating_scalar_mul_div(1500, 1000),
            Rgb24::new(1, 3, 4)
        );
        assert_eq!(
            Rgb24::new(1, 2, 3).saturating_scalar_mul_div(1500, 1),
            Rgb24::new(255, 255, 255)
        );
    }

    #[test]
    fn mul() {
        assert_eq!(
            Rgb24::new(20, 40, 60).saturating_scalar_mul(2),
            Rgb24::new(40, 80, 120),
        );
        assert_eq!(
            Rgb24::new(20, 40, 60).saturating_scalar_mul(10000),
            Rgb24::new(255, 255, 255),
        );
    }

    #[test]
    fn div() {
        assert_eq!(Rgb24::new(20, 40, 60).scalar_div(2), Rgb24::new(10, 20, 30));
        assert_eq!(
            Rgb24::new(255, 255, 255).scalar_div(256),
            Rgb24::new(0, 0, 0)
        );
    }

    #[test]
    #[should_panic]
    fn div_by_zero() {
        Rgb24::new(0, 0, 0).scalar_div(0);
    }

    #[test]
    fn normalised_mul() {
        assert_eq!(
            Rgb24::new(255, 255, 255).normalised_mul(Rgb24::new(1, 2, 3)),
            Rgb24::new(1, 2, 3)
        );
        assert_eq!(
            Rgb24::new(255, 127, 0).normalised_mul(Rgb24::new(10, 20, 30)),
            Rgb24::new(10, 9, 0)
        );
    }

    #[test]
    fn grey() {
        assert_eq!(Rgb24::new_grey(37), Rgb24::new(37, 37, 37));
    }

    #[test]
    fn floor() {
        assert_eq!(Rgb24::new(100, 5, 0).floor(10), Rgb24::new(100, 10, 10));
    }

    #[test]
    fn ceil() {
        assert_eq!(Rgb24::new(255, 250, 20).ceil(200), Rgb24::new(200, 200, 20));
    }

    #[test]
    fn normalised_scalar_mul() {
        assert_eq!(
            Rgb24::new(255, 128, 0).normalised_scalar_mul(128),
            Rgb24::new(128, 64, 0)
        );
    }
}
