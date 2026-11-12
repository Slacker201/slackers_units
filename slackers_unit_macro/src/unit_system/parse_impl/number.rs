use num_rational::Ratio;
use num_traits::{ConstZero, FromPrimitive};
use slk_tokenstream::TokenStream;
use syn::{Error, LitFloat, LitInt, Token, parse::Parse};

use crate::unit_system::{Number, NumberRepr};

impl Parse for Number {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitFloat) {
            let litfloat = input.parse::<LitFloat>()?;
            let suffix = litfloat.suffix().to_string();
            let ratio = parse_ratio(litfloat.base10_digits().as_bytes());

            Ok(Self {
                ratio,
                suffix,
                span: litfloat.span(),
            })
        } else if input.peek(LitInt) {
            let litint = input.parse::<LitInt>()?;
            let (numerator, _) = parse_integer(litint.base10_digits().as_bytes());
            let (ratio, suffix) =
                if litint.suffix().is_empty() && input.peek(Token![/]) && input.peek2(LitInt) {
                    let _ = input.parse::<Token![/]>()?;
                    let denom_lit = input.parse::<LitInt>()?;
                    let (denom, _) = parse_integer(denom_lit.base10_digits().as_bytes());
                    (Ratio::new(numerator, denom), denom_lit.suffix().to_string())
                } else {
                    (Ratio::from_integer(numerator), litint.suffix().to_string())
                };

            Ok(Self {
                ratio,
                suffix,
                span: litint.span(),
            })
        } else {
            Err(Error::new(input.span(), "Expected a number"))
        }
    }
}

fn parse_ratio(value: &[u8]) -> Ratio<NumberRepr> {
    let mut ts = TokenStream::new(value);
    ts.skip_while(|b| b.is_ascii_whitespace());
    let is_negative = ts.consume_if(|c| *c == b'-').is_some();
    let (numer, remainder) = parse_integer(ts.remainder());
    let mut ts = TokenStream::new(remainder);
    ts.consume_if(|c| *c == b'.');
    let post_numer_len = ts.tokens_remaining();
    let (decimal, remainder) = parse_integer(ts.remainder());
    let dec_len = post_numer_len - remainder.len();
    let denom = NumberRepr::from_i32(10).unwrap().pow(dec_len as u32);
    let mut numer = numer * denom + decimal;

    if is_negative {
        numer = -numer;
    }

    Ratio::new(numer, denom)
}

fn parse_integer(bytes: &[u8]) -> (NumberRepr, &[u8]) {
    let mut ts = TokenStream::new(bytes);
    ts.skip_while(|c| c.is_ascii_whitespace());
    let is_negative = ts.consume_if(|c| *c == b'-').is_some();
    ts.skip_while(|c| c.is_ascii_whitespace());
    let start = ts.mark();
    ts.skip_while(|c| c.is_ascii_digit());
    let number_bytes = ts.slice_from_marks(start, ts.mark());
    if number_bytes.is_empty() {
        return (NumberRepr::ZERO, ts.remainder());
    }
    let mut number = NumberRepr::from_ascii_radix(number_bytes, 10).expect("Number too large");
    if is_negative {
        number = -number;
    }
    (number, ts.remainder())
}
