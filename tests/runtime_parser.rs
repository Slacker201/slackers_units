mod runtime_parser {
    use crate::runtime_parser;
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
    fn parses_integers() {
        assert_eq!(Ok(unit!(1s)), RuntimeParser::parse("1 * s"));
        assert_eq!(Ok(unit!(5s)), RuntimeParser::parse("5 * s"));
        assert_eq!(Ok(unit!(-1s)), RuntimeParser::parse("-1 * s"));
        assert_eq!(Ok(unit!(-5s)), RuntimeParser::parse("-5 * s"));
        assert_eq!(Ok(unit!(10s)), RuntimeParser::parse("10 * s"));
        assert_eq!(Ok(unit!(50s)), RuntimeParser::parse("50 * s"));
        assert_eq!(Ok(unit!(1000000s)), RuntimeParser::parse("1000000 * s"));
        assert_eq!(Ok(unit!(5000000s)), RuntimeParser::parse("5000000 * s"));
        assert_eq!(Ok(unit!(-1000000s)), RuntimeParser::parse("-1000000 * s"));
        assert_eq!(Ok(unit!(-5000000s)), RuntimeParser::parse("-5000000 * s"));
    }
    #[test]
    fn parses_floats_identical_ints() {
        assert_eq!(Ok(unit!(1s)), RuntimeParser::parse("1.0 * s"));
        assert_eq!(Ok(unit!(5s)), RuntimeParser::parse("5.0 * s"));
        assert_eq!(Ok(unit!(-1s)), RuntimeParser::parse("-1.0 * s"));
        assert_eq!(Ok(unit!(-5s)), RuntimeParser::parse("-5.0 * s"));
        assert_eq!(Ok(unit!(10s)), RuntimeParser::parse("10.0 * s"));
        assert_eq!(Ok(unit!(50s)), RuntimeParser::parse("50.0 * s"));
        assert_eq!(Ok(unit!(5000000s)), RuntimeParser::parse("5000000.0 * s"));
        assert_eq!(Ok(unit!(-5000000s)), RuntimeParser::parse("-5000000.0 * s"));
    }
    #[test]
    fn parses_floats() {
        assert_eq!(Ok(unit!(1.3s)), RuntimeParser::parse("1.3 * s"));
        assert_eq!(Ok(unit!(5.1s)), RuntimeParser::parse("5.1 * s"));
        assert_eq!(Ok(unit!(-1.9s)), RuntimeParser::parse("-1.9 * s"));
        assert_eq!(Ok(unit!(-5.6s)), RuntimeParser::parse("-5.6 * s"));
        assert_eq!(Ok(unit!(10s)), RuntimeParser::parse("10 * s"));
        assert_eq!(Ok(unit!(50s)), RuntimeParser::parse("50 * s"));
        assert_eq!(Ok(unit!(964914.1s)), RuntimeParser::parse("964914.1 * s"));
        assert_eq!(
            Ok(unit!(-759316.59s)),
            RuntimeParser::parse("-759316.59 * s")
        );
    }
    #[test]
    fn parse_mult() {
        assert_eq!(Ok(unit!(1s) * unit!(1m)), RuntimeParser::parse("1 * s * m"));
        assert_eq!(Ok(unit!(1s) * unit!(1s)), RuntimeParser::parse("1 * s * s"));
    }
    #[test]
    fn parse_div() {
        assert_eq!(
            Ok(unit!(1) / unit!(1m) / unit!(1s)),
            RuntimeParser::parse("1 / s / m")
        );
        assert_eq!(
            Ok(unit!(1) / unit!(1s) / unit!(1s)),
            RuntimeParser::parse("1 / s / s")
        );
    }
    #[test]
    fn parse_mixed() {
        assert_eq!(
            Ok(unit!(1) / unit!(1s) * unit!(1m)),
            RuntimeParser::parse("1 / s * m")
        );
        assert_eq!(Ok(unit!(1)), RuntimeParser::parse("1 * s / s"));
    }
}
