use fastnum::decimal::Decimal;
use syn::{Error, LitFloat, LitInt, parse::Parse};

use crate::unit_system::Number;

impl Parse for Number {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitFloat) {
            let litfloat = input.parse::<LitFloat>()?;
            let suffix = litfloat.suffix().to_string();
            let dec = litfloat.base10_parse::<Decimal<8>>()?;

            Ok(Self {
                value: dec,
                suffix,
                span: litfloat.span(),
            })
        } else if input.peek(LitInt) {
            let litint = input.parse::<LitInt>()?;
            let suffix = litint.suffix().to_string();
            let dec = litint.base10_parse::<Decimal<8>>()?;

            Ok(Self {
                value: dec,
                suffix,
                span: litint.span(),
            })
        } else {
            Err(Error::new(input.span(), "Expected a number"))
        }
    }
}
