use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitByteStr;

use crate::data::validated_sys::ValidatedSystem;

impl ValidatedSystem {
    pub fn generate_ut_macro_impl(&self) -> TokenStream {
        let sys_hash = LitByteStr::new(&self.sys_hash, Span::call_site());
        let byte_str = LitByteStr::new(&self.unit_context.serialize(), Span::call_site());

        quote! {
            /// Generates the type for the provided expression
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
            /// let meter: ut!(m) = unit!(1m);
            /// let millimeter: ut!(m) = unit!(1m);
            /// let scale /* TODO implement support for dimensionless types */ = unit!(1e3);
            ///
            /// assert_eq!(meter, millimeter * scale);
            /// ```
            macro_rules! ut {
                ($($data: tt)*) => {
                    ::slackers_units::__ut_impl!(#sys_hash #byte_str $($data)*)
                }
            }

            pub(crate) use ut;
        }
    }
}
