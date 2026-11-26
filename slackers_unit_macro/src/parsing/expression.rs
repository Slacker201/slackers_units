use std::collections::HashMap;

use num_rational::Ratio;
use num_traits::{ConstZero, FromPrimitive};
use proc_macro2::Span;
use slk_tokenstream::TokenStream;
use syn::{Error, Ident, LitFloat, LitInt, Token, parenthesized, parse::Parse};

use crate::{
    data::{
        Decimal, DecimalPart, Id,
        parser_data::{EqOpt, EqualityExpression, Op},
        unit_context::UnitContext,
    },
    validation::unit_graph::GraphNode,
};

impl EqOpt<Id> {
    fn get_children(&self) -> Box<[Id]> {
        let mut v = vec![];
        match self {
            EqOpt::Multiply {
                first,
                next,
                span: _,
            } => {
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
                for (op, expr) in next {
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
                let mut mult = Decimal::ONE;
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

        let expr = EqOpt::parse_mul(input, true)?;
        let end_span = input.parse::<Token![,]>()?.span;
        Ok(Self {
            inner: expr,
            span: start_span.join(end_span).unwrap_or(start_span),
        })
    }
}

impl EqOpt<String> {
    pub fn resolve_suffixes(self, unit_context: &UnitContext) -> syn::Result<EqOpt<Id>> {
        let r = match self {
            EqOpt::Multiply { first, next, span } => EqOpt::Multiply {
                first: Box::new(first.resolve_suffixes(unit_context)?),
                span,
                next: next
                    .into_iter()
                    .map(|(op, next)| next.resolve_suffixes(unit_context).map(|q| (op, q)))
                    .collect::<Result<_, _>>()?,
            },
            EqOpt::Pow { lhs, pow, span } => EqOpt::Pow {
                lhs: Box::new(lhs.resolve_suffixes(unit_context)?),
                pow,
                span,
            },
            EqOpt::Atom {
                multiplier,
                unit,
                span,
            } => {
                let (new_multiplier, unit) = unit_context.parse_suffix(&unit, span)?;
                EqOpt::Atom {
                    multiplier: new_multiplier * multiplier,
                    unit,
                    span,
                }
            }
        };
        Ok(r)
    }
}

impl EqOpt<String> {
    pub fn parse_mul(input: syn::parse::ParseStream, allow_num: bool) -> syn::Result<Self> {
        /*
            number = decimal | integer | fraction
            suffixed_num = number ident

            atom = number | suffixed_num | ident | '(' mul ')'

            mul = pow (op pow)*
            pow = atom ('^' integer)?

        */

        let first = Self::parse_pow(input, allow_num)?;
        let start_span = first.span();
        let mut next = vec![];
        let mut end_span = start_span;
        while let Ok(op) = Self::parse_op(input) {
            let n = Self::parse_pow(input, allow_num)?;
            end_span = n.span();
            next.push((op, n));
        }

        Ok(Self::Multiply {
            first: Box::new(first),
            next: next.into_boxed_slice(),
            span: start_span.join(end_span).unwrap_or(start_span),
        })
    }

    fn parse_pow(input: syn::parse::ParseStream, allow_num: bool) -> syn::Result<Self> {
        let atom = Self::parse_atom(input, allow_num)?;
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

    fn parse_atom(input: syn::parse::ParseStream, allow_num: bool) -> syn::Result<Self> {
        if let Some((multiplier, unit, span)) = Self::parse_atom_part(input, allow_num)? {
            Ok(Self::Atom {
                multiplier,
                unit,
                span,
            })
        } else {
            let paren;
            let end_span = parenthesized!(paren in input).span.close();
            let mut s = Self::parse_mul(&paren, allow_num)?;
            s.merge_span(end_span);
            Ok(s)
        }
    }

    fn parse_atom_part(
        input: syn::parse::ParseStream,
        allow_num: bool,
    ) -> syn::Result<Option<(Decimal, String, Span)>> {
        if input.peek(LitInt) {
            let first = input.parse::<LitInt>()?;
            if !allow_num {
                return Err(Error::new(
                    first.span(),
                    format!("Expected identifier, found {first}"),
                ));
            }
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
                    first.base10_parse()?,
                    first.suffix().to_string(),
                    first.span(),
                )))
            }
        } else if input.peek(LitFloat) {
            let first = input.parse::<LitFloat>()?;
            if !allow_num {
                return Err(Error::new(
                    first.span(),
                    format!("Expected identifier, found {first}"),
                ));
            }
            Ok(Some((
                NumberHandler::handle_litfloat(&first)?,
                first.suffix().to_string(),
                first.span(),
            )))
        } else if input.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            Ok(Some((Decimal::ONE, ident.to_string(), ident.span())))
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

pub struct NumberHandler;

impl NumberHandler {
    pub fn handle_litfloat(litfloat: &LitFloat) -> syn::Result<Decimal> {
        let (number_portion, sci_not_mult) = Self::split_number_and_sci_float(litfloat)?;
        let mut ts = TokenStream::new(number_portion);
        let int =
            str::from_utf8(ts.consume_while(u8::is_ascii_digit)).expect("Safe by construction");
        let _ = ts.consume_if(|c| *c == b'.');
        let dec =
            str::from_utf8(ts.consume_while(u8::is_ascii_digit)).expect("Safe by construction");
        let dec_len = dec.trim_end_matches('0').len();

        let scale = DecimalPart::from_u8(10)
            .unwrap()
            .pow(u32::try_from(dec_len).unwrap());
        let int: DecimalPart = int.parse().map_err(|e| Error::new(litfloat.span(), e))?;
        let dec = if dec.is_empty() {
            DecimalPart::ZERO
        } else {
            dec.parse().map_err(|e| Error::new(litfloat.span(), e))?
        };

        let numer = int * scale + dec;

        let mut d = Decimal::new_raw(numer, scale);

        if let Some(sci_not) = sci_not_mult {
            let mut ratio = Ratio::from_integer(
                DecimalPart::from_u8(10)
                    .unwrap()
                    .pow(sci_not.abs().cast_unsigned()),
            );
            if sci_not.is_negative() {
                ratio = ratio.recip();
            }
            d *= ratio;
        }
        Ok(d)
    }

    fn split_number_and_sci_float(litfloat: &LitFloat) -> syn::Result<(&[u8], Option<i32>)> {
        if !litfloat.base10_digits().contains('e') {
            return Ok((litfloat.base10_digits().as_bytes(), None));
        }
        let mut ts = TokenStream::new(litfloat.base10_digits().as_bytes());
        let number_portion = ts.consume_while(|c| c.is_ascii_digit() || *c == b'.');
        let _ = ts.consume_if(|c| *c == b'e');
        let is_negative = ts.consume_if(|c| *c == b'-').is_some();
        let mut exponent = str::from_utf8(ts.consume_while(u8::is_ascii_digit))
            .expect("Safe by construction")
            .parse::<i32>()
            .map_err(|err| Error::new(litfloat.span(), err))?;

        if is_negative {
            exponent = -exponent;
        }
        Ok((number_portion, Some(exponent)))
    }
}
