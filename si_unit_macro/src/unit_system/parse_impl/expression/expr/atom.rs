use std::borrow::Cow;

use proc_macro2::Span;
use syn::{Error, Ident, LitFloat, LitInt, parenthesized, parse::Parse};

use crate::unit_system::parse_impl::expression::expr::{Atom, Item, MExpr};



impl Parse for Atom {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitInt) {
            let value: LitInt = input.parse()?;
            let span= value.span();
            return Ok (
                Self::Item { item: Item::LitInt { value}, span }
            );
        }
        if input.peek(LitFloat) {
            let value: LitFloat = input.parse()?;
            let span= value.span();
            return Ok (
                Self::Item { item: Item::LitFloat { value}, span }
            );
        }
        if input.peek(Ident) {
            let value: Ident = input.parse()?;
            let span= value.span();
            Ok (
                Self::Item { item: Item::Unit { value }, span }
            )
        } else {
            let content;
            parenthesized!(content in input);

            let m_expr = content.parse::<MExpr>()?;
            let span = m_expr.span();

            Ok(
                Self::Parenthesized { m_expr: Box::new(m_expr), span }
            )
        }
    }
}

impl Atom {
    #[allow(unused)]
    pub(crate) fn suffix<'a>(&'a self) -> Option<Cow<'a, str>> {
        match self {
            Atom::Item { item, span: _ } => {
                match item {
                    Item::LitInt { value } => Some(Cow::Borrowed(value.suffix())),
                    Item::LitFloat { value } => Some(Cow::Borrowed(value.suffix())),
                    Item::Unit { value } => {
                        Some(Cow::Owned(value.to_string()))
                    },
                }
            },
            Atom::Parenthesized { m_expr: _, span: _ } => None,
        }
    }

    pub(crate) fn span(&self) -> Span {
        match self {
            Atom::Item { item: _, span } => *span,
            Atom::Parenthesized { m_expr: _, span } => *span,
        }
    }

    pub fn contains_power(&self) -> Option<Span> {
        match self {
            Atom::Item { item: _, span: _ } => None,
            Atom::Parenthesized { m_expr, span: _ } => m_expr.contains_power(),
        }
    }
}