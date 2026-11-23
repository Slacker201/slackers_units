use syn::Error;

use crate::data::parser_data::System;

impl System {
    pub fn no_colliding_aliases(&self) -> syn::Result<()> {
        let mut unit_aliases: Box<[_]> = self
            .units
            .iter()
            .flat_map(|unit_def| &unit_def.aliases.aliases)
            .collect();
        unit_aliases.sort();
        if let Some(duplicate) = unit_aliases
            .windows(2)
            .filter_map(|w| if w[0] == w[1] { Some(w[1]) } else { None })
            .next()
        {
            return Err(Error::new(
                duplicate.span(),
                format!("Found duplicate abbreviation {duplicate}"),
            ));
        }
        Ok(())
    }
}
