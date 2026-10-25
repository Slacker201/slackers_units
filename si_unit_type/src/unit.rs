use std::{
    fmt::Display,
    ops::{Add, Div, Mul},
};

use crate::unit_vector::Uv;
#[derive(Debug, Default, Clone, Copy)]
pub struct Unit<D: Uv> {
    vector: D,
    amount: f64,
}

impl<Uv1: Uv + Mul<Uv2, Output = Uv3>, Uv2: Uv, Uv3: Uv + Default> Mul<Unit<Uv2>> for Unit<Uv1> {
    type Output = Unit<Uv3>;

    fn mul(self, rhs: Unit<Uv2>) -> Self::Output {
        Self::Output {
            vector: Uv3::default(),
            amount: self.amount * rhs.amount,
        }
    }
}

impl<Uv1: Uv + Div<Uv2, Output = Uv3>, Uv2: Uv, Uv3: Uv + Default> Div<Unit<Uv2>> for Unit<Uv1> {
    type Output = Unit<Uv3>;

    fn div(self, rhs: Unit<Uv2>) -> Self::Output {
        Self::Output {
            vector: Uv3::default(),
            amount: self.amount / rhs.amount,
        }
    }
}

impl<U: Uv + Default> Unit<U> {
    pub fn new(amount: f64) -> Self {
        Self {
            vector: U::default(),
            amount,
        }
    }
}

impl<U: Uv + Default> Add<Unit<U>> for Unit<U> {
    type Output = Unit<U>;

    fn add(self, rhs: Unit<U>) -> Self::Output {
        Self::Output {
            vector: U::default(),
            amount: self.amount + rhs.amount,
        }
    }
}

impl<U: Uv + Display> Display for Unit<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.amount, self.vector)
    }
}
