use proc_macro2::Span;
use syn::Ident;

use crate::data::{Decimal, Id};

#[derive(Clone)]
pub(crate) struct System {
    pub units: Box<[UnitDefinition<Id>]>,
    // pub bases: Box<[Id]>,
    // pub span: Span,
}
#[derive(Clone)]
pub(crate) struct UnitDefinition<T> {
    pub name: Ident,
    pub id: Id,
    pub aliases: Aliases,
    pub prefix_groups: PrefixGrouping,
    pub equality: Option<EqualityExpression<T>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct EqualityExpression<T> {
    pub inner: EqOpt<T>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum EqOpt<T> {
    Multiply {
        first: Box<Self>,
        next: Box<[(Op, Self)]>,
        span: Span,
    },
    #[allow(unused)]
    Pow {
        lhs: Box<Self>,
        pow: i32,
        span: Span,
    },
    Atom {
        multiplier: Decimal,
        unit: T,
        span: Span,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    Mul,
    Div,
}
syn::custom_keyword!(unit);
#[derive(Clone)]
pub struct PrefixGrouping {
    pub allowed_groups: Box<[Ident]>,
    pub span: Span,
}
syn::custom_keyword!(prefix_classes);
#[derive(Clone)]
pub struct Aliases {
    pub aliases: Box<[Ident]>,
    pub span: Span,
}
syn::custom_keyword!(aliases);

#[derive(Clone)]
pub struct Number<T> {
    value: Decimal,
    suffix: Option<T>,
}