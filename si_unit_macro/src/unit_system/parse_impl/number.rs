use std::{collections::VecDeque, fmt::Display};

use proc_macro2::Span;
use syn::{Error, LitFloat, LitInt, parse::Parse};

use crate::unit_system::{Digit, Number};

impl Parse for Number {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitFloat) {
            let lit_float = input.parse::<LitFloat>().unwrap();

            if !lit_float.suffix().is_empty() {
                // TODO verify that unsuffixed floats in userland do not have f64 suffix
                Err(Error::new(input.span(), "Do not add a suffix"))
            } else {
                Self::parse_str(lit_float.base10_digits(), lit_float.span())
            }
        } else if input.peek(LitInt) {
            let lit_int = input.parse::<LitInt>().unwrap();

            if !lit_int.suffix().is_empty() {
                // TODO verify that unsuffixed ints in userland do not have i32 suffix
                Err(Error::new(input.span(), "Do not add a suffix"))
            } else {
                Self::parse_str(lit_int.base10_digits(), lit_int.span())
            }
        } else {
            Err(Error::new(input.span(), "Expected a number"))
        }
    }
}

impl Number {
    fn parse_str(s: &str, span: Span) -> syn::Result<Self> {
        let mut integer = VecDeque::new();
        let mut decimal = VecDeque::new();

        let mut current_vec = &mut integer;

        for c in s.chars() {
            if c == '.' {
                current_vec = &mut decimal;
                continue;
            }
            current_vec.push_front(
                Digit::from_char(c)
                    .ok_or_else(|| Error::new(span, format!("Somehow got a non-integer digit")))?,
            );
        }
        Ok(Self { integer, decimal, span })
    }

    pub(crate) fn to_i64(&self) -> Result<i64, std::num::ParseIntError> {
        let str_ver = self.to_string();

        str_ver.parse::<i64>()
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.integer.is_empty(), self.decimal.is_empty()) {
            (true, true) => {
                write!(f, "0")
            }
            (true, false) => {
                let (first, second) = self.decimal.as_slices();
                let mut j: String = first.iter().map(|c| c.to_string()).collect();
                j.extend(second.iter().map(|c| c.to_string()));
                write!(f, "0.{j}")
            }
            (false, true) => {
                let (first, second) = self.integer.as_slices();
                let mut j: String = first.iter().map(|c| c.to_string()).collect();
                j.extend(second.iter().map(|c| c.to_string()));
                write!(f, "{j}")
            }
            (false, false) => {
                let (first, second) = self.integer.as_slices();
                let mut int: String = first.iter().map(|c| c.to_string()).collect();
                int.extend(second.iter().map(|c| c.to_string()));
                let (first, second) = self.decimal.as_slices();
                let mut dec: String = first.iter().map(|c| c.to_string()).collect();
                dec.extend(second.iter().map(|c| c.to_string()));
                write!(f, "{int}.{dec}")
            }
        }
    }
}
