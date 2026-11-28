slackers_units::system! {
    base Meter;
    unit Meter {
        aliases = [m, meter],
    }

    unit Foot {
        aliases = [ft],
        eq = 304.8mm,
    }

    prefix Milli {
        aliases = [m, milli],
        mult = 1e-3,
        prefix_classes = [Test],
    }
}

fn main() {}