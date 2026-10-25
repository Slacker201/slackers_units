use std::{
    fmt::Display,
    ops::{Add, Div, Mul},
};

use crate::{unit::backer::Backer, unit_vector::Uv};

pub(crate) mod backer;

#[derive(Debug, Default, Clone, Copy)]
pub struct Unit<B: Backer, D: Uv> {
    vector: D,
    amount: B,
}

impl<B: Backer, Uv1: Uv + Mul<Uv2, Output = Uv3>, Uv2: Uv, Uv3: Uv + Default> Mul<Unit<B, Uv2>>
    for Unit<B, Uv1>
{
    type Output = Unit<B, Uv3>;

    fn mul(self, rhs: Unit<B, Uv2>) -> Self::Output {
        Self::Output {
            vector: Uv3::default(),
            amount: self.amount * rhs.amount,
        }
    }
}

impl<B: Backer, Uv1: Uv + Div<Uv2, Output = Uv3>, Uv2: Uv, Uv3: Uv + Default> Div<Unit<B, Uv2>>
    for Unit<B, Uv1>
{
    type Output = Unit<B, Uv3>;

    fn div(self, rhs: Unit<B, Uv2>) -> Self::Output {
        Self::Output {
            vector: Uv3::default(),
            amount: self.amount / rhs.amount,
        }
    }
}

impl<B: Backer, U: Uv + Default> Unit<B, U> {
    pub fn new(amount: B) -> Self {
        Self {
            vector: U::default(),
            amount,
        }
    }
}

impl<B: Backer, U: Uv + Default> Add<Unit<B, U>> for Unit<B, U> {
    type Output = Unit<B, U>;

    fn add(self, rhs: Unit<B, U>) -> Self::Output {
        Self::Output {
            vector: U::default(),
            amount: self.amount + rhs.amount,
        }
    }
}

impl<B: Backer + Display, U: Uv + Display> Display for Unit<B, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.amount, self.vector)
    }
}
