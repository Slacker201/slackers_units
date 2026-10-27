use std::fmt::Display;

use syn::{Error, Ident, parse::Parse};

use crate::unit_system::BaseTypeKind;

const BASETYPES: phf::Map<&'static str, BaseTypeKind> = phf::phf_map! {
    "time" => BaseTypeKind::Time,
    "distance" => BaseTypeKind::Distance,
    "mass" => BaseTypeKind::Mass,
    "electric_current" => BaseTypeKind::ElectricCurrent,
    "temperature" => BaseTypeKind::Temperature,
    "amount" => BaseTypeKind::Amount,
    "luminous_intensity" => BaseTypeKind::LuminousIntensity,
};

const EXPRECTED_ERROR_MESSAGE: &str = "\"time\", \"distance\", \"mass\", \"electric_current\", \"temperature\", \"amount\", or \"luminous_intensity\"";

impl Parse for BaseTypeKind {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        let ret = match BASETYPES.get(&ident.to_string()) {
            Some(ret) => *ret,
            None => {
                let fixed_ident = ident.to_string().to_lowercase().replace("-", "_");

                if BASETYPES.contains_key(&fixed_ident) {
                    return Err(Error::new(
                        ident.span(),
                        format!(
                            "Expected {EXPRECTED_ERROR_MESSAGE}, got {}, did you mean {fixed_ident}?",
                            ident.to_string()
                        ),
                    ));
                }
                return Err(Error::new(
                    ident.span(),
                    format!(
                        "Expected {EXPRECTED_ERROR_MESSAGE}, got {}.",
                        ident.to_string()
                    ),
                ));
            }
        };
        Ok(ret)
    }
}

impl Display for BaseTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            BaseTypeKind::Time => "time",
            BaseTypeKind::Distance => "distance",
            BaseTypeKind::Mass => "mass",
            BaseTypeKind::ElectricCurrent => "electric_current",
            BaseTypeKind::Temperature => "temperature",
            BaseTypeKind::Amount => "amount",
            BaseTypeKind::LuminousIntensity => "luminous_intensity",
        };
        write!(f, "{v}")
    }
}
