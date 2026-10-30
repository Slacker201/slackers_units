pub(crate) mod conversions;
pub(crate) mod digit;
pub(crate) mod expression;
pub(crate) mod number;
pub(crate) mod prefix_declaration;
pub(crate) mod system;
pub(crate) mod type_identifier;
pub(crate) mod unit_declaration;

pub(crate) mod kw {
    syn::custom_keyword!(unit);
    syn::custom_keyword!(prefix);
    syn::custom_keyword!(prefixable);
    syn::custom_keyword!(abbrev);
    syn::custom_keyword!(multiplier);
    syn::custom_keyword!(conversions);
    syn::custom_keyword!(basetype);
}
