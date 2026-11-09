mod prefix {
    use crate::prefix;
    use slackers_units::system;

    system! {
        basetype: Second;
        basetype: Meter;

        unit Second {
            prefixable: true;
            abbrev: "s";
        }

        unit Meter {
            prefixable: true;
            abbrev: "m";
        }

        prefix Deci {
            abbrev: "d";
            multiplier: 0.1;
        }

        prefix Centi {
            abbrev: "c";
            multiplier: 0.01;
        }

        prefix Degenerate {
            abbrev: "deg";
            multiplier: 69420.69;
        }
        
        default_repr: f64;
    }

    #[test]
    fn prefix_applies_correctly() {
        assert_eq!(unit!(1ds), unit!(0.1s));
        assert_eq!(unit!(1cs), unit!(0.01s));
        assert_eq!(unit!(1degs), unit!(69420.69s));
        assert_eq!(unit!(5ds), unit!(0.5s));
        assert_eq!(unit!(5cs), unit!(0.05s));
        assert_eq!(unit!(5degs), unit!(347103.45s));
    }
}
