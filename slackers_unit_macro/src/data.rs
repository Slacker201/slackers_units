use bnum::Integer;
use num_rational::Ratio;

pub(crate) mod parser_data;
pub(crate) mod registries;
pub(crate) mod unit_context;
pub(crate) mod validated_sys;
pub type Decimal = Ratio<Integer<true, 8, 0, 0>>;
pub type DecimalPart = Integer<true, 8, 0, 0>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub u64);
