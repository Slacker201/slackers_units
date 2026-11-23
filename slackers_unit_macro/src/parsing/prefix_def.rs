use syn::{Error, Ident, LitFloat, LitInt, Token, braced, parse::Parse};

use crate::data::{
    Decimal,
    parser_data::{
        Aliases, PrefixDefinition, PrefixGrouping, aliases, mult, multiplier, prefix,
        prefix_classes,
    },
};

impl Parse for PrefixDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start_span = input.parse::<prefix>()?.span;
        let name = input.parse::<Ident>()?;

        let body;
        let end_span = braced!(body in input).span.close();

        let mut sm = None;
        let mut p_classes = None;
        let mut alia = Aliases::new_empty();

        loop {
            if body.peek(mult) || body.peek(multiplier) && sm.is_none() {
                if body.peek(mult) {
                    let _ = body.parse::<mult>()?;
                } else {
                    let _ = body.parse::<multiplier>()?;
                }
                let _ = body.parse::<Token![=]>()?;
                sm = Some(Self::parse_number(&body)?);
                let _ = body.parse::<Token![,]>()?;
                continue;
            }
            if body.peek(aliases) {
                alia = Aliases::parse(&body)?;

                continue;
            }
            if body.peek(prefix_classes) {
                p_classes = Some(PrefixGrouping::parse(&body)?);
            }

            if body.is_empty() {
                break;
            } else {
                return Err(Error::new(body.span(), "Expected `}`"));
            }
        }

        Ok(Self {
            name,
            aliases: alia,
            groups: p_classes.unwrap_or(PrefixGrouping::new_with_all()),
            span: start_span.join(end_span).unwrap_or(start_span),
        })
    }
}

impl PrefixDefinition {
    fn parse_number(input: syn::parse::ParseStream) -> syn::Result<Decimal> {
        if input.peek(LitInt) {
            let first = input.parse::<LitInt>()?;

            if input.peek(Token![/]) && input.peek2(LitInt) {
                let _ = input.parse::<Token![/]>()?;
                let second = input.parse::<LitInt>()?;

                if first.suffix().is_empty() && second.suffix().is_empty() {
                    Ok(first.base10_parse::<Decimal>()? / second.base10_parse::<Decimal>()?)
                } else {
                    Err(Error::new(
                        first.span().join(second.span()).unwrap_or(first.span()),
                        "Fractional numbers do not support suffixes",
                    ))
                }
            } else {
                first.base10_parse()
            }
        } else if input.peek(LitFloat) {
            let litfloat = input.parse::<LitFloat>()?;
            if litfloat.suffix().is_empty() {
                litfloat.base10_parse()
            } else {
                Err(Error::new(
                    litfloat.span(),
                    "Prefix multipliers must be dimensionless",
                ))
            }
        } else {
            Err(Error::new(input.span(), "Expected number or fraction"))
        }
    }
}
