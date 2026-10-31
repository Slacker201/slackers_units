use proc_macro2::Span;
use syn::{Error, Token, parse::Parse};

use crate::{
    system_impl::system_validator::validator_utils::AcceptOnlyOne,
    unit_system::parse_impl::expression::expr::{MExpr, MOp, PExpr},
};

impl Parse for MExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let first = input.parse::<PExpr>()?;
        let start_span = first.span();

        let mut next = vec![];
        let mut next_span = start_span;

        while let Ok(op) = input.parse::<MOp>() {
            if let Ok(v) = input.parse::<PExpr>() {
                next_span = v.span();
                next.push((op, v));
            } else {
                break;
            }
        }

        Ok(Self {
            first,
            next: next.into_boxed_slice(),
            span: start_span.join(next_span).unwrap_or(start_span),
        })
    }
}

impl Parse for MOp {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            Ok(Self::Mul)
        } else if input.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            Ok(Self::Div)
        } else {
            Err(Error::new(input.span(), "Expected `*` or `/`"))
        }
    }
}

impl MExpr {
    pub(crate) fn span(&self) -> Span {
        self.span
    }

    pub(crate) fn contains_power(&self) -> Option<Span> {
        if let Some(s) = self.first.contains_power() {
            return Some(s);
        }
        self.next
            .iter()
            .map(|(_, p_expr)| p_expr.contains_power())
            .find(|c| c.is_some())
            .flatten()
    }

    pub(crate) fn contains_two_units(&self) -> AcceptOnlyOne<Span> {
        let mut count = 0;
        let v = self.first.contains_two_units();
        if v.is_one() {
            count += 1;
        } else if v.is_two() {
            count += 2;
        }
        if count >= 2 {
            return v;
        }

        for (_, expr) in &self.next {
            match expr.contains_two_units() {
                AcceptOnlyOne::None => {}
                AcceptOnlyOne::One => {
                    count += 1;
                    if count >= 2 {
                        return AcceptOnlyOne::Two(expr.span());
                    }
                }
                AcceptOnlyOne::Two(s) => {
                    count += 2;
                    if count >= 2 {
                        return AcceptOnlyOne::Two(s);
                    }
                }
            }
        }

        match count {
            0 => AcceptOnlyOne::None,
            1 => AcceptOnlyOne::One,
            _ => AcceptOnlyOne::Two(self.first.span()),
        }
    }
}
