use std::ops::{Div, Mul};

use crate::{
    constants::{LUMINOUS_INTENSITY_INDEX, LUMINOUS_INTENSITY_MAP},
    units::UnitLuminousIntensity,
    value::Value,
};

impl Mul<UnitLuminousIntensity> for f64 {
    type Output = Value;
    fn mul(self, other: UnitLuminousIntensity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_luminous_flux_intensity: Some(other),
            unit_map: LUMINOUS_INTENSITY_MAP,
            ..Default::default()
        };
        ret.exp[LUMINOUS_INTENSITY_INDEX] = 1;
        ret
    }
}

impl Div<UnitLuminousIntensity> for f64 {
    type Output = Value;
    fn div(self, other: UnitLuminousIntensity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_luminous_flux_intensity: Some(other),
            unit_map: LUMINOUS_INTENSITY_MAP,
            ..Default::default()
        };
        ret.exp[LUMINOUS_INTENSITY_INDEX] = -1;
        ret
    }
}

impl Mul<UnitLuminousIntensity> for Value {
    type Output = Value;
    fn mul(self, other: UnitLuminousIntensity) -> Self::Output {
        let mut new: Value = self;
        if self.exp[LUMINOUS_INTENSITY_INDEX] == 0 {
            new.v_luminous_flux_intensity = Some(other);
            new.exp[LUMINOUS_INTENSITY_INDEX] = 1;
            new.unit_map |= LUMINOUS_INTENSITY_MAP;
        } else if self.exp[LUMINOUS_INTENSITY_INDEX] == -1 {
            new.exp[LUMINOUS_INTENSITY_INDEX] = 0;
            new.v_luminous_flux_intensity = None;
            new.unit_map ^= LUMINOUS_INTENSITY_MAP;
        } else {
            if self.v_luminous_flux_intensity != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_luminous_flux_intensity.unwrap()
                );
            }
            new.exp[LUMINOUS_INTENSITY_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitLuminousIntensity> for Value {
    type Output = Value;
    fn div(self, other: UnitLuminousIntensity) -> Value {
        let mut new: Value = self;
        if self.v_luminous_flux_intensity.is_some() && self.v_luminous_flux_intensity != Some(other)
        {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[LUMINOUS_INTENSITY_INDEX] == 0 {
            new.v_luminous_flux_intensity = Some(other);
            new.unit_map |= LUMINOUS_INTENSITY_MAP;
            new.exp[LUMINOUS_INTENSITY_INDEX] = -1;
        } else if new.exp[LUMINOUS_INTENSITY_INDEX] == 1 {
            new.exp[LUMINOUS_INTENSITY_INDEX] = 0;
            new.v_luminous_flux_intensity = None;
            new.unit_map ^= LUMINOUS_INTENSITY_MAP;
        } else {
            new.exp[LUMINOUS_INTENSITY_INDEX] -= 1;
        }
        new
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
        let v = 1.1 * UnitLuminousIntensity::Candela(Metric::None);
        assert!(v.is_luminous_intensity());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 * UnitLuminousIntensity::Candela(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitLuminousIntensity::Candela(Metric::None)
            * UnitLuminousIntensity::Candela(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1
            / UnitNone::None
            / UnitLuminousIntensity::Candela(Metric::None)
            / UnitLuminousIntensity::Candela(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitLuminousIntensity::Candela(Metric::None)
            / UnitLuminousIntensity::Candela(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitLuminousIntensity::Candela(Metric::None)
            * UnitLuminousIntensity::Candela(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1
            * UnitLuminousIntensity::Candela(Metric::Kilo)
            * UnitLuminousIntensity::Candela(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1
            / UnitLuminousIntensity::Candela(Metric::Kilo)
            / UnitLuminousIntensity::Candela(Metric::None);
    }
}
