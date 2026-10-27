use std::{array::from_fn, collections::{HashSet}};

use proc_macro2::Span;
use syn::{Error, Token, parse::Parse};

use crate::{System, unit_system::{BaseTypeKind, PrefixDeclaration, TypeIdentifier, UnitDeclaration}};


const EXPRECTED_ERROR_MESSAGE: &str = "\"time\", \"distance\", \"mass\", \"electric_current\", \"temperature\", \"amount\", and \"luminous_intensity\"";



impl Parse for System {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        

        let mut unit_declarations = Vec::new();
        let mut prefix_declarations = Vec::new();
        let mut base_types = vec![];

        loop {
            match input.parse::<UnitDeclaration>() {
                Ok(v) => {
                    unit_declarations.push(v);
                    continue;
                },
                Err(_) => {},
            }

            match input.parse::<PrefixDeclaration>() {
                Ok(v) => {
                    prefix_declarations.push(v);
                    continue;
                },
                Err(_) => {},
            }

            match input.parse::<BaseTypeDeclaration>() {
                Ok(v) => {
                    base_types.push(v);
                    continue;
                },
                Err(_) => {},
            }

            if !input.is_empty() {
                return Err(
                    Error::new(input.span(),
                    format!("Expected end of input, found {}", input.to_string())
                    )
                );
            }

            break;
        }
        



        if base_types.len() > 7 {
            return Err(
                Error::new(base_types[7].span, format!("Expected exactly 7 base type declarations, found {}", base_types.len()))
            );
        }

        if base_types.len() < 7 {
            return Err(
                Error::new(input.span(), format!("Expected exactly 7 base type declarations, found {}", base_types.len()))
            );
        }

        let mut seen = HashSet::new();

        for base in &base_types {
            let duplicate = seen.insert(base.base_type_kind);

            if duplicate {
                return Err(
                    Error::new(base.span, format!("Found duplicate {} declarations. Please declare each of {EXPRECTED_ERROR_MESSAGE}", base.base_type_kind))
                );
            }
        }

        let base_types: [_; 7] = from_fn(|i| core::mem::replace(&mut base_types[i].ty, TypeIdentifier::default()));

        
        Ok(
            Self { units: unit_declarations.into_boxed_slice(), prefixes: prefix_declarations.into_boxed_slice(), base_types }
        )
    }
}


struct BaseTypeDeclaration {
    base_type_kind: BaseTypeKind,

    ty: TypeIdentifier,

    span: Span,
}

impl Parse for BaseTypeDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

        let start_span = input.span();
        let base_type_kind = input.parse::<BaseTypeKind>()?;
        let _ = input.parse::<Token![:]>()?;
        let end_span = input.span();
        let ty = input.parse::<TypeIdentifier>()?;

        

        Ok(
            Self { base_type_kind, ty, span: start_span.join(end_span).unwrap() }
        )
    }
}