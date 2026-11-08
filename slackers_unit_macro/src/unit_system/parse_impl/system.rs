use module_path_extractor::find_module_path;
use proc_macro::Span;
use syn::{Error, Ident, Token, parse::Parse};

use crate::{
    System,
    unit_system::{BaseTypeDeclaration, PrefixDeclaration, TypeIdentifier, UnitDeclaration},
};

impl Parse for System {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut unit_declarations = Vec::new();
        let mut prefix_declarations = Vec::new();
        let mut base_types = vec![];

        let start_span = input.span();
        let mut end_span = input.span();

        loop {
            if input.peek(super::kw::unit) {
                let i = input.parse::<UnitDeclaration>()?;
                end_span = i.span;
                unit_declarations.push(i);
                continue;
            }
            if input.peek(super::kw::prefix) {
                let i = input.parse::<PrefixDeclaration>()?;
                end_span = i.span;
                prefix_declarations.push(i);
                continue;
            }
            if input.peek(Ident) {
                let i = input.parse::<BaseTypeDeclaration>()?;
                end_span = i.span;
                base_types.push(i);
                continue;
            }

            if input.is_empty() {
                break;
            }
            return Err(Error::new(
                input.span(),
                format!("Expected end of input, got {input}"),
            ));
        }

        let call_site_span = Span::call_site();
        let call_site = match find_module_path(&call_site_span.file(), call_site_span.line()) {
            Some(call_site) => call_site,
            None => {
                "crate".to_string()
            }
        };

        Ok(Self {
            span: start_span.join(end_span).unwrap_or(input.span()),
            units: unit_declarations.into_boxed_slice(),
            prefixes: prefix_declarations.into_boxed_slice(),
            base_types: base_types.into_boxed_slice(),
            call_site,
        })
    }
}

impl Parse for BaseTypeDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let start_span = input.parse::<super::kw::basetype>()?.span;
        let _ = input.parse::<Token![:]>()?;
        let ty = input.parse::<TypeIdentifier>()?;
        let end_span = input.parse::<Token![;]>()?.span;

        Ok(Self {
            ty,
            span: start_span.join(end_span).unwrap_or(input.span()),
        })
    }
}
