use proc_macro2::TokenStream;
use quote::quote;

use crate::{validated_system::ValidatedSystem};

mod map_gen;

impl ValidatedSystem {
    pub(crate) fn generate_runtime_parser(&self) -> TokenStream {
        let ctrie_enum = Self::construct_const_trie_enum();
        let maps = self.construct_parser_maps();
        let find_longest_trait = Self::construct_trait();
        let find_longest_impl = Self::construct_find_longest_impl();
        let error_enums = self.construct_error_enums();
        let suffix_parser = Self::construct_suffix_parser();
        let ratio_parser = Self::construct_ratio_parser();
        let parser = self.construct_parser();

        quote! {
            #ctrie_enum
            #maps
            #find_longest_trait
            #find_longest_impl
            #suffix_parser
            #error_enums
            #ratio_parser
            #parser
        }
    }

    fn construct_error_enums(&self) -> TokenStream {
        let dim_count = self.base_types.len();

        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub(crate) enum ParseResult {
                PrefixUnit(Prefix, UnitType),
                Unit(UnitType),
            }
            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub(crate) enum ParseError {
                UnknownUnit(Box<[u8]>),
                NotPrefixable(Box<[u8]>),
                FoundTrailingBytes(Box<[u8]>),
                MismatchedDimensions {
                    expected: [i64; #dim_count],
                    got: [i64; #dim_count],
                },
            }

            impl core::fmt::Debug for ParseError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        Self::UnknownUnit(arg0) => {
                            let arg0 = String::from_utf8_lossy(arg0);
                            f.debug_tuple("UnknownUnit").field(&arg0).finish()
                        },
                        Self::NotPrefixable(arg0) => {
                            let arg0 = String::from_utf8_lossy(arg0);
                            f.debug_tuple("NotPrefixable").field(&arg0).finish()
                        },
                        Self::FoundTrailingBytes(arg0) => {
                            let arg0 = String::from_utf8_lossy(arg0);
                            f.debug_tuple("FoundTrailingBytes").field(&arg0).finish()
                        },
                        Self::MismatchedDimensions { expected, got } => {
                            f.debug_struct("MismatchedDimensions").field("expected", expected)
                                .field("got", got)
                                .finish()
                        }
                    }
                }
            }

            #[derive(Debug, PartialEq)]
            enum ErrorOpt {
                NotPrefixable(Box<[u8]>),
                TrailingBytes(Box<[u8]>),
                None,
            }

            impl TryInto<ParseError> for ErrorOpt {
                type Error = ();

                fn try_into(self) -> Result<ParseError, Self::Error> {
                    match self {
                        ErrorOpt::NotPrefixable(items) => Ok(ParseError::NotPrefixable(items)),
                        ErrorOpt::TrailingBytes(items) => Ok(ParseError::FoundTrailingBytes(items)),
                        ErrorOpt::None => Err(()),
                    }
                }
            }

            impl ErrorOpt {
                pub(crate) fn set_if_none<F: Fn() -> ErrorOpt>(&mut self, f: F) {
                    if Self::None == *self {
                        *self = f();
                    }
                }
            }
        }
    }

    fn construct_suffix_parser() -> TokenStream {
        quote! {
            pub(crate) struct RuntimeParser;

            impl RuntimeParser {
                fn parse_suffix(suffix_bytes: &[u8]) -> Result<(ParseResult, &[u8]), ParseError> {
                    let mut error = ErrorOpt::None;
                    if let Some((prefix, remainder)) = PREFIX_MAP.find_longest(suffix_bytes)
                        && let Some((unit, unit_remainder)) = UNIT_MAP.find_longest(remainder) {
                        if unit.is_prefixable() {
                            return Ok((ParseResult::PrefixUnit(prefix, unit), unit_remainder));
                        } else {
                            error = ErrorOpt::NotPrefixable(remainder.into())
                        }
                    }
                    if let Some((unit, remainder)) = UNIT_MAP.find_longest(suffix_bytes) {
                        
                        return Ok((ParseResult::Unit(unit), remainder))
                            
                    }
                    Err(error.try_into().unwrap_or_else(|_| ParseError::UnknownUnit(suffix_bytes.into())))
                }
            }
        }
    }

    fn construct_ratio_parser() -> TokenStream {
        let repr = Self::get_integer_repr();
        quote! {
            type NumberRepr = #repr;
            impl RuntimeParser {
                fn parse_ratio(value: &[u8]) -> (::slackers_units::num_rational::Ratio<NumberRepr>, &[u8]) {
                    let mut ts = ::slackers_units::slk_tokenstream::TokenStream::new(value);

                    ts.skip_while(|b| b.is_ascii_whitespace());

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
                    let int_start = ts.mark();
                    ts.skip_while(|b| b.is_ascii_digit());
                    let int_end = ts.mark();

                    let (frac_start, frac_end) = if ts.consume_if(|&b| b == b'.').is_some() {
                        let start = ts.mark();
                        ts.skip_while(|b| b.is_ascii_digit());
                        (start, ts.mark())
                    } else {
                        (ts.mark(), ts.mark())
                    };

                    let integer = unsafe { std::str::from_utf8_unchecked(ts.slice_from_marks(int_start, int_end)) };
                    let decimal = unsafe { std::str::from_utf8_unchecked(ts.slice_from_marks(frac_start, frac_end)) };

                    let mut numer = if integer.is_empty() && decimal.is_empty() {
                        <NumberRepr as ::slackers_units::num_traits::ConstZero>::ZERO
                    } else if integer.is_empty() {
                        <NumberRepr as ::slackers_units::num_traits::ConstZero>::ZERO
                    } else {
                        NumberRepr::from_str_radix(integer, 10).expect("digits only")
                    };

                    let scale = if decimal.is_empty() {
                        <NumberRepr as ::slackers_units::num_traits::ConstOne>::ONE
                    } else {
                        let scale = <NumberRepr as ::slackers_units::num_traits::FromPrimitive>::from_u8(10u8).unwrap().pow(decimal.len() as u32);
                        let frac = NumberRepr::from_str_radix(decimal, 10).expect("digits only");
                        numer = numer * &scale + frac;
                        scale
                    };

                    if negative {
                        numer = -numer;
                    }

                    let ratio = slackers_units::num_rational::Ratio::new(numer, scale);
                    (ratio, &value[ts.cursor()..])
                }
            }
        }
    }

    fn construct_parser(&self) -> TokenStream {
        let dim_count = self.base_types.len();
        quote! {
            impl RuntimeParser {
                pub(crate) fn parse<U>(lit: &str) -> Result<U, ParseError> 
                where
                    U: UnitTrait,
                {

                    macro_rules! handle_parse_result {
                        ($pr: ident, $ratio: ident, $vector: ident) => {
                            match $pr {
                                ParseResult::PrefixUnit(prefix, unit_type) => {
                                    $ratio *= prefix.to_multiplier();
                                    for (vector_part, new_vec_part) in $vector.iter_mut().zip(unit_type.vector()) {
                                        *vector_part += new_vec_part;
                                    }
                                },
                                ParseResult::Unit(unit_type) => {
                                    for (vector_part, new_vec_part) in $vector.iter_mut().zip(unit_type.vector()) {
                                        *vector_part += new_vec_part;
                                    }
                                },
                            }
                        };
                    }



                    let (mut ratio, mut remainder) = Self::parse_ratio(lit.as_bytes());
                    let mut vector = [0i64; #dim_count];

                    let mut ts = ::slackers_units::slk_tokenstream::TokenStream::new(remainder);
                    ts.skip_while(|b| b.is_ascii_whitespace());
                    let start = ts.cursor();
                    remainder = &remainder[start..];
                    if let Ok((parse_result, remainder2)) = Self::parse_suffix(remainder) {
                        remainder = remainder2;
                        handle_parse_result!(parse_result, ratio, vector);
                    }
                    ts = ::slackers_units::slk_tokenstream::TokenStream::new(remainder);
                    loop {
                        ts.skip_while(|b| b.is_ascii_whitespace());
                        let Some(sign) = ts.consume_if(|c| matches!(c, b'*' | b'/')).copied() else {
                            break;
                        };
                        ts.skip_while(|b| b.is_ascii_whitespace());
                        let start = ts.cursor();
                        remainder = &remainder[start..];

                        match sign {
                            b'*' => {
                                let (parse_result, remainder2) = Self::parse_suffix(remainder)?;
                                remainder = remainder2;
                                handle_parse_result!(parse_result, ratio, vector);
                            }
                            b'/' => {
                                let (parse_result, remainder2) = Self::parse_suffix(remainder)?;
                                remainder = remainder2;
                                handle_parse_result!(parse_result, ratio, vector);
                            }
                            _ => unreachable!(),
                        }
                        ts = ::slackers_units::slk_tokenstream::TokenStream::new(remainder);
                    }

                    if ts.is_eof() {
                        if U::vector() == vector {
                            return Ok(
                                U::new(<U::Underlying as ::slackers_units::conversion::Conversion>::convert(ratio))
                            )
                        } else {
                            return Err(
                                ParseError::MismatchedDimensions{
                                    expected: U::vector(),
                                    got: vector,
                                }
                            )
                        }
                    } else {
                        let start = ts.mark();
                        ts.advance(usize::MAX);
                        let remainder = ts.slice_from_marks(start, ts.mark());

                        return Err(
                            ParseError::FoundTrailingBytes(
                                remainder.into()
                            )
                        )
                    }
                }
            }
        }
    }
}