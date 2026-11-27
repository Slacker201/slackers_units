use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

use crate::data::validated_sys::ValidatedSystem;

macro_rules! gen_vec_change_op {
    ($operand_1: ident, $operand_2: ident, $result: ident, $pd_names: ident, $value_op: ident, $value_op_fn: ident, $value_op_sym: tt, $typenum_op: ident, ) => {
        quote! {
            impl<
                #(
                    #$operand_1: ::typenum::Integer + core::ops::$typenum_op<#$operand_2, Output = #$result>,
                    #$operand_2: ::typenum::Integer,
                    #$result: ::typenum::Integer,
                )*
                B: core::ops::$value_op<B, Output=B>,
            > core::ops::$value_op<Unit<
                #(#$operand_2,)*
                B,
            >> for Unit<
                #(#$operand_1,)*
                B,
            > {
                type Output = Unit<#(#$result,)* B,>;

                fn $value_op_fn(self, other: Unit<#(#$operand_2,)* B>) -> Self::Output {
                    Unit::<#(#$result,)* B,> {
                        #(
                            #$pd_names: core::marker::PhantomData,
                        )*
                        value: self.value $value_op_sym other.value,
                    }
                }
            }
        }
    };
}
macro_rules! gen_norm_op {
    ($constrained: ident, $ud_names: ident, $pd_names: ident, $value_op: ident, $value_op_fn: ident, $value_op_sym: tt) => {
        quote! {
            impl<
                #$constrained
                B: core::ops::$value_op<B, Output=B>,
            > core::ops::$value_op<Unit<
                #$ud_names
                B,
            >> for Unit<
                #$ud_names
                B,
            > {
                type Output = Unit<#$ud_names B,>;

                fn $value_op_fn(self, other: Unit<#$ud_names B>) -> Self::Output {
                    Unit::<#$ud_names B,> {
                        #(
                            #$pd_names: core::marker::PhantomData,
                        )*
                        value: self.value $value_op_sym other.value,
                    }
                }
            }
        }
    };
}

impl ValidatedSystem {
    pub fn generate_unit_definition(&self) -> TokenStream {
        let constrained = UnitVector::<true>(&self.unit_generic_names);
        let ud_pd_names = &self.unit_generic_names;
        let pd_names = &self.unit_phantom_data_names;
        quote! {
            #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
            pub(crate) struct Unit<
                #constrained
                B,
            > {
                #(
                    #pd_names: core::marker::PhantomData<#ud_pd_names>,
                )*
                value: B,
            }
        }
    }
    pub fn generate_unit_arithmatic_impl(&self) -> TokenStream {
        let (operand_1, operand_2, result) = &self.unit_arith_generic_names;
        let constrained = UnitVector::<true>(&self.unit_generic_names);
        let ud_names = UnitVector::<false>(&self.unit_generic_names);
        let pd_names = &self.unit_phantom_data_names;
        let mut ts = TokenStream::new();
        ts.extend(gen_vec_change_op!(
            operand_1,
            operand_2,
            result,
            pd_names,
            Mul,
            mul,
            *,
            Add,
        ));
        ts.extend(gen_vec_change_op!(
            operand_1,
            operand_2,
            result,
            pd_names,
            Div,
            div,
            /,
            Sub,
        ));
        ts.extend(gen_norm_op!(
            constrained,
            ud_names,
            pd_names,
            Add,
            add,
            +
        ));
        ts.extend(gen_norm_op!(
            constrained,
            ud_names,
            pd_names,
            Sub,
            sub,
            -
        ));
        ts
    }

    pub fn generate_new(&self) -> TokenStream {
        let constrained = UnitVector::<true>(&self.unit_generic_names);
        let ud_names = UnitVector::<false>(&self.unit_generic_names);
        let pd_names = &self.unit_phantom_data_names;

        quote! {
            impl<
            #constrained
            B> Unit<#ud_names B> {
                pub(crate) fn new(value: B) -> Self {
                    Self {
                        #(
                            #pd_names: core::marker::PhantomData,
                        )*
                        value,
                    }
                }
            }
        }
    }

    pub fn generate_unit_debug(&self) -> TokenStream {
        // TODO incorporate unit vector
        let constrained = UnitVector::<true>(&self.unit_generic_names);
        let ud_names = UnitVector::<false>(&self.unit_generic_names);
        let ud_names_iter = &self.unit_generic_names;
        quote! {
            impl<
                #constrained
                B: core::fmt::Debug,
            > core::fmt::Debug for Unit<#ud_names B> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let vector = [
                        #(
                            #ud_names_iter::to_i32(),
                        )*
                    ];
                    write!(f, "{:?} {vector:?}", self.value)
                }
            }
        }
    }
}

struct UnitVector<'a, const SHOULD_CONSTRAIN: bool>(&'a [Ident]);

impl<const SHOULD_CONSTRAIN: bool> ToTokens for UnitVector<'_, SHOULD_CONSTRAIN> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let data = &self.0;
        if SHOULD_CONSTRAIN {
            tokens.extend(quote! {
                #(
                    #data: ::slackers_units::Integer,
                )*
            });
        } else {
            tokens.extend(quote! {
                #(
                    #data,
                )*
            });
        }
    }
}
