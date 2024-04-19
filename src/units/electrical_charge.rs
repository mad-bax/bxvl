use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitElectricCharge};

impl Display for UnitElectricCharge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}C", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricCharge {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricCharge> for UnitElectricCharge {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricCharge) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricCharge {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Coulomb(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Coulomb(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
