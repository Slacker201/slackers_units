use bnum::Integer;
use num_rational::Ratio;
use num_traits::{Signed, ToPrimitive};

pub trait Conversion {
    fn convert(ratio: Ratio<Integer<true, 256, 0, 0>>) -> Self;
}

impl Conversion for f64 {
    fn convert(ratio: Ratio<Integer<true, 256, 0, 0>>) -> Self {
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

impl Conversion for Ratio<Integer<true, 256, 0, 0>> {
    fn convert(ratio: Ratio<Integer<true, 256, 0, 0>>) -> Self {
        ratio
    }
}
