use std::fmt::Display;

use crate::constants;

use super::{BaseUnit, Convert, Metric, UnitRadioactivity};

impl Display for UnitRadioactivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Becquerel(m) => format!("{}Bq", m.as_str()),
                Self::Curie => "Ci".into(),
            }
        )
    }
}

impl From<UnitRadioactivity> for String {
    fn from(val: UnitRadioactivity) -> Self {
        val.to_string()
    }
}

impl Convert<UnitRadioactivity> for UnitRadioactivity {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitRadioactivity) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitRadioactivity {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Becquerel(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Becquerel(_) => 1.0,
            Self::Curie => constants::RADIO_C_TO_BQ,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Becquerel(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod radioactivity_testing {
    use crate::units::{radioactivity::UnitRadioactivity, BaseUnit, Metric};

    /// Unit Radioactivity Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_radioactivity_base_comparison() {
        // Becquerels are the base SI unit
        assert!(UnitRadioactivity::Becquerel(Metric::None).base() == 1.0);
        // Curies
        assert!(UnitRadioactivity::Curie.base() == 37_000_000_000.0);
    }

    #[test]
    fn unit_length_to_string() {
        for i in [
            (UnitRadioactivity::Becquerel(Metric::Ronto), "rBq"),
            (UnitRadioactivity::Becquerel(Metric::Ronna), "RBq"),
            (UnitRadioactivity::Becquerel(Metric::Quetta), "QBq"),
            (UnitRadioactivity::Becquerel(Metric::Quecto), "qBq"),
            (UnitRadioactivity::Becquerel(Metric::Atto), "aBq"),
            (UnitRadioactivity::Becquerel(Metric::Centi), "cBq"),
            (UnitRadioactivity::Becquerel(Metric::Deca), "daBq"),
            (UnitRadioactivity::Becquerel(Metric::Deci), "dBq"),
            (UnitRadioactivity::Becquerel(Metric::Exa), "EBq"),
            (UnitRadioactivity::Becquerel(Metric::Femto), "fBq"),
            (UnitRadioactivity::Becquerel(Metric::Giga), "GBq"),
            (UnitRadioactivity::Becquerel(Metric::Hecto), "hBq"),
            (UnitRadioactivity::Becquerel(Metric::Kilo), "kBq"),
            (UnitRadioactivity::Becquerel(Metric::Mega), "MBq"),
            (UnitRadioactivity::Becquerel(Metric::Micro), "μBq"),
            (UnitRadioactivity::Becquerel(Metric::Milli), "mBq"),
            (UnitRadioactivity::Becquerel(Metric::Nano), "nBq"),
            (UnitRadioactivity::Becquerel(Metric::None), "Bq"),
            (UnitRadioactivity::Becquerel(Metric::Peta), "PBq"),
            (UnitRadioactivity::Becquerel(Metric::Pico), "pBq"),
            (UnitRadioactivity::Becquerel(Metric::Tera), "TBq"),
            (UnitRadioactivity::Becquerel(Metric::Yocto), "yBq"),
            (UnitRadioactivity::Becquerel(Metric::Yotta), "YBq"),
            (UnitRadioactivity::Becquerel(Metric::Zepto), "zBq"),
            (UnitRadioactivity::Becquerel(Metric::Zetta), "ZBq"),
            (UnitRadioactivity::Curie, "Ci"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_length_scale() {
        for i in [
            (UnitRadioactivity::Becquerel(Metric::Ronto), Metric::Ronto),
            (UnitRadioactivity::Becquerel(Metric::Ronna), Metric::Ronna),
            (UnitRadioactivity::Becquerel(Metric::Quetta), Metric::Quetta),
            (UnitRadioactivity::Becquerel(Metric::Quecto), Metric::Quecto),
            (UnitRadioactivity::Becquerel(Metric::Atto), Metric::Atto),
            (UnitRadioactivity::Becquerel(Metric::Centi), Metric::Centi),
            (UnitRadioactivity::Becquerel(Metric::Deca), Metric::Deca),
            (UnitRadioactivity::Becquerel(Metric::Deci), Metric::Deci),
            (UnitRadioactivity::Becquerel(Metric::Exa), Metric::Exa),
            (UnitRadioactivity::Becquerel(Metric::Femto), Metric::Femto),
            (UnitRadioactivity::Becquerel(Metric::Giga), Metric::Giga),
            (UnitRadioactivity::Becquerel(Metric::Hecto), Metric::Hecto),
            (UnitRadioactivity::Becquerel(Metric::Kilo), Metric::Kilo),
            (UnitRadioactivity::Becquerel(Metric::Mega), Metric::Mega),
            (UnitRadioactivity::Becquerel(Metric::Micro), Metric::Micro),
            (UnitRadioactivity::Becquerel(Metric::Milli), Metric::Milli),
            (UnitRadioactivity::Becquerel(Metric::Nano), Metric::Nano),
            (UnitRadioactivity::Becquerel(Metric::None), Metric::None),
            (UnitRadioactivity::Becquerel(Metric::Peta), Metric::Peta),
            (UnitRadioactivity::Becquerel(Metric::Pico), Metric::Pico),
            (UnitRadioactivity::Becquerel(Metric::Tera), Metric::Tera),
            (UnitRadioactivity::Becquerel(Metric::Yocto), Metric::Yocto),
            (UnitRadioactivity::Becquerel(Metric::Yotta), Metric::Yotta),
            (UnitRadioactivity::Becquerel(Metric::Zepto), Metric::Zepto),
            (UnitRadioactivity::Becquerel(Metric::Zetta), Metric::Zetta),
            (UnitRadioactivity::Curie, Metric::None),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }
        for i in [
            (UnitRadioactivity::Becquerel(Metric::Ronto), 1.0e-27),
            (UnitRadioactivity::Becquerel(Metric::Ronna), 1.0e27),
            (UnitRadioactivity::Becquerel(Metric::Quetta), 1.0e30),
            (UnitRadioactivity::Becquerel(Metric::Quecto), 1.0e-30),
            (UnitRadioactivity::Becquerel(Metric::Atto), 1.0e-18),
            (UnitRadioactivity::Becquerel(Metric::Centi), 0.01),
            (UnitRadioactivity::Becquerel(Metric::Deca), 10.0),
            (UnitRadioactivity::Becquerel(Metric::Deci), 0.1),
            (UnitRadioactivity::Becquerel(Metric::Exa), 1.0e18),
            (UnitRadioactivity::Becquerel(Metric::Femto), 1.0e-15),
            (UnitRadioactivity::Becquerel(Metric::Giga), 1.0e9),
            (UnitRadioactivity::Becquerel(Metric::Hecto), 100.0),
            (UnitRadioactivity::Becquerel(Metric::Kilo), 1.0e3),
            (UnitRadioactivity::Becquerel(Metric::Mega), 1.0e6),
            (UnitRadioactivity::Becquerel(Metric::Micro), 1.0e-6),
            (UnitRadioactivity::Becquerel(Metric::Milli), 0.001),
            (UnitRadioactivity::Becquerel(Metric::Nano), 1.0e-9),
            (UnitRadioactivity::Becquerel(Metric::None), 1.0),
            (UnitRadioactivity::Becquerel(Metric::Peta), 1.0e15),
            (UnitRadioactivity::Becquerel(Metric::Pico), 1.0e-12),
            (UnitRadioactivity::Becquerel(Metric::Tera), 1.0e12),
            (UnitRadioactivity::Becquerel(Metric::Yocto), 1.0e-24),
            (UnitRadioactivity::Becquerel(Metric::Yotta), 1.0e24),
            (UnitRadioactivity::Becquerel(Metric::Zepto), 1.0e-21),
            (UnitRadioactivity::Becquerel(Metric::Zetta), 1.0e21),
            (UnitRadioactivity::Curie, 1.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
