use proc_macro2::TokenStream;
use quote::quote;

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
    ($ud_generic_name: ident, $pd_names: ident, $value_op: ident, $value_op_fn: ident, $value_op_sym: tt) => {
        quote! {
            impl<
                #(
                    #$ud_generic_name,
                )*
                B: core::ops::$value_op<B, Output=B>,
            > core::ops::$value_op<Unit<
                #(#$ud_generic_name,)*
                B,
            >> for Unit<
                #(#$ud_generic_name,)*
                B,
            > {
                type Output = Unit<#(#$ud_generic_name,)* B,>;

                fn $value_op_fn(self, other: Unit<#(#$ud_generic_name,)* B>) -> Self::Output {
                    Unit::<#(#$ud_generic_name,)* B,> {
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
        let ug_names = &self.unit_generic_names;
        let pd_names = &self.unit_phantom_data_names;
        quote! {
            pub(crate) struct Unit<
                #(
                    #ug_names,
                )*
                B,
            > {
                #(
                    #pd_names: core::marker::PhantomData<#ug_names>,
                )*
                value: B,
            }
        }
    }
    pub fn generate_unit_arithmatic_impl(&self) -> TokenStream {
        let (operand_1, operand_2, result) = &self.unit_arith_generic_names;
        let ud_names = &self.unit_generic_names;
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
            ud_names,
            pd_names,
            Add,
            add,
            +
        ));
        ts.extend(gen_norm_op!(
            ud_names,
            pd_names,
            Sub,
            sub,
            -
        ));
        ts
    }

    pub fn generate_unit_debug(&self) -> TokenStream {
        // TODO incorporate unit vector
        let ud_names = &self.unit_generic_names;
        quote! {
            impl<
                #(
                    #ud_names: ::typenum::Integer,
                )*
                B: core::fmt::Debug,
            > core::fmt::Debug for Unit<
                #(
                    #ud_names,
                )*
                B,
            > {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct("Unit").field("value", &self.value).finish()
                }
            }
        }
    }
}
