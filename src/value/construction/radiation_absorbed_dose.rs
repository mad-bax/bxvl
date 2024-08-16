use std::ops::{Div, Mul};

use crate::{
    constants::{ABSORBED_DOSE_INDEX, ABSORBED_DOSE_MAP},
    units::UnitAbsorbedDose,
    value::Value,
};

impl Mul<UnitAbsorbedDose> for f64 {
    type Output = Value;
    fn mul(self, other: UnitAbsorbedDose) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_ab_dose: Some(other),
            unit_map: ABSORBED_DOSE_MAP,
            ..Default::default()
        };
        ret.exp[ABSORBED_DOSE_INDEX] = 1;
        ret
    }
}

impl Div<UnitAbsorbedDose> for f64 {
    type Output = Value;
    fn div(self, other: UnitAbsorbedDose) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_ab_dose: Some(other),
            unit_map: ABSORBED_DOSE_MAP,
            ..Default::default()
        };
        ret.exp[ABSORBED_DOSE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitAbsorbedDose> for Value {
    type Output = Value;
    fn mul(self, other: UnitAbsorbedDose) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ABSORBED_DOSE_INDEX] == 0 {
            new.v_ab_dose = Some(other);
            new.exp[ABSORBED_DOSE_INDEX] = 1;
            new.unit_map |= ABSORBED_DOSE_MAP;
        } else if self.exp[ABSORBED_DOSE_INDEX] == -1 {
            new.exp[ABSORBED_DOSE_INDEX] = 0;
            new.v_ab_dose = None;
            new.unit_map ^= ABSORBED_DOSE_MAP;
        } else {
            if self.v_ab_dose != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_ab_dose.unwrap()
                );
            }
            new.exp[ABSORBED_DOSE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitAbsorbedDose> for Value {
    type Output = Value;
    fn div(self, other: UnitAbsorbedDose) -> Value {
        let mut new: Value = self;
        if self.v_ab_dose.is_some() && self.v_ab_dose != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ABSORBED_DOSE_INDEX] == 0 {
            new.v_ab_dose = Some(other);
            new.unit_map |= ABSORBED_DOSE_MAP;
            new.exp[ABSORBED_DOSE_INDEX] = -1;
        } else if self.exp[ABSORBED_DOSE_INDEX] == 1 {
            new.exp[ABSORBED_DOSE_INDEX] = 0;
            new.v_ab_dose = None;
            new.unit_map ^= ABSORBED_DOSE_MAP;
        } else {
            new.exp[ABSORBED_DOSE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{ABSORBED_DOSE_INDEX, ABSORBED_DOSE_MAP},
        units::{Metric, UnitAbsorbedDose, UnitNone},
    };

    const USED_MAP: usize = ABSORBED_DOSE_MAP;
    const USED_INDEX: usize = ABSORBED_DOSE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitAbsorbedDose::Gray(Metric::None);
        assert!(v.is_absorbed_dose());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitAbsorbedDose::Gray(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitAbsorbedDose::Gray(Metric::None)
            * UnitAbsorbedDose::Gray(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitAbsorbedDose::Gray(Metric::None)
            / UnitAbsorbedDose::Gray(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitAbsorbedDose::Gray(Metric::None)
            / UnitAbsorbedDose::Gray(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitAbsorbedDose::Gray(Metric::None)
            * UnitAbsorbedDose::Gray(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitAbsorbedDose::Gray(Metric::Kilo) * UnitAbsorbedDose::Gray(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitAbsorbedDose::Gray(Metric::Kilo) / UnitAbsorbedDose::Gray(Metric::None);
    }
}
