use std::ops::{Div, Mul};

use crate::{
    consts::{TIME_INDEX, TIME_MAP},
    units::UnitTime,
    Value,
};

impl Mul<UnitTime> for f64 {
    type Output = Value;
    fn mul(self, other: UnitTime) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_time: Some(other),
            unit_map: TIME_MAP,
            ..Default::default()
        };
        ret.exp[TIME_INDEX] = 1;
        ret
    }
}

impl Div<UnitTime> for f64 {
    type Output = Value;
    fn div(self, other: UnitTime) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_time: Some(other),
            unit_map: TIME_MAP,
            ..Default::default()
        };
        ret.exp[TIME_INDEX] = -1;
        ret
    }
}

impl Mul<UnitTime> for Value {
    type Output = Value;
    fn mul(self, other: UnitTime) -> Self::Output {
        let mut new: Value = self;
        if self.exp[TIME_INDEX] == 0 {
            new.v_time = Some(other);
            new.exp[TIME_INDEX] = 1;
            new.unit_map |= TIME_MAP;
        } else if self.exp[TIME_INDEX] == -1 {
            new.exp[TIME_INDEX] = 0;
            new.v_time = None;
            new.unit_map ^= TIME_MAP;
        } else {
            if self.v_time != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_time.unwrap()
                );
            }
            new.exp[TIME_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitTime> for Value {
    type Output = Value;
    fn div(self, other: UnitTime) -> Value {
        let mut new: Value = self;
        if self.v_time.is_some() && self.v_time != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[TIME_INDEX] == 0 {
            new.v_time = Some(other);
            new.unit_map |= TIME_MAP;
            new.exp[TIME_INDEX] = -1;
        } else if self.exp[TIME_INDEX] == 1 {
            new.exp[TIME_INDEX] = 0;
            new.v_time = None;
            new.unit_map ^= TIME_MAP;
        } else {
            new.exp[TIME_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{TIME_INDEX, TIME_MAP},
        units::{Metric, UnitNone, UnitTime},
    };

    const USED_MAP: usize = TIME_MAP;
    const USED_INDEX: usize = TIME_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitTime::Second(Metric::None);
        assert!(v.is_time());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitTime::Second(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v =
            1.1 * UnitNone::None * UnitTime::Second(Metric::None) * UnitTime::Second(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v =
            1.1 * UnitNone::None / UnitTime::Second(Metric::None) / UnitTime::Second(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v =
            1.1 * UnitNone::None * UnitTime::Second(Metric::None) / UnitTime::Second(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v =
            1.1 * UnitNone::None / UnitTime::Second(Metric::None) * UnitTime::Second(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitTime::Second(Metric::Kilo) * UnitTime::Second(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitTime::Second(Metric::Kilo) / UnitTime::Second(Metric::None);
    }
}
