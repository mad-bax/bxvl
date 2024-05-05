use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitMagneticFlux};

impl Display for UnitMagneticFlux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Wb", self.get_metric().as_str())
    }
}

impl Into<String> for UnitMagneticFlux {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitMagneticFlux> for UnitMagneticFlux {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitMagneticFlux) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitMagneticFlux {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Weber(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Weber(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
