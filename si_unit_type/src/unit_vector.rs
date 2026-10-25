use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use typenum::{Diff, Integer, P1, Sum, Z0};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct UnitVector<
    Time: Integer,
    Distance: Integer,
    Mass: Integer,
    ElectricCurrent: Integer,
    Temperature: Integer,
    Amount: Integer,
    Lumenosity: Integer,
> {
    _pd: PhantomData<(
        Time,
        Distance,
        Mass,
        ElectricCurrent,
        Temperature,
        Amount,
        Lumenosity,
    )>,
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
> Mul<UnitVector<T2, D2, M2, E2, Te2, A2, L2>> for UnitVector<T1, D1, M1, E1, Te1, A1, L1>
{
    type Output = UnitVector<
        Sum<T1, T2>,
        Sum<D1, D2>,
        Sum<M1, M2>,
        Sum<E1, E2>,
        Sum<Te1, Te2>,
        Sum<A1, A2>,
        Sum<L1, L2>,
    >;

    fn mul(self, _rhs: UnitVector<T2, D2, M2, E2, Te2, A2, L2>) -> Self::Output {
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
> Div<UnitVector<T2, D2, M2, E2, Te2, A2, L2>> for UnitVector<T1, D1, M1, E1, Te1, A1, L1>
{
    type Output = UnitVector<
        Diff<T1, T2>,
        Diff<D1, D2>,
        Diff<M1, M2>,
        Diff<E1, E2>,
        Diff<Te1, Te2>,
        Diff<A1, A2>,
        Diff<L1, L2>,
    >;

    fn div(self, _rhs: UnitVector<T2, D2, M2, E2, Te2, A2, L2>) -> Self::Output {
        Self::Output::default()
    }
}

pub type Time = UnitVector<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Distance = UnitVector<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
pub type Mass = UnitVector<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
pub type ElectricCurrent = UnitVector<Z0, Z0, Z0, P1, Z0, Z0, Z0>;
pub type Temperature = UnitVector<Z0, Z0, Z0, Z0, P1, Z0, Z0>;
pub type Amount = UnitVector<Z0, Z0, Z0, Z0, Z0, P1, Z0>;
pub type Lumenosity = UnitVector<Z0, Z0, Z0, Z0, Z0, Z0, P1>;
pub type Null = UnitVector<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

pub trait Uv {}
impl<
    Time: Integer,
    Distance: Integer,
    Mass: Integer,
    ElectricCurrent: Integer,
    Temperature: Integer,
    Amount: Integer,
    Lumenosity: Integer,
> Uv for UnitVector<Time, Distance, Mass, ElectricCurrent, Temperature, Amount, Lumenosity>
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
> Display for UnitVector<Time, Distance, Mass, ElectricCurrent, Temperature, Amount, Lumenosity>
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

        let abbreviations = ["s", "m", "kg", "A", "K", "mol", "cd"];

        let mut combined: Box<[(_, _)]> = powers.into_iter().zip(abbreviations).collect();

        combined.sort_by_key(|a| core::cmp::Reverse(a.0));

        let mut wrote_abbrev = false;

        for (pow, abbrev) in combined {
            if pow == 0 {
                continue;
            }

            if pow == 1 {
                if wrote_abbrev {
                    write!(f, "*{abbrev}")?;
                } else {
                    write!(f, "{abbrev}")?;
                }
            } else {
                if pow < 0 {
                    if pow == -1 {
                        write!(f, "/{abbrev}")?;
                    } else {
                        write!(f, "/{abbrev}^{}", pow.abs())?;
                    }
                } else {
                    if wrote_abbrev {
                        write!(f, "*{abbrev}^{pow}")?;
                    } else {
                        write!(f, "{abbrev}^{pow}")?;
                    }
                }
            }
            wrote_abbrev = true;
        }
        Ok(())
    }
}
