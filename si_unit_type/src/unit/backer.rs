use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Backer:
    Add<Self, Output = Self>
    + AddAssign
    + Sub<Self, Output = Self>
    + SubAssign
    + Div<Self, Output = Self>
    + DivAssign
    + Mul<Self, Output = Self>
    + MulAssign
    + Clone
{
}

impl Backer for f64 {}
