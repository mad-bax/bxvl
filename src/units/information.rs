use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{Metric, BaseUnit, Convert};

/// The unit types for a measurement of information
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum UnitInformation {
    /// Not SI but uses metric prefixing
    Bit(Metric),
    /// Not SI but uses metric prefixing
    Byte(Metric),
}

impl Display for UnitInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bit(m) => format!("{}bits", m.as_str()),
                Self::Byte(m) => format!("{}b", m.as_str()),
            }
        )
    }
}

impl Into<String> for UnitInformation {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitInformation> for UnitInformation {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitInformation) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitInformation {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            UnitInformation::Bit(m) | UnitInformation::Byte(m) => match m {
                Metric::Yotta => 1208925819614629174706176.0,
                Metric::Zetta => 1180591620717411303424.0,
                Metric::Exa => 1152921504606846976.0,
                Metric::Peta => 1125899906842624.0,
                Metric::Tera => 1099511627776.0,
                Metric::Giga => 1073741824.0,
                Metric::Mega => 1048576.0,
                Metric::Kilo => 1024.0,
                Metric::None => 1.0,
                _ => 1.0,
            },
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Byte(_) => 1.0,
            Self::Bit(_) => 0.125,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Bit(m) => *m,
            Self::Byte(m) => *m,
        }
    }
}

#[cfg(test)]
mod information_testing {
    use crate::units::{information::UnitInformation, Metric, BaseUnit};

    /// Unit Information Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_information_base_comparison() {
        // Bits
        assert!(UnitInformation::Bit(Metric::None).base() == 0.125);
        // Bytes are the base 'SI unit'
        assert!(UnitInformation::Byte(Metric::None).base() == 1.0);
    }
}
