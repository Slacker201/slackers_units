

macro_rules! warning {
    ($msg: literal) => {
        {
            
            #[must_use = $msg]
            fn warn() -> () {}

            warn();
        }
    };
}


mod arith {
    use crate::arith;
    use slackers_units::system;

    system! {
        basetype: Second;
        basetype: Meter;

        unit Second {
            prefixable: true;
            abbrev: s;
        }

        unit Meter {
            prefixable: false;
            abbrev: m;
        }

        prefix Deci {
            abbrev: d;
            multiplier: 0.1;
        }

        prefix Centi {
            abbrev: c;
            multiplier: 0.01;
        }

        prefix Degenerate {
            abbrev: deg;
            multiplier: 69420.69;
        }

        default_repr: f64;
    }

    #[test]
    fn addition_positive() {
        let one = unit!(1s);
        let two = unit!(2s);
        assert_eq!(unit!(3s), one + two);
    }
    #[test]
    fn addition_negative() {
        let one = unit!(1s);
        let two = unit!(-2s);
        assert_eq!(unit!(-1s), one + two);
    }
    #[test]
    fn subtraction_positive() {
        let one = unit!(1s);
        let two = unit!(2s);
        assert_eq!(unit!(-1s), one - two);
    }
    #[test]
    fn subtraction_negative() {
        let one = unit!(1s);
        let two = unit!(-2s);
        assert_eq!(unit!(3s), one - two);
    }
    #[test]
    fn multiplication_positive() {
        let one = unit!(1s);
        let two = unit!(2s);

        warning!("TODO replace with compile time parser");
        assert_eq!(
            RuntimeParser::parse::<Unit<_, _>>("2*s*s").unwrap(),
            one * two
        )
    }

    #[test]
    fn multiplication_negative() {
        let one = unit!(1s);
        let two = unit!(2s);
        warning!("TODO replace with compile time parser");
        assert_eq!(
            RuntimeParser::parse::<Unit<_, _>>("2*s*s").unwrap(),
            one * two
        )
    }

    #[test]
    fn division_positive() {
        let one = unit!(1s);
        let two = unit!(2s);

        warning!("TODO replace with compile time parser");
        assert_eq!(
            RuntimeParser::parse::<Unit<_, _>>("0.5").unwrap(),
            one / two
        )
    }

    #[test]
    fn division_negative() {
        let one = unit!(1s);
        let two = unit!(-2s);
        warning!("TODO replace with compile time parser");
        assert_eq!(
            RuntimeParser::parse::<Unit<_, _>>("-0.5").unwrap(),
            one / two
        )
    }
}