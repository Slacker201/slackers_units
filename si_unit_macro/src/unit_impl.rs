use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{Error, LitFloat, LitInt, parse::Parse};

use crate::{prefix_trie::PrefixTrie, unit_impl::unit_context::UnitContext};

mod unit_context;

pub(crate) fn unit_impl(tokens: TokenStream) -> TokenStream {
    let unit: FullUnit = match syn::parse2(tokens) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().to_token_stream()
    };

    let idents = unit.vector.iter().map(
        |value| {
            if *value == 0 {
                format_ident!("Z0")
            } else if *value < 0 {
                format_ident!("N{}", value.abs() as u64)
            } else {
                format_ident!("P{}", value.abs() as u64)
            }
        }
    );

    quote! {
        { 
            crate::Unit::<
            #(
                ::slackers_si_units::#idents,
            )*
            >::new(1.0)
        }
    }
}


struct FullUnit {
    vector: Box<[i64]>,
}

impl Parse for FullUnit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        
        let unit_context: UnitContext = input.parse()?;

        let mut unit_parse_trie = PrefixTrie::new();
        let mut prefix_parse_trie = PrefixTrie::new();

        for unit in &unit_context.units {
            unit_parse_trie.add_word(&unit.abbrev);
        }

        for prefix in &unit_context.prefixes {
            prefix_parse_trie.add_word(&prefix.abbrev);
        }

        let item = input.parse::<RType>()?;

        let vector = calc_unit_vector(item.suffix(), prefix_parse_trie, unit_parse_trie, unit_context).ok_or(
            Error::new(
                item.span(), 
                "Unknown suffix"
            )
        )?;

    
        Ok(FullUnit {
            vector
        })
    }
}

enum RType {
    LitInt(LitInt),
    LitFloat(LitFloat),
}

impl Parse for RType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitInt) {
            Ok(
                Self::LitInt(input.parse::<LitInt>()?)
            )
        } else if input.peek(LitFloat)  {
            Ok(
                Self::LitFloat(input.parse::<LitFloat>()?)
            )
        } else {
            eprintln!("Remainder: {:?}", input.to_string());
            Err(
                Error::new(
                    input.span(), 
                    "Expected integer or float"
                )
            )
        }
    }
}

impl RType {
    fn suffix(&self) -> &str {
        match self {
            RType::LitInt(lit_int) => lit_int.suffix(),
            RType::LitFloat(lit_float) => lit_float.suffix(),
        }
    }
    fn span(&self) -> Span {
        match self {
            RType::LitInt(lit_int) => lit_int.span(),
            RType::LitFloat(lit_float) => lit_float.span(),
        }
    }
}


fn calc_unit_vector(suffix: &str, prefix: PrefixTrie, unit: PrefixTrie, uc: UnitContext) -> Option<Box<[i64]>> {
    eprintln!("Suffix: {}", suffix);
    eprintln!("{:?}", unit);
    if let Some((_prefix, remainder)) = prefix.find_longest(suffix.as_bytes()) {
        if let Some((mut unit, remainder)) = unit.find_longest(remainder) {
            unit.reverse();
            if remainder.is_empty() {
                Some(
                    uc.units.iter().find(|v| v.abbrev.as_bytes() == &unit)?.vector.clone()
                )
            } else {
                None
            }
        } else {
            let (mut unit, remainder) = unit.find_longest(suffix.as_bytes())?;
            unit.reverse();
            if remainder.is_empty() {
                Some(
                    uc.units.iter().find(|v| v.abbrev.as_bytes() == unit)?.vector.clone()
                )
            } else {
                None
            }
        }
    } else {
        let (mut unit, remainder) = unit.find_longest(suffix.as_bytes())?;
        unit.reverse();
        if remainder.is_empty() {
            Some(
                uc.units.iter().find(|v| v.abbrev.as_bytes() == &unit)?.vector.clone()
            )
        } else {
            None
        }
    }

    
}

#[test]
fn t() {
    println!("{:?}", "m".as_bytes())
}