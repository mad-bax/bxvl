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
        Metric::None
    }

    fn scale(&self) -> f64 {
        1.0
    }

    fn base(&self) -> f64 {
        1.0
    }
}
