use bnum::Integer;
use proc_macro2::Span;
use std::hash::Hash;

use crate::unit_system::parse_impl::{default_repr::DefaultRepr, expression::expr::MExpr};

mod parse_impl;

cfg_select! {
    feature = "converter_256" => {
        pub type NumberRepr = Integer<true, 256, 0, 0>;
    }
    feature = "converter_128" => {
        pub type NumberRepr = Integer<true, 128, 0, 0>;
    }
    feature = "converter_64" => {
        pub type NumberRepr = Integer<true, 64, 0, 0>;
    }
    feature = "converter_32" => {
        pub type NumberRepr = Integer<true, 32, 0, 0>;
    }
    feature = "converter_16" => {
        pub type NumberRepr = Integer<true, 16, 0, 0>;
    }
    feature = "converter_8" => {
        pub type NumberRepr = Integer<true, 8, 0, 0>;
    }
    feature = "converter_4" => {
        pub type NumberRepr = Integer<true, 4, 0, 0>;
    }
    feature = "converter_2" => {
        pub type NumberRepr = Integer<true, 2, 0, 0>;
    }
    feature = "converter_1" => {
        pub type NumberRepr = Integer<true, 1, 0, 0>;
    }
    _ => {
        pub type NumberRepr = Integer<true, 64, 0, 0>;
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct Number {
    pub ratio: num_rational::Ratio<NumberRepr>,
    pub suffix: String,
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
    pub call_site: String,
    pub default_repr: Option<DefaultRepr>,

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
