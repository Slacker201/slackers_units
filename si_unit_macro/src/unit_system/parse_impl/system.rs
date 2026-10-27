use syn::{Error, Token, parse::Parse};

use crate::{
    System, unit_system::{BaseTypeDeclaration, BaseTypeKind, PrefixDeclaration, TypeIdentifier, UnitDeclaration},
};

impl Parse for System {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut unit_declarations = Vec::new();
        let mut prefix_declarations = Vec::new();
        let mut base_types = vec![];

        let start_span = input.span();
        let mut end_span = input.span();

        loop {
            match input.parse::<UnitDeclaration>() {
                Ok(v) => {
                    end_span = v.span;
                    unit_declarations.push(v);
                    continue;
                }
                Err(_) => {}
            }

            match input.parse::<PrefixDeclaration>() {
                Ok(v) => {
                    end_span = v.span;
                    prefix_declarations.push(v);
                    continue;
                }
                Err(_) => {}
            }

            match input.parse::<BaseTypeDeclaration>() {
                Ok(v) => {
                    end_span = v.span;
                    base_types.push(v);
                    continue;
                }
                Err(_) => {}
            }

            if !input.is_empty() {
                return Err(Error::new(
                    input.span(),
                    format!("Expected end of input, found {}", input.to_string()),
                ));
            }

            break;
        }

        Ok(Self {
            span: start_span.join(end_span).unwrap_or(input.span()),
            units: unit_declarations.into_boxed_slice(),
            prefixes: prefix_declarations.into_boxed_slice(),
            base_types: base_types.into_boxed_slice(),
        })
    }
}

impl Parse for BaseTypeDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start_span = input.span();
        let base_type_kind = input.parse::<BaseTypeKind>()?;
        let _ = input.parse::<Token![:]>()?;
        let end_span = input.span();
        let ty = input.parse::<TypeIdentifier>()?;
        let _ = input.parse::<Token![;]>()?;

        Ok(Self {
            base_type_kind,
            ty,
            span: start_span.join(end_span).unwrap_or(input.span()),
        })
    }
}
