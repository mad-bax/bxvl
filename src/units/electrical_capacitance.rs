use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitElectricCapacitance};

impl Display for UnitElectricCapacitance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}F", self.get_metric().as_str())
    }
}

impl Into<String> for UnitElectricCapacitance {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitElectricCapacitance> for UnitElectricCapacitance {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitElectricCapacitance) -> f64 {
        self.scale() / other.scale()
    }
}

impl BaseUnit for UnitElectricCapacitance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Farad(m) => m.scale(),
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Farad(m) => *m,
        }
    }

    fn base(&self) -> f64 {
        1.0
    }
}
