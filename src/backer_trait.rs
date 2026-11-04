use fastnum::decimal::Decimal;

pub trait Backer: From<Decimal<8>> {}

impl Backer for f64 {}