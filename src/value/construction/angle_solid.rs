use std::ops::{Div, Mul};

use crate::{
    consts::{SOLID_ANGLE_INDEX, SOLID_ANGLE_MAP},
    units::UnitSolidAngle,
    Value,
};

impl Mul<UnitSolidAngle> for f64 {
    type Output = Value;
    fn mul(self, other: UnitSolidAngle) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_solid_angle: Some(other),
            unit_map: SOLID_ANGLE_MAP,
            ..Default::default()
        };
        ret.exp[SOLID_ANGLE_INDEX] = 1;
        ret
    }
}

impl Div<UnitSolidAngle> for f64 {
    type Output = Value;
    fn div(self, other: UnitSolidAngle) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_solid_angle: Some(other),
            unit_map: SOLID_ANGLE_MAP,
            ..Default::default()
        };
        ret.exp[SOLID_ANGLE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitSolidAngle> for Value {
    type Output = Value;
    fn mul(self, other: UnitSolidAngle) -> Self::Output {
        let mut new: Value = self;
        if self.exp[SOLID_ANGLE_INDEX] == 0 {
            new.v_solid_angle = Some(other);
            new.exp[SOLID_ANGLE_INDEX] = 1;
            new.unit_map |= SOLID_ANGLE_MAP;
        } else if self.exp[SOLID_ANGLE_INDEX] == -1 {
            new.exp[SOLID_ANGLE_INDEX] = 0;
            new.v_solid_angle = None;
            new.unit_map ^= SOLID_ANGLE_MAP;
        } else {
            if self.v_solid_angle != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_solid_angle.unwrap()
                );
            }
            new.exp[SOLID_ANGLE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitSolidAngle> for Value {
    type Output = Value;
    fn div(self, other: UnitSolidAngle) -> Value {
        let mut new: Value = self;
        if self.v_solid_angle.is_some() && self.v_solid_angle != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[SOLID_ANGLE_INDEX] == 0 {
            new.v_solid_angle = Some(other);
            new.unit_map |= SOLID_ANGLE_MAP;
            new.exp[SOLID_ANGLE_INDEX] = -1;
        } else if new.exp[SOLID_ANGLE_INDEX] == 1 {
            new.exp[SOLID_ANGLE_INDEX] = 0;
            new.v_solid_angle = None;
            new.unit_map ^= SOLID_ANGLE_MAP;
        } else {
            new.exp[SOLID_ANGLE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{SOLID_ANGLE_INDEX, SOLID_ANGLE_MAP},
        units::{Metric, UnitNone, UnitSolidAngle},
    };

    const USED_MAP: usize = SOLID_ANGLE_MAP;
    const USED_INDEX: usize = SOLID_ANGLE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitSolidAngle::Steradian(Metric::None);
        assert!(v.is_solid_angle());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitSolidAngle::Steradian(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitSolidAngle::Steradian(Metric::None)
            * UnitSolidAngle::Steradian(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitSolidAngle::Steradian(Metric::None)
            / UnitSolidAngle::Steradian(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitSolidAngle::Steradian(Metric::None)
            / UnitSolidAngle::Steradian(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitSolidAngle::Steradian(Metric::None)
            * UnitSolidAngle::Steradian(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ =
            1.1 * UnitSolidAngle::Steradian(Metric::Kilo) * UnitSolidAngle::Steradian(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ =
            1.1 / UnitSolidAngle::Steradian(Metric::Kilo) / UnitSolidAngle::Steradian(Metric::None);
    }
}
