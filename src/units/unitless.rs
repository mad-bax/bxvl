use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitNone};

impl Display for UnitNone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "",
                Self::Percentage => "%",
            }
        )
    }
}

impl Into<String> for UnitNone {
    fn into(self) -> String {
        self.to_string()
    }
}

impl<T> Convert<T> for UnitNone
where
    T: BaseUnit,
{
    fn convert(&self, other: &T) -> f64 {
        other.scale() * other.base()
    }
}

impl BaseUnit for UnitNone {
    fn scale(&self) -> f64 {
        1.0
    }

    fn base(&self) -> f64 {
        1.0
    }

    fn get_metric(&self) -> Metric {
        Metric::None
    }
}

#[cfg(test)]
mod unitless_testing {
    use crate::{
        constants::LENGTH_MILE_TO_METER,
        units::{BaseUnit, Convert, Metric, UnitLength, UnitNone},
    };

    /// Unit Time Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_time_base_comparison() {
        assert_eq!(UnitNone::None.base(), 1.0);
        assert_eq!(UnitNone::Percentage.base(), 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [(UnitNone::None, ""), (UnitNone::Percentage, "%")] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitNone::None, Metric::None),
            (UnitNone::None, Metric::None),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [(UnitNone::None, 1.0), (UnitNone::None, 1.0)] {
            assert_eq!(i.0.scale(), i.1);
        }
    }

    #[test]
    fn unitless_convert() {
        assert_eq!(
            (UnitNone::None).convert(&UnitLength::Mile),
            LENGTH_MILE_TO_METER
        );
    }
}
