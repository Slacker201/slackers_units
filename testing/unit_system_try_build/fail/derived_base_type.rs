slackers_units::system! {
    base Foot;

    unit Yard {
        aliases = [yd, yard],
    }

    unit Foot {
        aliases = [ft, foot, feet],
        eq = 1yd/3,
    }

    prefix Milli {
        aliases = [m, milli],
        mult = 1e-3,
    }
}


fn main() {}