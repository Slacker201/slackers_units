use std::collections::VecDeque;

mod parse_impl;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum BaseTypeKind {
    #[default]
    Time,
    Distance,
    Mass,
    ElectricCurrent,
    Temperature,
    Amount,
    LuminousIntensity,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Digit {
    #[default]
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Number {
    integer: VecDeque<Digit>,
    decimal: VecDeque<Digit>,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct TypeIdentifier {
    raw: String,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct Expression {
    units: Box<[(TypeIdentifier, i64)]>,
    multiplier: Number,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct Conversion {
    from: Expression,
    to: Expression,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct Conversions {
    conversions: Box<[Conversion]>,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct UnitDeclaration {
    unit: TypeIdentifier,
    prefixable: bool,
    abbreviation: String,
    conversions: Conversions,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct PrefixDeclaration {
    prefix: TypeIdentifier,
    abbreviation: String,
    multiplier: Number,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct System {
    units: Box<[UnitDeclaration]>,
    prefixes: Box<[PrefixDeclaration]>,

    base_types: [TypeIdentifier; 7],
}
