use crate::errors::V3Error;

use super::Metric;

impl Metric {
    /// Returns the numeric scaling of a given metric prefix
    pub fn scale(&self) -> f64 {
        match self {
            Metric::Quetta => 1.0e30,
            Metric::Ronna => 1.0e27,
            Metric::Yotta => 1.0e24,
            Metric::Zetta => 1.0e21,
            Metric::Exa => 1.0e18,
            Metric::Peta => 1.0e15,
            Metric::Tera => 1.0e12,
            Metric::Giga => 1.0e9,
            Metric::Mega => 1.0e6,
            Metric::Kilo => 1.0e3,
            Metric::Hecto => 1.0e2,
            Metric::Deca => 10.0,
            Metric::None => 1.0,
            Metric::Deci => 0.1,
            Metric::Centi => 0.01,
            Metric::Milli => 1.0e-3,
            Metric::Micro => 1.0e-6,
            Metric::Nano => 1.0e-9,
            Metric::Pico => 1.0e-12,
            Metric::Femto => 1.0e-15,
            Metric::Atto => 1.0e-18,
            Metric::Zepto => 1.0e-21,
            Metric::Yocto => 1.0e-24,
            Metric::Ronto => 1.0e-27,
            Metric::Quecto => 1.0e-30,
        }
    }

    /// Returns the string representation of the metric prefix
    pub fn as_str(&self) -> &str {
        match self {
            Metric::Quetta => "Q",
            Metric::Ronna => "R",
            Metric::Yotta => "Y",
            Metric::Zetta => "Z",
            Metric::Exa => "E",
            Metric::Peta => "P",
            Metric::Tera => "T",
            Metric::Giga => "G",
            Metric::Mega => "M",
            Metric::Kilo => "k",
            Metric::Hecto => "h",
            Metric::Deca => "da",
            Metric::None => "",
            Metric::Deci => "d",
            Metric::Centi => "c",
            Metric::Milli => "m",
            Metric::Micro => "μ",
            Metric::Nano => "n",
            Metric::Pico => "p",
            Metric::Femto => "f",
            Metric::Atto => "a",
            Metric::Zepto => "z",
            Metric::Yocto => "y",
            Metric::Ronto => "r",
            Metric::Quecto => "q",
        }
    }
}

impl TryFrom<&str> for Metric {
    type Error = V3Error;
    fn try_from(m: &str) -> Result<Self, Self::Error> {
        if m.chars().count() > 2 {
            Err(V3Error::ParsingError("Invalid metric prefix".into()))
        } else if m.chars().count() > 1 {
            if m.chars().nth(0) == Some('d') && m.chars().nth(1) == Some('a') {
                Ok(Metric::Deca)
            } else {
                Err(V3Error::ParsingError("Invalid metric prefix".into()))
            }
        } else {
            match m.chars().nth(0).unwrap() {
                'Q' => Ok(Metric::Quetta),
                'R' => Ok(Metric::Ronna),
                'Y' => Ok(Metric::Yotta),
                'Z' => Ok(Metric::Zetta),
                'E' => Ok(Metric::Exa),
                'P' => Ok(Metric::Peta),
                'T' => Ok(Metric::Tera),
                'G' => Ok(Metric::Giga),
                'M' => Ok(Metric::Mega),
                'k' => Ok(Metric::Kilo),
                'h' => Ok(Metric::Hecto),
                'd' => Ok(Metric::Deci),
                'c' => Ok(Metric::Centi),
                'm' => Ok(Metric::Milli),
                'u' | 'μ' => Ok(Metric::Micro),
                'n' => Ok(Metric::Nano),
                'p' => Ok(Metric::Pico),
                'f' => Ok(Metric::Femto),
                'a' => Ok(Metric::Atto),
                'z' => Ok(Metric::Zepto),
                'y' => Ok(Metric::Yocto),
                'r' => Ok(Metric::Ronto),
                'q' => Ok(Metric::Quecto),
                _ => Err(V3Error::ParsingError("Invalid metric prefix".into())),
            }
        }
    }
}

#[cfg(test)]
mod metric_testing {
    use crate::units::Metric;

    #[test]
    fn metric_comparison() {
        assert_eq!(true, Metric::Quecto < Metric::Ronto);
        assert_eq!(true, Metric::Ronto < Metric::Yocto);
        assert_eq!(true, Metric::Yocto < Metric::Zepto);
        assert_eq!(true, Metric::Zepto < Metric::Atto);
        assert_eq!(true, Metric::Atto < Metric::Femto);
        assert_eq!(true, Metric::Femto < Metric::Pico);
        assert_eq!(true, Metric::Pico < Metric::Nano);
        assert_eq!(true, Metric::Nano < Metric::Micro);
        assert_eq!(true, Metric::Micro < Metric::Milli);
        assert_eq!(true, Metric::Milli < Metric::Centi);
        assert_eq!(true, Metric::Centi < Metric::Deci);
        assert_eq!(true, Metric::Deci < Metric::None);
        assert_eq!(true, Metric::None < Metric::Deca);
        assert_eq!(true, Metric::Deca < Metric::Hecto);
        assert_eq!(true, Metric::Hecto < Metric::Kilo);
        assert_eq!(true, Metric::Kilo < Metric::Mega);
        assert_eq!(true, Metric::Mega < Metric::Giga);
        assert_eq!(true, Metric::Giga < Metric::Tera);
        assert_eq!(true, Metric::Tera < Metric::Peta);
        assert_eq!(true, Metric::Peta < Metric::Exa);
        assert_eq!(true, Metric::Exa < Metric::Zetta);
        assert_eq!(true, Metric::Zetta < Metric::Yotta);
        assert_eq!(true, Metric::Yotta < Metric::Ronna);
        assert_eq!(true, Metric::Ronna < Metric::Quetta);
    }

    #[test]
    fn metric_comparison_scale() {
        assert_eq!(true, Metric::Quecto.scale() < Metric::Ronto.scale());
        assert_eq!(true, Metric::Ronto.scale() < Metric::Yocto.scale());
        assert_eq!(true, Metric::Yocto.scale() < Metric::Zepto.scale());
        assert_eq!(true, Metric::Zepto.scale() < Metric::Atto.scale());
        assert_eq!(true, Metric::Atto.scale() < Metric::Femto.scale());
        assert_eq!(true, Metric::Femto.scale() < Metric::Pico.scale());
        assert_eq!(true, Metric::Pico.scale() < Metric::Nano.scale());
        assert_eq!(true, Metric::Nano.scale() < Metric::Micro.scale());
        assert_eq!(true, Metric::Micro.scale() < Metric::Milli.scale());
        assert_eq!(true, Metric::Milli.scale() < Metric::Centi.scale());
        assert_eq!(true, Metric::Centi.scale() < Metric::Deci.scale());
        assert_eq!(true, Metric::Deci.scale() < Metric::None.scale());
        assert_eq!(true, Metric::None.scale() < Metric::Deca.scale());
        assert_eq!(true, Metric::Deca.scale() < Metric::Hecto.scale());
        assert_eq!(true, Metric::Hecto.scale() < Metric::Kilo.scale());
        assert_eq!(true, Metric::Kilo.scale() < Metric::Mega.scale());
        assert_eq!(true, Metric::Mega.scale() < Metric::Giga.scale());
        assert_eq!(true, Metric::Giga.scale() < Metric::Tera.scale());
        assert_eq!(true, Metric::Tera.scale() < Metric::Peta.scale());
        assert_eq!(true, Metric::Peta.scale() < Metric::Exa.scale());
        assert_eq!(true, Metric::Exa.scale() < Metric::Zetta.scale());
        assert_eq!(true, Metric::Zetta.scale() < Metric::Yotta.scale());
        assert_eq!(true, Metric::Yotta.scale() < Metric::Ronna.scale());
        assert_eq!(true, Metric::Ronna.scale() < Metric::Quetta.scale());
    }

    #[test]
    fn metric_string_scale() {
        assert_eq!(Metric::try_from("Q").unwrap(), Metric::Quetta);
        assert_eq!(Metric::try_from("R").unwrap(), Metric::Ronna);
        assert_eq!(Metric::try_from("Y").unwrap(), Metric::Yotta);
        assert_eq!(Metric::try_from("Z").unwrap(), Metric::Zetta);
        assert_eq!(Metric::try_from("E").unwrap(), Metric::Exa);
        assert_eq!(Metric::try_from("P").unwrap(), Metric::Peta);
        assert_eq!(Metric::try_from("T").unwrap(), Metric::Tera);
        assert_eq!(Metric::try_from("G").unwrap(), Metric::Giga);
        assert_eq!(Metric::try_from("M").unwrap(), Metric::Mega);
        assert_eq!(Metric::try_from("k").unwrap(), Metric::Kilo);
        assert_eq!(Metric::try_from("h").unwrap(), Metric::Hecto);
        assert_eq!(Metric::try_from("d").unwrap(), Metric::Deci);
        assert_eq!(Metric::try_from("c").unwrap(), Metric::Centi);
        assert_eq!(Metric::try_from("μ").unwrap(), Metric::Micro);
        assert_eq!(Metric::try_from("u").unwrap(), Metric::Micro);
        assert_eq!(Metric::try_from("n").unwrap(), Metric::Nano);
        assert_eq!(Metric::try_from("p").unwrap(), Metric::Pico);
        assert_eq!(Metric::try_from("f").unwrap(), Metric::Femto);
        assert_eq!(Metric::try_from("a").unwrap(), Metric::Atto);
        assert_eq!(Metric::try_from("z").unwrap(), Metric::Zepto);
        assert_eq!(Metric::try_from("y").unwrap(), Metric::Yocto);
        assert_eq!(Metric::try_from("da").unwrap(), Metric::Deca);
        assert_eq!(Metric::try_from("m").unwrap(), Metric::Milli);
        assert_eq!(Metric::try_from("r").unwrap(), Metric::Ronto);
        assert_eq!(Metric::try_from("q").unwrap(), Metric::Quecto);
    }

    #[test]
    fn metric_string_scale_fails() {
        assert_eq!(Metric::try_from("bb").is_err(), true);
        assert_eq!(Metric::try_from("K").is_err(), true);
        assert_eq!(Metric::try_from("aaa").is_err(), true);
        assert_eq!(Metric::try_from("de").is_err(), true);
        assert_eq!(Metric::try_from("fa").is_err(), true);
    }
}
