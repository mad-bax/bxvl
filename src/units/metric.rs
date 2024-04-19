use crate::errors::V3Error;

use super::Metric;

impl Metric {
    /// Returns the numeric scaling of a given metric prefix
    pub fn scale(&self) -> f64 {
        match self {
            Metric::Yotta => 1000000000000000000000000.0,
            Metric::Zetta => 1000000000000000000000.0,
            Metric::Exa => 1000000000000000000.0,
            Metric::Peta => 1000000000000000.0,
            Metric::Tera => 1000000000000.0,
            Metric::Giga => 1000000000.0,
            Metric::Mega => 1000000.0,
            Metric::Kilo => 1000.0,
            Metric::Hecto => 100.0,
            Metric::Deca => 10.0,
            Metric::None => 1.0,
            Metric::Deci => 0.1,
            Metric::Centi => 0.01,
            Metric::Milli => 0.001,
            Metric::Micro => 0.000001,
            Metric::Nano => 0.000000001,
            Metric::Pico => 0.000000000001,
            Metric::Femto => 0.000000000000001,
            Metric::Atto => 0.000000000000000001,
            Metric::Zepto => 0.000000000000000000001,
            Metric::Yocto => 0.000000000000000000000001,
        }
    }

    /// Returns the string representation of the metric prefix
    pub fn as_str(&self) -> &str {
        match self {
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
        }
    }
}

impl TryFrom<&str> for Metric {
    type Error = V3Error;
    fn try_from(m: &str) -> Result<Self, Self::Error> {
        if m.chars().count() > 2 {
            if m.chars().nth(0) == Some('d') && m.chars().nth(1) == Some('a') {
                Ok(Metric::Deca)
            } else {
                Err(V3Error::ParsingError("Invalid metric prefix".into()))
            }
        } else if m.chars().count() > 1 {
            match m.chars().nth(0).unwrap() {
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
                _ => Err(V3Error::ParsingError("Invalid metric prefix".into())),
            }
        } else {
            Err(V3Error::ParsingError("Invalid metric prefix".into()))
        }
    }
}

#[cfg(test)]
mod metric_testing {
    use crate::units::Metric;

    #[test]
    fn metric_comparison() {
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
    }

    #[test]
    fn metric_comparison_scale() {
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
    }
}
