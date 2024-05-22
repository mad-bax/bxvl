use std::fmt::Display;

use crate::constants::KELVIN_TO_CELSIUS;

use super::{BaseUnit, Metric, UnitTemperature};

impl Display for UnitTemperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Celsius => "°c".into(),
                Self::Kelvin(m) => format!("{}K", m.as_str()),
                Self::Fahrenheit => "°f".into(),
            }
        )
    }
}

impl Into<String> for UnitTemperature {
    fn into(self) -> String {
        self.to_string()
    }
}

impl UnitTemperature {
    /// Returns a `f64` to assign to a `Value`
    pub fn convert(&self, other: &UnitTemperature, val: f64) -> f64 {
        if self == other {
            return val;
        }

        match self {
            Self::Celsius => match other {
                Self::Celsius => val,
                Self::Fahrenheit => (val * 1.8) + 32.0,
                Self::Kelvin(m) => f64::max(val + KELVIN_TO_CELSIUS, 0.0) / m.scale(),
            },
            Self::Fahrenheit => match other {
                Self::Celsius => (val - 32.0) / 1.8,
                Self::Fahrenheit => val,
                Self::Kelvin(m) => {
                    f64::max(((val - 32.0) / 1.8) + KELVIN_TO_CELSIUS, 0.0) / m.scale()
                }
            },
            Self::Kelvin(old_m) => match other {
                Self::Celsius => val - KELVIN_TO_CELSIUS,
                Self::Fahrenheit => ((val - KELVIN_TO_CELSIUS) * 1.8) + 32.0,
                Self::Kelvin(new_m) => val * (old_m.scale() / new_m.scale()),
            },
        }
    }
}

impl BaseUnit for UnitTemperature {
    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            UnitTemperature::Kelvin(m) => *m,
            _ => Metric::None,
        }
    }

    fn scale(&self) -> f64 {
        match self {
            UnitTemperature::Kelvin(m) => m.scale(),
            _ => 1.0,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod temperature_testing {
    use crate::units::{BaseUnit, Metric, UnitTemperature};

    #[test]
    fn unit_radioactivity_base_comparison() {
        assert!(UnitTemperature::Kelvin(Metric::None).base() == 1.0);
        assert!(UnitTemperature::Fahrenheit.base() == 1.0);
        assert!(UnitTemperature::Celsius.base() == 1.0);
    }

    #[test]
    fn unit_length_to_string() {
        for i in [
            (UnitTemperature::Kelvin(Metric::Ronto), "rK"),
            (UnitTemperature::Kelvin(Metric::Ronna), "RK"),
            (UnitTemperature::Kelvin(Metric::Quetta), "QK"),
            (UnitTemperature::Kelvin(Metric::Quecto), "qK"),
            (UnitTemperature::Kelvin(Metric::Atto), "aK"),
            (UnitTemperature::Kelvin(Metric::Centi), "cK"),
            (UnitTemperature::Kelvin(Metric::Deca), "daK"),
            (UnitTemperature::Kelvin(Metric::Deci), "dK"),
            (UnitTemperature::Kelvin(Metric::Exa), "EK"),
            (UnitTemperature::Kelvin(Metric::Femto), "fK"),
            (UnitTemperature::Kelvin(Metric::Giga), "GK"),
            (UnitTemperature::Kelvin(Metric::Hecto), "hK"),
            (UnitTemperature::Kelvin(Metric::Kilo), "kK"),
            (UnitTemperature::Kelvin(Metric::Mega), "MK"),
            (UnitTemperature::Kelvin(Metric::Micro), "μK"),
            (UnitTemperature::Kelvin(Metric::Milli), "mK"),
            (UnitTemperature::Kelvin(Metric::Nano), "nK"),
            (UnitTemperature::Kelvin(Metric::None), "K"),
            (UnitTemperature::Kelvin(Metric::Peta), "PK"),
            (UnitTemperature::Kelvin(Metric::Pico), "pK"),
            (UnitTemperature::Kelvin(Metric::Tera), "TK"),
            (UnitTemperature::Kelvin(Metric::Yocto), "yK"),
            (UnitTemperature::Kelvin(Metric::Yotta), "YK"),
            (UnitTemperature::Kelvin(Metric::Zepto), "zK"),
            (UnitTemperature::Kelvin(Metric::Zetta), "ZK"),
            (UnitTemperature::Fahrenheit, "°f"),
            (UnitTemperature::Celsius, "°c"),
        ] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_length_scale() {
        for i in [
            (UnitTemperature::Kelvin(Metric::Ronto), Metric::Ronto),
            (UnitTemperature::Kelvin(Metric::Ronna), Metric::Ronna),
            (UnitTemperature::Kelvin(Metric::Quetta), Metric::Quetta),
            (UnitTemperature::Kelvin(Metric::Quecto), Metric::Quecto),
            (UnitTemperature::Kelvin(Metric::Atto), Metric::Atto),
            (UnitTemperature::Kelvin(Metric::Centi), Metric::Centi),
            (UnitTemperature::Kelvin(Metric::Deca), Metric::Deca),
            (UnitTemperature::Kelvin(Metric::Deci), Metric::Deci),
            (UnitTemperature::Kelvin(Metric::Exa), Metric::Exa),
            (UnitTemperature::Kelvin(Metric::Femto), Metric::Femto),
            (UnitTemperature::Kelvin(Metric::Giga), Metric::Giga),
            (UnitTemperature::Kelvin(Metric::Hecto), Metric::Hecto),
            (UnitTemperature::Kelvin(Metric::Kilo), Metric::Kilo),
            (UnitTemperature::Kelvin(Metric::Mega), Metric::Mega),
            (UnitTemperature::Kelvin(Metric::Micro), Metric::Micro),
            (UnitTemperature::Kelvin(Metric::Milli), Metric::Milli),
            (UnitTemperature::Kelvin(Metric::Nano), Metric::Nano),
            (UnitTemperature::Kelvin(Metric::None), Metric::None),
            (UnitTemperature::Kelvin(Metric::Peta), Metric::Peta),
            (UnitTemperature::Kelvin(Metric::Pico), Metric::Pico),
            (UnitTemperature::Kelvin(Metric::Tera), Metric::Tera),
            (UnitTemperature::Kelvin(Metric::Yocto), Metric::Yocto),
            (UnitTemperature::Kelvin(Metric::Yotta), Metric::Yotta),
            (UnitTemperature::Kelvin(Metric::Zepto), Metric::Zepto),
            (UnitTemperature::Kelvin(Metric::Zetta), Metric::Zetta),
            (UnitTemperature::Fahrenheit, Metric::None),
            (UnitTemperature::Celsius, Metric::None),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }
        for i in [
            (UnitTemperature::Kelvin(Metric::Ronto), 1.0e-27),
            (UnitTemperature::Kelvin(Metric::Ronna), 1.0e27),
            (UnitTemperature::Kelvin(Metric::Quetta), 1.0e30),
            (UnitTemperature::Kelvin(Metric::Quecto), 1.0e-30),
            (UnitTemperature::Kelvin(Metric::Atto), 1.0e-18),
            (UnitTemperature::Kelvin(Metric::Centi), 0.01),
            (UnitTemperature::Kelvin(Metric::Deca), 10.0),
            (UnitTemperature::Kelvin(Metric::Deci), 0.1),
            (UnitTemperature::Kelvin(Metric::Exa), 1.0e18),
            (UnitTemperature::Kelvin(Metric::Femto), 1.0e-15),
            (UnitTemperature::Kelvin(Metric::Giga), 1.0e9),
            (UnitTemperature::Kelvin(Metric::Hecto), 100.0),
            (UnitTemperature::Kelvin(Metric::Kilo), 1.0e3),
            (UnitTemperature::Kelvin(Metric::Mega), 1.0e6),
            (UnitTemperature::Kelvin(Metric::Micro), 1.0e-6),
            (UnitTemperature::Kelvin(Metric::Milli), 0.001),
            (UnitTemperature::Kelvin(Metric::Nano), 1.0e-9),
            (UnitTemperature::Kelvin(Metric::None), 1.0),
            (UnitTemperature::Kelvin(Metric::Peta), 1.0e15),
            (UnitTemperature::Kelvin(Metric::Pico), 1.0e-12),
            (UnitTemperature::Kelvin(Metric::Tera), 1.0e12),
            (UnitTemperature::Kelvin(Metric::Yocto), 1.0e-24),
            (UnitTemperature::Kelvin(Metric::Yotta), 1.0e24),
            (UnitTemperature::Kelvin(Metric::Zepto), 1.0e-21),
            (UnitTemperature::Kelvin(Metric::Zetta), 1.0e21),
            (UnitTemperature::Fahrenheit, 1.0),
            (UnitTemperature::Celsius, 1.0),
        ] {
            assert_eq!(i.0.scale(), i.1);
        }
    }
}
