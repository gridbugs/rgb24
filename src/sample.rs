use crate::Rgb24;
use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformInt, UniformSampler};
use rand::prelude::*;

pub struct UniformRgb24LinearInterpolate {
    inner: UniformInt<u8>,
    low: Rgb24,
    high: Rgb24,
}

impl UniformSampler for UniformRgb24LinearInterpolate {
    type X = Rgb24;
    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        Self {
            inner: UniformInt::<u8>::new(::std::u8::MIN, ::std::u8::MAX),
            low: *low.borrow(),
            high: *high.borrow(),
        }
    }
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        UniformSampler::new(low, high)
    }
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        self.low
            .linear_interpolate(self.high, self.inner.sample(rng))
    }
}

impl SampleUniform for Rgb24 {
    type Sampler = UniformRgb24LinearInterpolate;
}
