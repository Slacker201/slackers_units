mod base_type_kind;
mod conversions;
mod digit;
mod expression;
mod number;
mod prefix_declaration;
mod system;
mod type_identifier;
mod unit_declaration;

mod kw {
    syn::custom_keyword!(unit);
    syn::custom_keyword!(prefix);
    syn::custom_keyword!(prefixable);
    syn::custom_keyword!(abbrev);
    syn::custom_keyword!(multiplier);
    syn::custom_keyword!(conversions);
}
