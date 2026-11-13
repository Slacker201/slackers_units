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

    default_repr: f64;
}


fn main() {
    let _ = unit!(1q);
}