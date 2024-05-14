use std::ops::{Div, Mul};

use crate::{
    constants::{FORCE_INDEX, FORCE_MAP},
    units::UnitForce,
    values::Value,
};

impl Mul<UnitForce> for f64 {
    type Output = Value;
    fn mul(self, other: UnitForce) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_force: Some(other),
            unit_map: FORCE_MAP,
            ..Default::default()
        };
        ret.exp[FORCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitForce> for f64 {
    type Output = Value;
    fn div(self, other: UnitForce) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_force: Some(other),
            unit_map: FORCE_MAP,
            ..Default::default()
        };
        ret.exp[FORCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitForce> for Value {
    type Output = Value;
    fn mul(self, other: UnitForce) -> Self::Output {
        let mut new: Value = self;
        if self.exp[FORCE_INDEX] == 0 {
            new.v_force = Some(other);
            new.exp[FORCE_INDEX] = 1;
            new.unit_map |= FORCE_MAP;
        } else if self.exp[FORCE_INDEX] == -1 {
            new.exp[FORCE_INDEX] = 0;
            new.v_force = None;
            new.unit_map ^= FORCE_MAP;
        } else {
            if self.v_force != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_force.unwrap()
                );
            }
            new.exp[FORCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitForce> for Value {
    type Output = Value;
    fn div(self, other: UnitForce) -> Value {
        let mut new: Value = self;
        if self.v_force.is_some() && self.v_force != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[FORCE_INDEX] == 0 {
            new.v_force = Some(other);
            new.unit_map |= FORCE_MAP;
            new.exp[FORCE_INDEX] = -1;
        } else if new.exp[FORCE_INDEX] == 1 {
            new.exp[FORCE_INDEX] = 0;
            new.v_force = None;
            new.unit_map ^= FORCE_MAP;
        } else {
            new.exp[FORCE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{FORCE_INDEX, FORCE_MAP},
        units::{Metric, UnitForce, UnitNone},
    };

    const USED_MAP: usize = FORCE_MAP;
    const USED_INDEX: usize = FORCE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitForce::Newton(Metric::None);
        assert!(v.is_force());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitForce::Newton(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitForce::Newton(Metric::None)
            * UnitForce::Newton(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitForce::Newton(Metric::None)
            / UnitForce::Newton(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitForce::Newton(Metric::None)
            / UnitForce::Newton(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitForce::Newton(Metric::None)
            * UnitForce::Newton(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitForce::Newton(Metric::Kilo) * UnitForce::Newton(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitForce::Newton(Metric::Kilo) / UnitForce::Newton(Metric::None);
    }
}
