use proc_macro2::Span;
use syn::Ident;

use crate::data::{Decimal, Id, registries::NameRegistry, unit_context::UnitContext};

#[derive(Clone, Debug)]
pub(crate) struct System {
    pub units: Box<[UnitDefinition<Id>]>,
    #[allow(unused)]
    pub prefixes: Box<[PrefixDefinition]>,
    pub bases: Box<[BaseDefinition]>,
    pub unit_registry: NameRegistry,
    pub unit_context: UnitContext,
}
#[derive(Clone, Debug)]
pub(crate) struct UnitDefinition<T> {
    pub name: Ident,
    pub id: Id,
    pub aliases: Aliases,
    pub prefix_groups: PrefixGrouping,
    pub equality: Option<EqualityExpression<T>>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub(crate) struct PrefixDefinition {
    #[allow(unused)]
    pub name: Ident,
    pub id: Id,
    pub multiplier: Decimal,
    pub aliases: Aliases,
    #[allow(unused)]
    pub groups: PrefixGrouping,
    #[allow(unused)]
    pub span: Span,
}

syn::custom_keyword!(prefix);
syn::custom_keyword!(multiplier);
syn::custom_keyword!(mult);

syn::custom_keyword!(base);

#[derive(Clone, Debug)]
pub struct BaseDefinition {
    pub unit: Ident,
}

#[derive(Clone, Debug)]
pub struct EqualityExpression<T> {
    pub inner: EqOpt<T>,
    pub span: Span,
}

#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct PrefixGrouping {
    #[allow(unused)]
    pub allowed_groups: Box<[Ident]>,
    pub span: Span,
}
syn::custom_keyword!(prefix_classes);
#[derive(Clone, Debug)]
pub struct Aliases {
    pub aliases: Box<[Ident]>,
    pub span: Span,
}
syn::custom_keyword!(aliases);
