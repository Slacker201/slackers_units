use std::{
    fmt::Display,
    ops::{Add, Div, Mul},
};

use crate::{
    unit::{backer::Backer, unit_system::UnitSystem},
    unit_vector::Uv,
};

pub(crate) mod backer;
pub(crate) mod unit_system;

#[derive(Debug, Default, Clone, Copy)]
pub struct Unit<B: Backer, D: Uv, Us: UnitSystem> {
    vector: D,
    amount: B,
    _unit_system: Us,
}

impl<B: Backer, Us: UnitSystem, Uv1: Uv + Mul<Uv2, Output = Uv3>, Uv2: Uv, Uv3: Uv + Default>
    Mul<Unit<B, Uv2, Us>> for Unit<B, Uv1, Us>
{
    type Output = Unit<B, Uv3, Us>;

    fn mul(self, rhs: Unit<B, Uv2, Us>) -> Self::Output {
        Self::Output {
            vector: Uv3::default(),
            amount: self.amount * rhs.amount,
            _unit_system: Us::default(),
        }
    }
}

impl<Us: UnitSystem, B: Backer, Uv1: Uv + Div<Uv2, Output = Uv3>, Uv2: Uv, Uv3: Uv + Default>
    Div<Unit<B, Uv2, Us>> for Unit<B, Uv1, Us>
{
    type Output = Unit<B, Uv3, Us>;

    fn div(self, rhs: Unit<B, Uv2, Us>) -> Self::Output {
        Self::Output {
            vector: Uv3::default(),
            amount: self.amount / rhs.amount,
            _unit_system: Us::default(),
        }
    }
}

impl<Us: UnitSystem, B: Backer, U: Uv + Default> Unit<B, U, Us> {
    pub fn new(amount: B) -> Self {
        Self {
            vector: U::default(),
            amount,
            _unit_system: Us::default(),
        }
    }
}

impl<Us: UnitSystem, B: Backer, U: Uv + Default> Add<Unit<B, U, Us>> for Unit<B, U, Us> {
    type Output = Unit<B, U, Us>;

    fn add(self, rhs: Unit<B, U, Us>) -> Self::Output {
        Self::Output {
            vector: U::default(),
            amount: self.amount + rhs.amount,
            _unit_system: Us::default(),
        }
    }
}

impl<Us: UnitSystem, B: Backer + Display, U: Uv + Display> Display for Unit<B, U, Us> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.amount, self.vector)
    }
}
