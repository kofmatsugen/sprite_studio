use std::ops::{Add, Mul, Sub};

pub trait Interpolate:
    Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<f32, Output = Self> + Clone + Copy
{
}

impl<T> Interpolate for T where
    T: Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<f32, Output = Self> + Clone + Copy
{
}
