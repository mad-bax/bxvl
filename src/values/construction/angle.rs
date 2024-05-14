use std::ops::{Div, Mul};

use crate::{
    constants::{ANGLE_INDEX, ANGLE_MAP},
    units::UnitAngle,
    values::Value,
};

impl Mul<UnitAngle> for f64 {
    type Output = Value;
    fn mul(self, other: UnitAngle) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_angle: Some(other),
            unit_map: ANGLE_MAP,
            ..Default::default()
        };
        ret.exp[ANGLE_INDEX] = 1;
        ret
    }
}

impl Div<UnitAngle> for f64 {
    type Output = Value;
    fn div(self, other: UnitAngle) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_angle: Some(other),
            unit_map: ANGLE_MAP,
            ..Default::default()
        };
        ret.exp[ANGLE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitAngle> for Value {
    type Output = Value;
    fn mul(self, other: UnitAngle) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ANGLE_INDEX] == 0 {
            new.v_angle = Some(other);
            new.exp[ANGLE_INDEX] = 1;
            new.unit_map |= ANGLE_MAP;
        } else if self.exp[ANGLE_INDEX] == -1 {
            new.exp[ANGLE_INDEX] = 0;
            new.v_angle = None;
            new.unit_map ^= ANGLE_MAP;
        } else {
            if self.v_angle != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_angle.unwrap()
                );
            }
            new.exp[ANGLE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitAngle> for Value {
    type Output = Value;
    fn div(self, other: UnitAngle) -> Value {
        let mut new: Value = self;
        if self.v_angle.is_some() && self.v_angle != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ANGLE_INDEX] == 0 {
            new.v_angle = Some(other);
            new.unit_map |= ANGLE_MAP;
            new.exp[ANGLE_INDEX] = -1;
        } else if self.exp[ANGLE_INDEX] == 1 {
            new.exp[ANGLE_INDEX] = 0;
            new.v_angle = None;
            new.unit_map ^= ANGLE_MAP;
        } else {
            new.exp[ANGLE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{ANGLE_INDEX, ANGLE_MAP},
        units::{Metric, UnitAngle, UnitNone},
    };

    const USED_MAP: usize = ANGLE_MAP;
    const USED_INDEX: usize = ANGLE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitAngle::Radian(Metric::None);
        assert!(v.is_angle());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitAngle::Radian(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitAngle::Radian(Metric::None)
            * UnitAngle::Radian(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitAngle::Radian(Metric::None)
            / UnitAngle::Radian(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitAngle::Radian(Metric::None)
            / UnitAngle::Radian(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitAngle::Radian(Metric::None)
            * UnitAngle::Radian(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitAngle::Radian(Metric::Kilo) * UnitAngle::Radian(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitAngle::Radian(Metric::Kilo) / UnitAngle::Radian(Metric::None);
    }
}
