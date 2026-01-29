use std::ops::{Div, Mul};

use crate::{
    consts::{POWER_INDEX, POWER_MAP},
    units::UnitPower,
    value::Value,
};

impl Mul<UnitPower> for f64 {
    type Output = Value;
    fn mul(self, other: UnitPower) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_power: Some(other),
            unit_map: POWER_MAP,
            ..Default::default()
        };
        ret.exp[POWER_INDEX] = 1;
        ret
    }
}

impl Div<UnitPower> for f64 {
    type Output = Value;
    fn div(self, other: UnitPower) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_power: Some(other),
            unit_map: POWER_MAP,
            ..Default::default()
        };
        ret.exp[POWER_INDEX] = -1;
        ret
    }
}

impl Mul<UnitPower> for Value {
    type Output = Value;
    fn mul(self, other: UnitPower) -> Self::Output {
        let mut new: Value = self;
        if self.exp[POWER_INDEX] == 0 {
            new.v_power = Some(other);
            new.exp[POWER_INDEX] = 1;
            new.unit_map |= POWER_MAP;
        } else if self.exp[POWER_INDEX] == -1 {
            new.exp[POWER_INDEX] = 0;
            new.v_power = None;
            new.unit_map ^= POWER_MAP;
        } else {
            if self.v_power != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_power.unwrap()
                );
            }
            new.exp[POWER_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitPower> for Value {
    type Output = Value;
    fn div(self, other: UnitPower) -> Value {
        let mut new: Value = self;
        if self.v_power.is_some() && self.v_power != Some(other) {
            panic!("[div] Cannot decrement unit: {other} from Value {self}");
        }
        if new.exp[POWER_INDEX] == 0 {
            new.v_power = Some(other);
            new.unit_map |= POWER_MAP;
            new.exp[POWER_INDEX] = -1;
        } else if new.exp[POWER_INDEX] == 1 {
            new.exp[POWER_INDEX] = 0;
            new.v_power = None;
            new.unit_map ^= POWER_MAP;
        } else {
            new.exp[POWER_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{POWER_INDEX, POWER_MAP},
        units::{Metric, UnitNone, UnitPower},
    };

    const USED_MAP: usize = POWER_MAP;
    const USED_INDEX: usize = POWER_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitPower::Watt(Metric::None);
        assert!(v.is_power());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitPower::Watt(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v =
            1.1 * UnitNone::None * UnitPower::Watt(Metric::None) * UnitPower::Watt(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v =
            1.1 * UnitNone::None / UnitPower::Watt(Metric::None) / UnitPower::Watt(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v =
            1.1 * UnitNone::None * UnitPower::Watt(Metric::None) / UnitPower::Watt(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v =
            1.1 * UnitNone::None / UnitPower::Watt(Metric::None) * UnitPower::Watt(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitPower::Watt(Metric::Kilo) * UnitPower::Watt(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitPower::Watt(Metric::Kilo) / UnitPower::Watt(Metric::None);
    }
}
