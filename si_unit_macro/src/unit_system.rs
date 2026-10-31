use std::{collections::VecDeque, hash::Hash};

use proc_macro2::Span;

use crate::unit_system::parse_impl::expression::expr::MExpr;


mod parse_impl;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Digit {
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

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct Number {
    pub integer: VecDeque<Digit>,
    pub decimal: VecDeque<Digit>,

    pub span: Span,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct TypeIdentifier {
    pub raw: String,

    pub span: Span,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct Expression {
    pub expr: MExpr,

    pub span: Span,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct Conversion {
    pub from: Expression,
    pub to: Expression,

    pub span: Span,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct Conversions {
    pub conversions: Box<[Conversion]>,

    pub span: Span,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct UnitDeclaration {
    pub unit: TypeIdentifier,
    pub prefixable: bool,
    pub abbreviation: Abbreviation,
    pub conversions: Conversions,

    pub span: Span,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct PrefixDeclaration {
    pub prefix: TypeIdentifier,
    pub abbreviation: Abbreviation,
    pub multiplier: Number,

    pub span: Span,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct BaseTypeDeclaration {
    pub ty: TypeIdentifier,

    pub span: Span,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct System {
    pub units: Box<[UnitDeclaration]>,
    pub prefixes: Box<[PrefixDeclaration]>,
    pub base_types: Box<[BaseTypeDeclaration]>,

    pub span: Span,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct Abbreviation {
    pub raw: String,

    pub span: Span,
}

impl PartialEq for Abbreviation {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Hash for Abbreviation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl Eq for Abbreviation {}
