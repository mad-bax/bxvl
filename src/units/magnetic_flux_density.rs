use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitMagneticFluxDensity};

impl Display for UnitMagneticFluxDensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}T", self.get_metric().as_str())
    }
}

impl Into<String> for UnitMagneticFluxDensity {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitMagneticFluxDensity> for UnitMagneticFluxDensity {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitMagneticFluxDensity) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitMagneticFluxDensity {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Tesla(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Tesla(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
