use proc_macro2::Span;
use syn::{Error, Ident, braced};

use crate::{
    data::{
        Id,
        parser_data::{
            Aliases, EqOpt, EqualityExpression, PrefixGrouping, UnitDefinition, aliases,
            prefix_classes, unit,
        },
        registries::NameRegistry,
        unit_context::UnitContext,
    },
    parsing::expression::{eq, equivalent},
};

impl UnitDefinition<String> {
    pub fn parse(
        input: syn::parse::ParseStream,
        unit_registry: &mut NameRegistry,
    ) -> syn::Result<Self> {
        let start_span = input.parse::<unit>()?.span;
        let name = input.parse::<Ident>()?;
        let body;
        let end_span = braced!(body in input).span.close();
        let mut prefixable: Option<PrefixGrouping> = None;
        let mut alia: Option<Aliases> = None;
        let mut equiv: Option<EqualityExpression<String>> = None;
        loop {
            if body.peek(prefix_classes) {
                if let Some(pref) = prefixable {
                    return Err(Error::new(pref.span, "Expected at most 1 prefix class"));
                }
                prefixable = Some(body.parse::<PrefixGrouping>()?);
                continue;
            }
            if body.peek(eq) || body.peek(equivalent) {
                if let Some(eq) = equiv {
                    return Err(Error::new(
                        eq.span,
                        "Expected at most 1 equivalent expression",
                    ));
                }
                equiv = Some(body.parse::<EqualityExpression<String>>()?);
                continue;
            }
            if body.peek(aliases) {
                if let Some(alia) = alia {
                    return Err(Error::new(alia.span, "Expected at most 1 alias list"));
                }
                alia = Some(body.parse::<Aliases>()?);
                continue;
            }

            if body.is_empty() {
                break;
            }
            return Err(Error::new(body.span(), "Expected '}'"));
        }

        let id = unit_registry.get_id(name.to_string());
        let aliases = alia.unwrap_or(Aliases::new_empty());
        aliases.register_aliases(id, unit_registry)?;
        let prefixable = prefixable.unwrap_or(PrefixGrouping {
            allowed_groups: Box::new([Ident::new("All", Span::call_site())]),
            span: Span::call_site(),
        });

        Ok(Self {
            name,
            id,
            aliases,
            prefix_groups: prefixable,
            equality: equiv,
            span: start_span.join(end_span).unwrap_or(start_span),
        })
    }

    pub fn resolve_suffixes(self, registry: &UnitContext) -> syn::Result<UnitDefinition<Id>> {
        let equality = if let Some(r) = self.equality.map(|eq| eq.inner.resolve_suffixes(registry))
        {
            let m = r?;
            Some(EqualityExpression {
                span: m.span(),
                inner: m,
            })
        } else {
            None
        };
        Ok(UnitDefinition {
            name: self.name,
            id: self.id,
            aliases: self.aliases,
            prefix_groups: self.prefix_groups,
            equality,
            span: self.span,
        })
    }
}

impl<T> EqOpt<T> {
    pub fn span(&self) -> Span {
        match self {
            EqOpt::Multiply {
                first: _,
                next: _,
                span,
            }
            | EqOpt::Pow {
                lhs: _,
                pow: _,
                span,
            }
            | EqOpt::Atom {
                multiplier: _,
                unit: _,
                span,
            } => *span,
        }
    }
    pub fn merge_span(&mut self, other: Span) {
        match self {
            EqOpt::Multiply {
                first: _,
                next: _,
                span,
            }
            | EqOpt::Pow {
                lhs: _,
                pow: _,
                span,
            }
            | EqOpt::Atom {
                multiplier: _,
                unit: _,
                span,
            } => *span = span.join(other).unwrap_or(*span),
        }
    }
}
