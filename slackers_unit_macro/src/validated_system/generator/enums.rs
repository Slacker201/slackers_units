use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{unit_impl::unit_str_parser::decimal_constructor::DecWrapper, validated_system::ValidatedSystem};

impl ValidatedSystem {
    pub(crate) fn generate_unit_prefix_enums(&self) -> TokenStream {
        let prefixable = self.units.iter().map(|v| v.prefixable);
        let dim_count = self.base_types.len();
        let unit_names = &self.unit_enum_names;
        let prefix_names = &self.prefix_enum_names;
        let dim_mapping: Box<[_]> = self.base_types_raw_vectors.iter().map(|v| DimMapping { items: v }).collect();
        let prefix_multi: Box<[_]> = self.prefixes.iter().map(|prefix| DecWrapper(prefix.multiplier.ratio)).collect();
        let int_repr = Self::get_integer_repr();

        quote! {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub(crate) enum UnitType {
                #(
                    #unit_names,
                )*
            }
            #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub(crate) enum Prefix {
                #(
                    #prefix_names,
                )*
                None,
            }

            impl UnitType {
                pub(crate) fn is_prefixable(&self) -> bool {
                    match self {
                        #(
                            UnitType::#unit_names => #prefixable,
                        )*
                    }
                }

                pub(crate) fn vector(&self) -> [i64; #dim_count] {
                    match self {
                        #(
                            UnitType::#unit_names => #dim_mapping,
                        )*
                    }
                }
            }

            impl Prefix {
                pub(crate) fn to_multiplier(&self) -> ::slackers_units::num_rational::Ratio<#int_repr> {
                    match self {
                        #(
                            Self::#prefix_names => #prefix_multi
                        )*

                        Self::None => ::slackers_units::num_rational::Ratio::from_integer(<#int_repr as ::slackers_units::num_traits::ConstOne>::ONE)
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct DimMapping<'a> {
    items: &'a [i64],
}

impl<'a> ToTokens for DimMapping<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let items = self.items;
        tokens.extend(
            quote! {
                [
                    #(
                        #items,
                    )*
                ]
            }
        );
    }
}