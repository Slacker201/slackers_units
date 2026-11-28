use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitByteStr;

use crate::data::{unit_context::UnitContext, validated_sys::ValidatedSystem};

impl ValidatedSystem {
    pub fn generate_unit_macro_impl(&self) -> TokenStream {
        let sys_hash = LitByteStr::new(&self.sys_hash, Span::call_site());
        debug_assert_eq!(
            UnitContext::deserialize(&self.unit_context.serialize()),
            self.unit_context
        );
        let byte_str = LitByteStr::new(&self.unit_context.serialize(), Span::call_site());

        quote! {
            /// Generates a typed unit literal
            ///
            /// # Examples
            ///
            /// ```rust,ignore
            /// slackers_units::system! {
            ///     base Meter;
            ///     unit Meter {
            ///         aliases = [m],
            ///     }
            ///
            ///     prefix Milli {
            ///         aliases = [m, milli],
            ///         mult = 1e-3,
            ///     }
            /// }
            ///
            /// let meter = unit!(1m);
            /// let millimeter = unit!(1m);
            /// let scale = unit!(1e3);
            ///
            /// assert_eq!(meter, millimeter * scale);
            /// ```
            macro_rules! unit {
                ($($data: tt)*) => {
                    ::slackers_units::__unit_impl!(#sys_hash #byte_str $($data)*)
                }
            }

            pub(crate) use unit;
        }
    }
}
