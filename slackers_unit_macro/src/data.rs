pub(crate) mod parser_data;
pub(crate) mod registries;
pub(crate) mod unit_context;
pub type Decimal = f64;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub u64);
