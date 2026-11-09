use num_rational::Ratio;
use num_traits::{Signed, ToPrimitive};
mod number_repr;
use number_repr::NumberRepr;

pub trait Conversion {
    fn convert(ratio: Ratio<NumberRepr>) -> Self;
}


impl Conversion for f64 {
    fn convert(ratio: Ratio<NumberRepr>) -> Self {
        let (numer, denom) = ratio.into_raw();

        let numer = numer.to_f64().unwrap_or(if numer.is_negative() {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        });
        let denom = denom.to_f64().unwrap_or(if numer.is_negative() {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        });

        numer / denom
    }
}

impl Conversion for Ratio<NumberRepr> {
    fn convert(ratio: Ratio<NumberRepr>) -> Self {
        ratio
    }
}
