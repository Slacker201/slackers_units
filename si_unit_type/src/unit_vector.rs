use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use typenum::{Diff, Integer, P1, Sum, Z0};

use crate::unit::unit_system::{Metric, UnitSystem};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct UnitVector<
    Time: Integer,
    Distance: Integer,
    Mass: Integer,
    ElectricCurrent: Integer,
    Temperature: Integer,
    Amount: Integer,
    Lumenosity: Integer,
    System: UnitSystem,
> {
    _pd1: PhantomData<(Time, Distance, Mass, ElectricCurrent)>,
    _pd2: PhantomData<(Temperature, Amount, Lumenosity, System)>,
}

impl<
    T1: Integer + Add<T2, Output = T3>,
    T2: Integer,
    T3: Integer,
    D1: Integer + Add<D2, Output = D3>,
    D2: Integer,
    D3: Integer,
    M1: Integer + Add<M2, Output = M3>,
    M2: Integer,
    M3: Integer,
    E1: Integer + Add<E2, Output = E3>,
    E2: Integer,
    E3: Integer,
    Te1: Integer + Add<Te2, Output = Te3>,
    Te2: Integer,
    Te3: Integer,
    A1: Integer + Add<A2, Output = A3>,
    A2: Integer,
    A3: Integer,
    L1: Integer + Add<L2, Output = L3>,
    L2: Integer,
    L3: Integer,
    System: UnitSystem,
> Mul<UnitVector<T2, D2, M2, E2, Te2, A2, L2, System>>
    for UnitVector<T1, D1, M1, E1, Te1, A1, L1, System>
{
    type Output = UnitVector<
        Sum<T1, T2>,
        Sum<D1, D2>,
        Sum<M1, M2>,
        Sum<E1, E2>,
        Sum<Te1, Te2>,
        Sum<A1, A2>,
        Sum<L1, L2>,
        System,
    >;

    fn mul(self, _rhs: UnitVector<T2, D2, M2, E2, Te2, A2, L2, System>) -> Self::Output {
        Self::Output::default()
    }
}

impl<
    T1: Integer + Sub<T2, Output = T3>,
    T2: Integer,
    T3: Integer,
    D1: Integer + Sub<D2, Output = D3>,
    D2: Integer,
    D3: Integer,
    M1: Integer + Sub<M2, Output = M3>,
    M2: Integer,
    M3: Integer,
    E1: Integer + Sub<E2, Output = E3>,
    E2: Integer,
    E3: Integer,
    Te1: Integer + Sub<Te2, Output = Te3>,
    Te2: Integer,
    Te3: Integer,
    A1: Integer + Sub<A2, Output = A3>,
    A2: Integer,
    A3: Integer,
    L1: Integer + Sub<L2, Output = L3>,
    L2: Integer,
    L3: Integer,
    System: UnitSystem,
> Div<UnitVector<T2, D2, M2, E2, Te2, A2, L2, System>>
    for UnitVector<T1, D1, M1, E1, Te1, A1, L1, System>
{
    type Output = UnitVector<
        Diff<T1, T2>,
        Diff<D1, D2>,
        Diff<M1, M2>,
        Diff<E1, E2>,
        Diff<Te1, Te2>,
        Diff<A1, A2>,
        Diff<L1, L2>,
        System,
    >;

    fn div(self, _rhs: UnitVector<T2, D2, M2, E2, Te2, A2, L2, System>) -> Self::Output {
        Self::Output::default()
    }
}

pub type Second = UnitVector<P1, Z0, Z0, Z0, Z0, Z0, Z0, Metric>;
pub type Meter = UnitVector<Z0, P1, Z0, Z0, Z0, Z0, Z0, Metric>;
pub type Gram = UnitVector<Z0, Z0, P1, Z0, Z0, Z0, Z0, Metric>;
pub type Ampere = UnitVector<Z0, Z0, Z0, P1, Z0, Z0, Z0, Metric>;
pub type Kelvin = UnitVector<Z0, Z0, Z0, Z0, P1, Z0, Z0, Metric>;
pub type Mole = UnitVector<Z0, Z0, Z0, Z0, Z0, P1, Z0, Metric>;
pub type Candela = UnitVector<Z0, Z0, Z0, Z0, Z0, Z0, P1, Metric>;
pub type MetricNull = UnitVector<Z0, Z0, Z0, Z0, Z0, Z0, Z0, Metric>;

pub trait Uv {}
impl<
    Time: Integer,
    Distance: Integer,
    Mass: Integer,
    ElectricCurrent: Integer,
    Temperature: Integer,
    Amount: Integer,
    Lumenosity: Integer,
    System: UnitSystem,
> Uv
    for UnitVector<Time, Distance, Mass, ElectricCurrent, Temperature, Amount, Lumenosity, System>
{
}

impl<
    Time: Integer,
    Distance: Integer,
    Mass: Integer,
    ElectricCurrent: Integer,
    Temperature: Integer,
    Amount: Integer,
    Lumenosity: Integer,
    System: UnitSystem,
> Display
    for UnitVector<Time, Distance, Mass, ElectricCurrent, Temperature, Amount, Lumenosity, System>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let powers = [
            Time::I64,
            Distance::I64,
            Mass::I64,
            ElectricCurrent::I64,
            Temperature::I64,
            Amount::I64,
            Lumenosity::I64,
        ];

        let abbreviations = [
            System::TIME_SUFFIX,
            System::DISTANCE_SUFFIX,
            System::MASS_SUFFIX,
            System::ELECTRIC_CURRENT_SUFFIX,
            System::TEMPERATURE_SUFFIX,
            System::AMOUNT_SUFFIX,
            System::LUMENOSITY_SUFFIX,
        ];

        let combined: Box<[(_, _)]> = powers.into_iter().zip(abbreviations).collect();

        let mut positive = vec![];
        let mut negative = vec![];

        for (power, t) in combined {
            if power < 0 {
                negative.push((power, t));
            } else {
                positive.push((power.abs(), t));
            }
        }

        positive.sort_by_key(|a| std::cmp::Reverse(a.0));
        negative.sort_by_key(|a| std::cmp::Reverse(a.0));

        let mut top_string = String::new();

        let mut positive_abbrev = false;

        for (power, name) in positive {
            if power == 0 {
                continue;
            }

            if positive_abbrev {
                top_string.push('*');
            } else {
                positive_abbrev = true;
            }

            if power == 1 {
                top_string.push_str(name);
            } else {
                top_string.push_str(name);
                top_string.push('^');
                top_string.push_str(&power.to_string());
            }
        }

        let mut negative_abbrev = false;
        let mut bottom_string = String::new();

        for (power, name) in negative {
            if negative_abbrev {
                bottom_string.push('*');
            } else {
                negative_abbrev = true;
            }

            if power == 1 {
                bottom_string.push_str(name);
            } else {
                bottom_string.push_str(name);
                bottom_string.push('^');
                bottom_string.push_str(&power.to_string());
            }
        }

        match (!top_string.is_empty(), !bottom_string.is_empty()) {
            (false, false) => Ok(()),
            (true, false) => {
                write!(f, "{top_string}")
            }
            (false, true) => {
                write!(f, "/({bottom_string})")
            }
            (true, true) => {
                write!(f, "*({top_string}/{bottom_string})")
            }
        }
    }
}
