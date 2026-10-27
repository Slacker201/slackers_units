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
    pub integer: VecDeque<Digit>,
    pub decimal: VecDeque<Digit>,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct TypeIdentifier {
    pub raw: String,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct Expression {
    pub units: Box<[(TypeIdentifier, i64)]>,
    pub multiplier: Number,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct Conversion {
    pub from: Expression,
    pub to: Expression,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct Conversions {
    pub conversions: Box<[Conversion]>,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct UnitDeclaration {
    pub unit: TypeIdentifier,
    pub prefixable: bool,
    pub abbreviation: String,
    pub conversions: Conversions,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct PrefixDeclaration {
    pub prefix: TypeIdentifier,
    pub abbreviation: String,
    pub multiplier: Number,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct System {
    pub units: Box<[UnitDeclaration]>,
    pub prefixes: Box<[PrefixDeclaration]>,

    pub base_types: [TypeIdentifier; 7],
}
