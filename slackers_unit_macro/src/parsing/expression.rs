use std::collections::HashMap;

use proc_macro2::Span;
use syn::{Error, Ident, LitFloat, LitInt, Token, parenthesized, parse::Parse};

use crate::{
    data::{
        Decimal, Id,
        parser_data::{EqOpt, EqualityExpression, Op},
        registries::NameRegistry,
    },
    validation::unit_graph::GraphNode,
};

impl EqOpt<Id> {
    fn get_children(&self) -> Box<[Id]> {
        let mut v = vec![];
        match self {
            EqOpt::Multiply { first, next, span } => {
                first.gc(&mut v);
                next.iter().for_each(|node| node.1.gc(&mut v));
            }
            EqOpt::Pow {
                lhs,
                pow: _,
                span: _,
            } => {
                lhs.gc(&mut v);
            }
            EqOpt::Atom {
                multiplier: _,
                unit: _,
                span: _,
            } => {}
        }
        v.into_boxed_slice()
    }

    fn gc(&self, v: &mut Vec<Id>) {
        match self {
            EqOpt::Multiply {
                first,
                next,
                span: _,
            } => {
                first.gc(v);
                next.iter().for_each(|node| node.1.gc(v));
            }
            EqOpt::Pow {
                lhs,
                pow: _,
                span: _,
            } => {
                lhs.gc(v);
            }
            EqOpt::Atom {
                multiplier: _,
                unit,
                span: _,
            } => {
                v.push(*unit);
            }
        }
    }
    fn try_get_vmult(&self, nodes: &HashMap<Id, GraphNode>) -> Result<(Decimal, Box<[i32]>), Id> {
        match self {
            EqOpt::Multiply {
                first,
                next,
                span: _,
            } => {
                let (mut f_mult, mut f_vec) = first.try_get_vmult(nodes)?;
                for (op, expr) in next.iter() {
                    let (op, (mult, vec)) = (*op, expr.try_get_vmult(nodes)?);
                    if op == Op::Mul {
                        f_mult *= mult;
                        f_vec.iter_mut().zip(vec).for_each(|(a, b)| *a += b);
                    } else {
                        f_mult /= mult;
                        f_vec.iter_mut().zip(vec).for_each(|(a, b)| *a -= b);
                    }
                }
                Ok((f_mult, f_vec))
            }
            EqOpt::Pow { lhs, pow, span: _ } => {
                let (o_mult, mut o_vec) = lhs.try_get_vmult(nodes)?;
                let mut mult = 1.0;
                if pow.is_negative() {
                } else {
                    o_vec.iter_mut().for_each(|v| *v *= pow);
                    for _ in 0..pow.abs() {
                        mult *= o_mult;
                    }
                }
                Ok((mult, o_vec))
            }
            EqOpt::Atom {
                multiplier,
                unit,
                span: _,
            } => {
                let node = nodes
                    .get(unit)
                    .expect("This should always be a valid id as well");
                let Some((mult, vec)) = &node.vmult else {
                    return Err(node.id);
                };
                Ok((*multiplier * *mult, vec.clone()))
            }
        }
    }
}

impl EqualityExpression<Id> {
    pub(crate) fn get_children(&self) -> Box<[Id]> {
        self.inner.get_children()
    }
    pub(crate) fn try_get_vmult(
        &self,
        nodes: &HashMap<Id, GraphNode>,
    ) -> Result<(Decimal, Box<[i32]>), Id> {
        self.inner.try_get_vmult(nodes)
    }
}

syn::custom_keyword!(equivalent);
syn::custom_keyword!(eq);

impl Parse for EqualityExpression<String> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start_span = if input.peek(eq) {
            input.parse::<eq>()?.span
        } else if input.peek(equivalent) {
            input.parse::<equivalent>()?.span
        } else {
            return Err(Error::new(input.span(), "Expected `eq` or `equivalent`"));
        };
        let _ = input.parse::<Token![=]>()?;

        let expr = EqOpt::parse_mul(input)?;
        let end_span = input.parse::<Token![,]>()?.span;
        Ok(Self {
            inner: expr,
            span: start_span.join(end_span).unwrap_or(start_span),
        })
    }
}

impl EqOpt<String> {
    pub fn to_ided(self, unit_registry: &NameRegistry) -> syn::Result<EqOpt<Id>> {
        let r = match self {
            EqOpt::Multiply { first, next, span } => EqOpt::Multiply {
                first: Box::new(first.to_ided(unit_registry)?),
                span,
                next: next
                    .into_iter()
                    .map(|(op, next)| next.to_ided(unit_registry).map(|q| (op, q)))
                    .collect::<Result<_, _>>()?,
            },
            EqOpt::Pow { lhs, pow, span } => EqOpt::Pow {
                lhs: Box::new(lhs.to_ided(unit_registry)?),
                pow,
                span,
            },
            EqOpt::Atom {
                multiplier,
                unit,
                span,
            } => EqOpt::Atom {
                multiplier,
                unit: unit_registry.try_get(&unit, span)?,
                span,
            },
        };
        Ok(r)
    }
}

impl EqOpt<String> {
    fn parse_mul(input: syn::parse::ParseStream) -> syn::Result<Self> {
        /*
            number = decimal | integer | fraction
            suffixed_num = number ident

            atom = number | suffixed_num | ident | '(' mul ')'

            mul = pow (op pow)*
            pow = atom ('^' integer)?

        */

        let first = Self::parse_pow(input)?;
        let start_span = first.span();
        let mut next = vec![];
        let mut end_span = start_span;
        while let Ok(op) = Self::parse_op(input) {
            let n = Self::parse_pow(input)?;
            end_span = n.span();
            next.push((op, n));
        }

        Ok(Self::Multiply {
            first: Box::new(first),
            next: next.into_boxed_slice(),
            span: start_span.join(end_span).unwrap_or(start_span),
        })
    }

    fn parse_pow(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let atom = Self::parse_atom(input)?;
        let (pow, span) = if input.parse::<Token![^]>().is_ok() {
            let litint = input.parse::<LitInt>()?;
            let span = atom.span().join(litint.span()).unwrap_or(atom.span());
            (litint.base10_parse::<i32>()?, span)
        } else {
            (1, atom.span())
        };

        Ok(Self::Pow {
            lhs: Box::new(atom),
            pow,
            span,
        })
    }

    fn parse_atom(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Some((multiplier, unit, span)) = Self::parse_atom_part(input)? {
            Ok(Self::Atom {
                multiplier,
                unit,
                span,
            })
        } else {
            let paren;
            let end_span = parenthesized!(paren in input).span.close();
            let mut s = Self::parse_mul(&paren)?;
            s.merge_span(end_span);
            Ok(s)
        }
    }

    fn parse_atom_part(
        input: syn::parse::ParseStream,
    ) -> syn::Result<Option<(Decimal, String, Span)>> {
        if input.peek(LitInt) {
            let first = input.parse::<LitInt>()?;
            if first.suffix().is_empty() && input.peek(Token![/]) && input.peek2(LitInt) {
                let _ = input.parse::<Token![/]>()?;
                let denom = input.parse::<LitInt>()?;
                let numer = first.base10_parse::<Decimal>()?;
                let true_denom = denom.base10_parse::<Decimal>()?;
                Ok(Some((
                    numer / true_denom,
                    denom.suffix().to_string(),
                    first.span().join(denom.span()).unwrap_or(first.span()),
                )))
            } else {
                Ok(Some((
                    first.base10_parse::<Decimal>()?,
                    first.suffix().to_string(),
                    first.span(),
                )))
            }
        } else if input.peek(LitFloat) {
            let first = input.parse::<LitFloat>()?;

            Ok(Some((
                first.base10_parse::<Decimal>()?,
                first.suffix().to_string(),
                first.span(),
            )))
        } else if input.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            Ok(Some((1.0, ident.to_string(), ident.span())))
        } else {
            Ok(None)
        }
    }

    fn parse_op(input: syn::parse::ParseStream) -> syn::Result<Op> {
        if input.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            Ok(Op::Mul)
        } else if input.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            Ok(Op::Div)
        } else {
            Err(Error::new(input.span(), "Expected `*` or `/`"))
        }
    }
}
