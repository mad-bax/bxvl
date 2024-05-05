use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitIlluminance};

impl Display for UnitIlluminance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}lx", self.get_metric().as_str())
    }
}

impl Into<String> for UnitIlluminance {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitIlluminance> for UnitIlluminance {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitIlluminance) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitIlluminance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Lux(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Lux(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
