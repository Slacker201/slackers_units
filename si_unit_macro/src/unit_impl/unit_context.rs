use fastnum::{D512, decimal::Decimal};
use syn::{Ident, LitFloat, LitInt, Token, braced, bracketed, parenthesized, parse::Parse, punctuated::Punctuated};


#[derive(Debug)]
#[allow(unused)]
pub(crate) struct UnitContext {
    pub(crate) units: Box<[UnitPiece]>,
    pub(crate) prefixes: Box<[PrefixPiece]>,
}

impl Parse for UnitContext {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![const]>()?;
        input.parse::<Token![_]>()?;
        input.parse::<Token![:]>()?;
        let _unused;
        parenthesized!(_unused in input);
        input.parse::<Token![=]>()?;



        let content;
        braced!(content in input);
        
        let units = Punctuated::<UnitPiece, Token![,]>::parse_separated_nonempty(&content)?.into_iter().collect();
        if content.peek(Token![,]) {
            let _ = content.parse::<Token![,]>();
        }

        let prefixes = if content.peek(Token![~]) {
            let _ = content.parse::<Token![~]>()?;
            Punctuated::<PrefixPiece, Token![,]>::parse_terminated(&content)?.into_iter().collect()
        } else {
            Box::default()
        };

        Ok(
            Self { units, prefixes }
        )
    }
}
#[derive(Debug)]
#[allow(unused)]
pub(crate) struct UnitPiece {
    pub(crate) abbrev: String,
    pub(crate) vector: Box<[i64]>,
}

impl Parse for UnitPiece {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let abbrev = input.parse::<Ident>()?.to_string();
        let _ = input.parse::<Token![=]>()?;
        let _ = input.parse::<Token![>]>()?;
        let content;
        bracketed!(content in input);

        let vector = Punctuated::<LitInt, Token![,]>::parse_terminated(&content)?.into_iter()
            .map(|litint| {
                litint.base10_parse::<i64>()
            })
            .collect::<Result<_, _>>()?;

        Ok(
            Self { abbrev, vector }
        )
    }
}

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct PrefixPiece {
    pub(crate) abbrev: String,
    pub(crate) multiplier: D512,
}

impl Parse for PrefixPiece {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let abbrev = input.parse::<Ident>()?.to_string();
        let _ = input.parse::<Token![=]>()?;
        let _ = input.parse::<Token![>]>()?;

        let multiplier = input.parse::<LitFloat>().and_then(|litfloat| litfloat.base10_parse::<Decimal<8>>())?;


        Ok(
            Self { abbrev, multiplier }
        )
    }
}