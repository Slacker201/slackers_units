::slackers_units::system! {
    basetype: Second;
    basetype: Meter;
    unit Second {
        prefixable: true;
        abbrev: s;
    }
    unit Meter {
        prefixable: true;
        abbrev: m;
    }

    prefix Mutar {
        abbrev: j;
        multiplier: 0.1;
    }

    default_repr: f64;
}


fn main() {
    let _ = unit!(1m^);
}