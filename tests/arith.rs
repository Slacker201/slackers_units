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

        prefix Deca {
            abbrev: da;
            multiplier: 10;
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

        assert_eq!(unit!(2s ^ 2), one * two)
    }
    #[test]
    fn multiplication_negative() {
        let one = unit!(1s);
        let two = unit!(-2s);
        assert_eq!(unit!(-2s ^ 2), one * two)
    }
    #[test]
    fn division_positive() {
        let one = unit!(1s);
        let two = unit!(2s);
        assert_eq!(unit!(0.5), one / two)
    }
    #[test]
    fn division_negative() {
        let one = unit!(1s);
        let two = unit!(-2s);
        assert_eq!(unit!(-0.5), one / two)
    }
    #[test]
    fn prefix_correct() {
        assert_eq!(unit!(10/s), unit!(1/ds));
        assert_eq!(unit!(100/s), unit!(1/cs));
        assert_eq!(unit!(50000m/s), unit!(50dam/cs));
    }
}
