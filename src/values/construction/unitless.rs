use std::ops::{Div, Mul};

use crate::{units::UnitNone, values::Value};

impl Mul<UnitNone> for f64 {
    type Output = Value;
    fn mul(self, _: UnitNone) -> Self::Output {
        Value {
            val: self,
            ..Default::default()
        }
    }
}

impl Div<UnitNone> for f64 {
    type Output = Value;
    fn div(self, _: UnitNone) -> Self::Output {
        Value {
            val: self,
            ..Default::default()
        }
    }
}

impl Mul<UnitNone> for Value {
    type Output = Value;
    fn mul(self, _: UnitNone) -> Self::Output {
        self
    }
}

impl Div<UnitNone> for Value {
    type Output = Value;
    fn div(self, _: UnitNone) -> Value {
        self
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{LUMINOUS_INTENSITY_INDEX, LUMINOUS_INTENSITY_MAP},
        units::{Metric, UnitLuminousIntensity, UnitNone},
    };

    const USED_MAP: usize = LUMINOUS_INTENSITY_MAP;
    const USED_INDEX: usize = LUMINOUS_INTENSITY_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitNone::None;
        assert!(v.is_empty());
        assert_eq!(v.unit_map, 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitNone::None;
        assert!(v.is_empty());
        assert_eq!(v.unit_map, 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1 * UnitLuminousIntensity::Candela(Metric::None) * UnitNone::None;
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 1);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitLuminousIntensity::Candela(Metric::None) / UnitNone::None;
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 1);
        assert_eq!(v, 1.1);
    }
}
