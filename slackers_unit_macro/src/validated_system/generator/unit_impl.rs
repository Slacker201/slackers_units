use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::validated_system::ValidatedSystem;

macro_rules! generate_dimensional_op {
    ($result: ident, $op1: ident, $op2: ident, $pd_names: ident, $operation: ident, $fn_name: ident, $op_sym: tt) => {
        quote! {
            impl<
            #(
                #$result: ::slackers_units::Integer,
                #$op1: ::slackers_units::Integer + core::ops::$operation<#$op2, Output = #$result>,
                #$op2: ::slackers_units::Integer,
            )*> core::ops::$operation<Unit<
                #(#$op2,)*>> for Unit<#(#$op1,)*> {
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
                #$result: ::slackers_units::Integer,
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
        let debug_impl = self.generate_debug_impl();
        let partial_eq = self.generate_eq_ord();

        let default_repr_path = &self.default_repr;

        quote! {
            pub(crate) struct Unit<#(#dv_names,)* B = #default_repr_path> where
                B: ,
                #(
                    #dv_names: ::slackers_units::Integer,
                )*
            {
                #(#dv_pd_names: std::marker::PhantomData<#dv_names>,)*
                value: B,
            }

            #mul_impl
            #div_impl
            #add_impl
            #sub_impl
            #new_impl
            #debug_impl
            #partial_eq
        }
    }

    fn get_dvec_idents(&self) -> impl Iterator<Item = Ident> {
        self.dimensional_vector_names
            .iter()
            .map(|v| format_ident!("Ident{v}"))
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
                B,
            #(
                #result: ::slackers_units::Integer,
            )*
            > Unit<
                #(
                    #result,
                )* B,
            > {
                pub(crate) fn new(value: B) -> Self {
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

    fn generate_debug_impl(&self) -> TokenStream {
        let result: Box<[_]> = self.get_dvec_idents().collect();

        quote! {
            impl<
                B: core::fmt::Debug,
                #(
                    #result: core::fmt::Debug + ::slackers_units::Integer,
                )*
            > core::fmt::Debug for Unit<
                #(
                    #result,
                )*
            B,
            > {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(
                        "Unit"
                    ).field("value", &self.value).finish()
                }
            }
        }
    }

    fn generate_eq_ord(&self) -> TokenStream {
        let idents: Box<[_]> = self.get_dvec_idents().collect();
        quote! {
            impl<
            B: PartialEq,
            #(
                #idents: slackers_units::Integer,
            )*
            > PartialEq for Unit<
            #(#idents,)*
            B> {
                fn eq(&self, other: &Self) -> bool {
                    self.value == other.value
                }
            }

        impl<
            B: PartialOrd,
            #(
                #idents: slackers_units::Integer,
            )*
            > PartialOrd for Unit<
            #(#idents,)* B> {
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    self.value.partial_cmp(&other.value)
                }
            }

        impl<
            B: Eq,
            #(
                #idents: slackers_units::Integer,
            )*
            > Eq for Unit<
            #(#idents,)* B> {}

        impl<
            B: Ord,
            #(
                #idents: slackers_units::Integer,
            )*
            > Ord for Unit<
            #(#idents,)* B> {
                fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                    self.value.cmp(&other.value)
                }
            }
        }
    }
}
