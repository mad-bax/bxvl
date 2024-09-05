use std::ops::{Div, Mul};

use crate::{
    consts::{ILLUMINANCE_INDEX, ILLUMINANCE_MAP},
    units::UnitIlluminance,
    value::Value,
};

impl Mul<UnitIlluminance> for f64 {
    type Output = Value;
    fn mul(self, other: UnitIlluminance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_illuminance: Some(other),
            unit_map: ILLUMINANCE_MAP,
            ..Default::default()
        };
        ret.exp[ILLUMINANCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitIlluminance> for f64 {
    type Output = Value;
    fn div(self, other: UnitIlluminance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_illuminance: Some(other),
            unit_map: ILLUMINANCE_MAP,
            ..Default::default()
        };
        ret.exp[ILLUMINANCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitIlluminance> for Value {
    type Output = Value;
    fn mul(self, other: UnitIlluminance) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ILLUMINANCE_INDEX] == 0 {
            new.v_illuminance = Some(other);
            new.exp[ILLUMINANCE_INDEX] = 1;
            new.unit_map |= ILLUMINANCE_MAP;
        } else if self.exp[ILLUMINANCE_INDEX] == -1 {
            new.exp[ILLUMINANCE_INDEX] = 0;
            new.v_illuminance = None;
            new.unit_map ^= ILLUMINANCE_MAP;
        } else {
            if self.v_illuminance != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_illuminance.unwrap()
                );
            }
            new.exp[ILLUMINANCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitIlluminance> for Value {
    type Output = Value;
    fn div(self, other: UnitIlluminance) -> Value {
        let mut new: Value = self;
        if self.v_illuminance.is_some() && self.v_illuminance != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ILLUMINANCE_INDEX] == 0 {
            new.v_illuminance = Some(other);
            new.unit_map |= ILLUMINANCE_MAP;
            new.exp[ILLUMINANCE_INDEX] = -1;
        } else if new.exp[ILLUMINANCE_INDEX] == 1 {
            new.exp[ILLUMINANCE_INDEX] = 0;
            new.v_illuminance = None;
            new.unit_map ^= ILLUMINANCE_MAP;
        } else {
            new.exp[ILLUMINANCE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{ILLUMINANCE_INDEX, ILLUMINANCE_MAP},
        units::{Metric, UnitIlluminance, UnitNone},
    };

    const USED_MAP: usize = ILLUMINANCE_MAP;
    const USED_INDEX: usize = ILLUMINANCE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitIlluminance::Lux(Metric::None);
        assert!(v.is_illuminance());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitIlluminance::Lux(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitIlluminance::Lux(Metric::None)
            * UnitIlluminance::Lux(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitIlluminance::Lux(Metric::None)
            / UnitIlluminance::Lux(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitIlluminance::Lux(Metric::None)
            / UnitIlluminance::Lux(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitIlluminance::Lux(Metric::None)
            * UnitIlluminance::Lux(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitIlluminance::Lux(Metric::Kilo) * UnitIlluminance::Lux(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitIlluminance::Lux(Metric::Kilo) / UnitIlluminance::Lux(Metric::None);
    }
}
