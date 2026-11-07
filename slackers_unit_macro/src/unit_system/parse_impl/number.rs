use num_rational::Ratio;
use num_traits::{ConstOne, ConstZero, FromPrimitive};
use slk_tokenstream::TokenStream;
use syn::{Error, LitFloat, LitInt, parse::Parse};

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
            let suffix = litint.suffix().to_string();
            let rat = parse_ratio(litint.base10_digits().as_bytes());

            Ok(Self {
                ratio: rat,
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

    // Skip leading whitespace
    ts.skip_while(|b| b.is_ascii_whitespace());

    // Optional sign
    let negative = match ts.peek() {
        Some(b'-') => {
            ts.skip();
            true
        }
        Some(b'+') => {
            ts.skip();
            false
        }
        _ => false,
    };

    // Integer part
    let int_start = ts.mark();
    ts.skip_while(u8::is_ascii_digit);
    let int_end = ts.mark();

    // Decimal part
    let (frac_start, frac_end) = if ts.consume_if(|&b| b == b'.').is_some() {
        let start = ts.mark();
        ts.skip_while(u8::is_ascii_digit);
        (start, ts.mark())
    } else {
        (ts.mark(), ts.mark()) // empty
    };

    // Convert slices (safe because we only accepted ASCII digits)
    let integer = unsafe { std::str::from_utf8_unchecked(ts.slice_from_marks(int_start, int_end)) };
    let decimal =
        unsafe { std::str::from_utf8_unchecked(ts.slice_from_marks(frac_start, frac_end)) };

    // Parse numerator
    let mut numer = if integer.is_empty() && decimal.is_empty() {
        // Just a sign or empty → treat as 0
        <NumberRepr as ConstZero>::ZERO
    } else if integer.is_empty() {
        NumberRepr::ZERO
    } else {
        NumberRepr::from_str_radix(integer, 10).expect("digits only")
    };

    let scale = if decimal.is_empty() {
        <NumberRepr as ConstOne>::ONE
    } else {
        let scale = <NumberRepr as FromPrimitive>::from_u8(10u8)
            .unwrap()
            .pow(u32::try_from(decimal.len()).unwrap_or(u32::MAX));
        let frac = NumberRepr::from_str_radix(decimal, 10).expect("digits only");
        numer = numer * scale + frac;
        scale
    };

    if negative {
        numer = -numer;
    }

    Ratio::new(numer, scale)
}
