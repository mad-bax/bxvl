use std::ops::{Div, Mul};

use crate::{
    constants::{LENGTH_INDEX, LENGTH_MAP},
    units::UnitLength,
    value::Value,
};

impl Mul<UnitLength> for f64 {
    type Output = Value;
    fn mul(self, other: UnitLength) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_length: Some(other),
            unit_map: LENGTH_MAP,
            ..Default::default()
        };
        ret.exp[LENGTH_INDEX] = 1;
        ret
    }
}

impl Div<UnitLength> for f64 {
    type Output = Value;
    fn div(self, other: UnitLength) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_length: Some(other),
            unit_map: LENGTH_MAP,
            ..Default::default()
        };
        ret.exp[LENGTH_INDEX] = -1;
        ret
    }
}

impl Mul<UnitLength> for Value {
    type Output = Value;
    fn mul(self, other: UnitLength) -> Self::Output {
        let mut new: Value = self;
        if self.exp[LENGTH_INDEX] == 0 {
            new.v_length = Some(other);
            new.exp[LENGTH_INDEX] = 1;
            new.unit_map |= LENGTH_MAP;
        } else if self.exp[LENGTH_INDEX] == -1 {
            new.exp[LENGTH_INDEX] = 0;
            new.v_length = None;
            new.unit_map ^= LENGTH_MAP;
        } else {
            if self.v_length != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_length.unwrap()
                );
            }
            new.exp[LENGTH_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitLength> for Value {
    type Output = Value;
    fn div(self, other: UnitLength) -> Value {
        let mut new: Value = self;
        if self.v_length.is_some() && self.v_length != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[LENGTH_INDEX] == 0 {
            new.v_length = Some(other);
            new.unit_map |= LENGTH_MAP;
            new.exp[LENGTH_INDEX] = -1;
        } else if self.exp[LENGTH_INDEX] == 1 {
            new.exp[LENGTH_INDEX] = 0;
            new.v_length = None;
            new.unit_map ^= LENGTH_MAP;
        } else {
            new.exp[LENGTH_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{LENGTH_INDEX, LENGTH_MAP},
        units::{Metric, UnitLength, UnitNone},
    };

    const USED_MAP: usize = LENGTH_MAP;
    const USED_INDEX: usize = LENGTH_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitLength::Meter(Metric::None);
        assert!(v.is_length());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitLength::Meter(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitLength::Meter(Metric::None)
            * UnitLength::Meter(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitLength::Meter(Metric::None)
            / UnitLength::Meter(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitLength::Meter(Metric::None)
            / UnitLength::Meter(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitLength::Meter(Metric::None)
            * UnitLength::Meter(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitLength::Meter(Metric::Kilo) * UnitLength::Meter(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitLength::Meter(Metric::Kilo) / UnitLength::Meter(Metric::None);
    }
}
