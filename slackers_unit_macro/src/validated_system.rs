use syn::{Ident, Path};

use crate::unit_system::{BaseTypeDeclaration, PrefixDeclaration, UnitDeclaration};

mod generator;
mod utils;

pub(crate) struct ValidatedSystem {
    pub units: Box<[UnitDeclaration]>,
    pub prefixes: Box<[PrefixDeclaration]>,

    pub unit_enum_names: Box<[Ident]>,
    pub prefix_enum_names: Box<[Ident]>,
    #[allow(unused)]
    pub base_types: Box<[BaseTypeDeclaration]>,
    pub base_type_mapping: Box<[usize]>,
    pub dimensional_vector_names: Box<[Ident]>,
    pub dimensional_phantom_data_names: Box<[Ident]>,
    pub base_types_raw_vectors: Box<[Box<[i64]>]>,

    pub call_site: String,
    pub default_repr: Path,
}
