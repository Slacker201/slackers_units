slackers_units::system! {
    base Meter;
    unit Meter {
        aliases = [m, meter],
    }

    prefix Milli {
        aliases = [m, milli],
        mult = 1e-3,
    }

    unit A {
        aliases = [a],
        eq = 1a,
    }
    unit B {
        aliases = [b],
        eq = 1b,
    }
}

fn main() {}