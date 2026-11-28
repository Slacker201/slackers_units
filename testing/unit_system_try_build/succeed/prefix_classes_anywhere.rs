slackers_units::system! {
    base Meter;
    base Second;
    unit Meter {
        aliases = [m, meter],
    }
    unit Second {
        aliases = [s, second],
    }
    prefix Milli {
        aliases = [m, milli],
        prefix_classes = [Default],
        mult = 1e-3,
    }
    prefix Kilo {
        prefix_classes = [Default],
        aliases = [k, kilo],
        mult = 1e3,
    }
    prefix Deca {
        aliases = [da, deca],
        mult = 10,
        prefix_classes = [Default],
    }
}

fn main() {}