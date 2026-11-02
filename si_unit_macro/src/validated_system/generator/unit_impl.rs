use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::validated_system::ValidatedSystem;

macro_rules! generate_dimensional_op {
    ($result: ident, $op1: ident, $op2: ident, $pd_names: ident, $operation: ident, $fn_name: ident, $op_sym: tt) => {
        quote! {
            impl<
            #(
                #$result: slackers_si_units::Integer,
                #$op1: slackers_si_units::Integer + core::ops::$operation<#$op2, Output = #$result>,
                #$op2: slackers_si_units::Integer,
            )*> core::ops::$operation<Unit<#(#$op2,)*>> for Unit<#(#$op1,)*> {
                type Output = Unit<#(#$result,)*>;

                fn $fn_name(self, other: Unit<#(#$op2,)*>) -> Self::Output {
                    Self::Output {
                        #(
                            #$pd_names: core::marker::PhantomData,
                        )*
                        value: self.value $op_sym other.value
                    }
                }
            }
        }
    };
}

macro_rules! generate_normal_op {
    ($result: ident, $pd_names: ident, $operation: ident, $fn_name: ident, $op_sym: tt) => {
        quote! {
            impl<
            #(
                #$result: slackers_si_units::Integer,
            )*> core::ops::$operation<Self> for Unit<#(#$result,)*> {
                type Output = Self;

                fn $fn_name(self, other: Self) -> Self::Output {
                    Self::Output {
                        #(
                            #$pd_names: core::marker::PhantomData,
                        )*
                        value: self.value $op_sym other.value
                    }
                }
            }
        }
    };
}

impl ValidatedSystem {
    pub(crate) fn generate_units(&self) -> TokenStream {
        let dv_names = &self.dimensional_vector_names;
        let dv_pd_names = &self.dimensional_phantom_data_names;

        let mul_impl = self.generate_mul_impl();
        let div_impl = self.generate_div_impl();
        let add_impl = self.generate_add_impl();
        let sub_impl = self.generate_sub_impl();
        let new_impl = self.generate_new();

        quote! {
            pub(crate) struct Unit<#(#dv_names,)*> where
                #(
                    #dv_names: slackers_si_units::Integer,
                )*
            {
                #(#dv_pd_names: std::marker::PhantomData<#dv_names>,)*
                value: f64,
            }

            #mul_impl
            #div_impl
            #add_impl
            #sub_impl
            #new_impl
        }
    }

    fn get_arith_types(&self) -> (Vec<Ident>, Vec<Ident>, Vec<Ident>, &[Ident]) {
        let (result, (op1, op2)): (Vec<_>, (Vec<_>, Vec<_>)) = self
            .dimensional_vector_names
            .iter()
            .map(|v| {
                (
                    format_ident!("Result_{v}"),
                    (
                        format_ident!("Operand_One_{v}"),
                        format_ident!("Operand_Two_{v}"),
                    ),
                )
            })
            .unzip();

        let pd_names = &self.dimensional_phantom_data_names;

        (result, op1, op2, pd_names)
    }

    fn generate_mul_impl(&self) -> TokenStream {
        let (result, op1, op2, pd_names) = self.get_arith_types();

        generate_dimensional_op!(
            result, op1, op2, pd_names, Mul, mul, *
        )
    }

    fn generate_div_impl(&self) -> TokenStream {
        let (result, op1, op2, pd_names) = self.get_arith_types();

        generate_dimensional_op!(
            result, op1, op2, pd_names, Div, div, /
        )
    }

    fn generate_add_impl(&self) -> TokenStream {
        let (result, _, _, pd_names) = self.get_arith_types();

        generate_normal_op!(
            result, pd_names, Add, add, +
        )
    }

    fn generate_sub_impl(&self) -> TokenStream {
        let (result, _, _, pd_names) = self.get_arith_types();

        generate_normal_op!(
            result, pd_names, Sub, sub, -
        )
    }

    fn generate_new(&self) -> TokenStream {
        let (result, _, _, pd_names) = self.get_arith_types();
        quote! {
            impl<
            #(
                #result: ::slackers_si_units::Integer,
            )*
            > Unit<
                #(
                    #result,
                )*
            > {
                pub(crate) fn new(value: f64) -> Self {
                    Self {
                        #(
                            #pd_names: core::marker::PhantomData,
                        )*
                        value
                    }
                }
            }
        }
    }
}
