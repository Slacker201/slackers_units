mod base_type_kind;
mod digit;
mod number;
mod type_identifier;
mod expression;
mod conversions;
mod unit_declaration;
mod prefix_declaration;
mod system;


mod kw {
    syn::custom_keyword!(unit);
    syn::custom_keyword!(prefix);
    syn::custom_keyword!(prefixable);
    syn::custom_keyword!(abbrev);
    syn::custom_keyword!(multiplier);
}