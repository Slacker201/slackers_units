use syn::{Error, Token, parse::Parse};

use crate::unit_system::{Expression, Number, TypeIdentifier};

impl Parse for Expression {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let multiplier = Number::parse(input)?;

        let mut units = vec![];

        let mut final_span = input.span();

        loop {
            let t = match TypeIdentifier::parse(input) {
                Ok(t) => t,
                Err(_) => {
                    break;
                }
            };

            let exponent = match input.parse::<Token![^]>() {
                Ok(_) => {
                    let num = Number::parse(input)?;
                    let the_i64 = num.to_i64().map_err(|e| {
                        match e.kind() {
                            std::num::IntErrorKind::Empty => {
                                Error::new(input.span(), format!("failed to convert {num} into an integer because it was somehow empty????????"))
                            },
                            std::num::IntErrorKind::InvalidDigit => {
                                Error::new(input.span(), format!("failed to convert {num} into an integer because it somehow had an invalid character????"))
                            },
                            std::num::IntErrorKind::PosOverflow => {
                                Error::new(input.span(), format!("{num} is too large for an i64"))
                            },
                            std::num::IntErrorKind::NegOverflow => {
                                Error::new(input.span(), format!("{num} is too small for an i64"))
                            },
                            std::num::IntErrorKind::Zero => unreachable!(),
                            _ => todo!(),
                        }
                    })?;
                    (the_i64, num.span)
                }
                Err(_) => (0, t.span),
            };
            final_span = exponent.1;
            units.push((t, exponent.0));
        }

        Ok(Self {
            span: multiplier.span.join(final_span).unwrap_or(input.span()),
            units: units.into_boxed_slice(),
            multiplier,

        })
    }
}
