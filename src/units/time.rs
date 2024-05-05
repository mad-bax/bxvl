use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitFrequency, UnitTime};

impl Display for UnitTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnitTime::Second(m) => format!("{}s", m.as_str()),
                UnitTime::Minute => "min".into(),
                UnitTime::Hour => "hr".into(),
                UnitTime::Day => "day".into(),
            }
        )
    }
}

impl Into<String> for UnitTime {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Convert<UnitTime> for UnitTime {
    fn convert(&self, other: &UnitTime) -> f64 {
        ((self.scale() / other.scale()) * (self.base() / other.base())).into()
    }
}

impl Convert<UnitFrequency> for UnitTime {
    fn convert(&self, other: &UnitFrequency) -> f64 {
        1.0 / (self.base() * self.scale() * other.scale())
    }
}

impl BaseUnit for UnitTime {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Second(m) => m.scale(),
            _ => 1.0,
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Second(_) => 1.0,
            Self::Minute => 60.0,
            Self::Hour => 3600.0,
            Self::Day => 86400.0,
        }
    }

    /// Returns the `Metric` prefix for the unit
    fn get_metric(&self) -> Metric {
        match self {
            Self::Second(m) => *m,
            _ => Metric::None,
        }
    }
}

#[cfg(test)]
mod time_testing {
    use crate::units::{time::UnitTime, BaseUnit, Metric};

    /// Unit Time Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_time_base_comparison() {
        assert_eq!(UnitTime::Second(Metric::None).base(), 1.0);
        assert_eq!(UnitTime::Minute.base(), 60.0);
        assert_eq!(UnitTime::Hour.base(), 3600.0);
        assert_eq!(UnitTime::Day.base(), 86400.0);
    }
}
