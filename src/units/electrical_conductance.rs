use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitElectricConductance};

impl Display for UnitElectricConductance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}S", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricConductance {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricConductance> for UnitElectricConductance {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricConductance) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricConductance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Siemens(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Siemens(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
