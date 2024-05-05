use std::fmt::Display;

use crate::constants;

use super::{BaseUnit, Convert, Metric, UnitAngle};

impl Display for UnitAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Radian(Metric::Milli) => "mil".into(),
                Self::Radian(m) => format!("{}rad", m.as_str()),
                Self::Degree => "°".into(),
                Self::Moa => "moa".into(),
            }
        )
    }
}

impl Into<String> for UnitAngle {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitAngle> for UnitAngle {
    /// Returns the `f64` multiplier to convert a `Value`
    fn convert(&self, other: &UnitAngle) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }
}

impl BaseUnit for UnitAngle {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Radian(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Radian(_) => 1.0,
            Self::Degree => constants::ANGLE_DEG_TO_RAD,
            Self::Moa => constants::ANGLE_MOA_TO_RAD,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Radian(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod angle_testing {
    use crate::units::{BaseUnit, Metric, UnitAngle};

    /// Unit Angle Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_angle_base_comparison() {
        // Radians are the base SI unit
        assert!(UnitAngle::Radian(Metric::None).base() == 1.0);
        // Degrees
        assert!(UnitAngle::Degree.base() >= 0.017_453_292_50);
        // Minute of Angle
        assert!(UnitAngle::Moa.base() >= 0.000_290_888_208_664);
    }
}
