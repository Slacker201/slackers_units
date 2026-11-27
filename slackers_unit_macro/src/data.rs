use num_rational::Ratio;

pub(crate) mod parser_data;
pub(crate) mod registries;
pub(crate) mod unit_context;
pub(crate) mod validated_sys;
mod decimal_type;

pub type RatioRepr = Ratio<NumberRepr>;
pub type NumberRepr = decimal_type::NumberRepr;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub u64);
