slackers_units::system! {
    base Meter;
    unit Meter {
        aliases = [m, meter],
    }

    unit Foot {
        aliases = [ft],
        eq = 304.8mmk,
    }

    prefix Milli {
        aliases = [m, milli],
        mult = 1e-3,
    }
}


fn main() {}