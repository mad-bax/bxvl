use std::ops::{Div, Mul};

use crate::{
    consts::{RADIOACTIVITY_EXPOSURE_INDEX, RADIOACTIVITY_EXPOSURE_MAP},
    units::UnitRadioactivityExposure,
    value::Value,
};

impl Mul<UnitRadioactivityExposure> for f64 {
    type Output = Value;
    fn mul(self, other: UnitRadioactivityExposure) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_radioactivity_exposure: Some(other),
            unit_map: RADIOACTIVITY_EXPOSURE_MAP,
            ..Default::default()
        };
        ret.exp[RADIOACTIVITY_EXPOSURE_INDEX] = 1;
        ret
    }
}

impl Div<UnitRadioactivityExposure> for f64 {
    type Output = Value;
    fn div(self, other: UnitRadioactivityExposure) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_radioactivity_exposure: Some(other),
            unit_map: RADIOACTIVITY_EXPOSURE_MAP,
            ..Default::default()
        };
        ret.exp[RADIOACTIVITY_EXPOSURE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitRadioactivityExposure> for Value {
    type Output = Value;
    fn mul(self, other: UnitRadioactivityExposure) -> Self::Output {
        let mut new: Value = self;
        if self.exp[RADIOACTIVITY_EXPOSURE_INDEX] == 0 {
            new.v_radioactivity_exposure = Some(other);
            new.exp[RADIOACTIVITY_EXPOSURE_INDEX] = 1;
            new.unit_map |= RADIOACTIVITY_EXPOSURE_MAP;
        } else if self.exp[RADIOACTIVITY_EXPOSURE_INDEX] == -1 {
            new.exp[RADIOACTIVITY_EXPOSURE_INDEX] = 0;
            new.v_radioactivity_exposure = None;
            new.unit_map ^= RADIOACTIVITY_EXPOSURE_MAP;
        } else {
            if self.v_radioactivity_exposure != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_radioactivity_exposure.unwrap()
                );
            }
            new.exp[RADIOACTIVITY_EXPOSURE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitRadioactivityExposure> for Value {
    type Output = Value;
    fn div(self, other: UnitRadioactivityExposure) -> Value {
        let mut new: Value = self;
        if self.v_radioactivity_exposure.is_some() && self.v_radioactivity_exposure != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[RADIOACTIVITY_EXPOSURE_INDEX] == 0 {
            new.v_radioactivity_exposure = Some(other);
            new.unit_map |= RADIOACTIVITY_EXPOSURE_MAP;
            new.exp[RADIOACTIVITY_EXPOSURE_INDEX] = -1;
        } else if new.exp[RADIOACTIVITY_EXPOSURE_INDEX] == 1 {
            new.exp[RADIOACTIVITY_EXPOSURE_INDEX] = 0;
            new.v_radioactivity_exposure = None;
            new.unit_map ^= RADIOACTIVITY_EXPOSURE_MAP;
        } else {
            new.exp[RADIOACTIVITY_EXPOSURE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{RADIOACTIVITY_EXPOSURE_INDEX, RADIOACTIVITY_EXPOSURE_MAP},
        units::{Metric, UnitNone, UnitRadioactivityExposure},
    };

    const USED_MAP: usize = RADIOACTIVITY_EXPOSURE_MAP;
    const USED_INDEX: usize = RADIOACTIVITY_EXPOSURE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitRadioactivityExposure::Sievert(Metric::None);
        assert!(v.is_equivalent_dose());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitRadioactivityExposure::Sievert(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitRadioactivityExposure::Sievert(Metric::None)
            * UnitRadioactivityExposure::Sievert(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitRadioactivityExposure::Sievert(Metric::None)
            / UnitRadioactivityExposure::Sievert(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitRadioactivityExposure::Sievert(Metric::None)
            / UnitRadioactivityExposure::Sievert(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitRadioactivityExposure::Sievert(Metric::None)
            * UnitRadioactivityExposure::Sievert(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1
            * UnitRadioactivityExposure::Sievert(Metric::Kilo)
            * UnitRadioactivityExposure::Sievert(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1
            / UnitRadioactivityExposure::Sievert(Metric::Kilo)
            / UnitRadioactivityExposure::Sievert(Metric::None);
    }
}
