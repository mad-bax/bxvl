use std::ops::{Div, Mul};

use crate::{
    consts::{FREQUENCY_INDEX, FREQUENCY_MAP},
    units::UnitFrequency,
    value::Value,
};

impl Mul<UnitFrequency> for f64 {
    type Output = Value;
    fn mul(self, other: UnitFrequency) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_frequency: Some(other),
            unit_map: FREQUENCY_MAP,
            ..Default::default()
        };
        ret.exp[FREQUENCY_INDEX] = 1;
        ret
    }
}

impl Div<UnitFrequency> for f64 {
    type Output = Value;
    fn div(self, other: UnitFrequency) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_frequency: Some(other),
            unit_map: FREQUENCY_MAP,
            ..Default::default()
        };
        ret.exp[FREQUENCY_INDEX] = -1;
        ret
    }
}

impl Mul<UnitFrequency> for Value {
    type Output = Value;
    fn mul(self, other: UnitFrequency) -> Self::Output {
        let mut new: Value = self;
        if self.exp[FREQUENCY_INDEX] == 0 {
            new.v_frequency = Some(other);
            new.exp[FREQUENCY_INDEX] = 1;
            new.unit_map |= FREQUENCY_MAP;
        } else if self.exp[FREQUENCY_INDEX] == -1 {
            new.exp[FREQUENCY_INDEX] = 0;
            new.v_frequency = None;
            new.unit_map ^= FREQUENCY_MAP;
        } else {
            if self.v_frequency != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_frequency.unwrap()
                );
            }
            new.exp[FREQUENCY_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitFrequency> for Value {
    type Output = Value;
    fn div(self, other: UnitFrequency) -> Value {
        let mut new: Value = self;
        if self.v_frequency.is_some() && self.v_frequency != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[FREQUENCY_INDEX] == 0 {
            new.v_frequency = Some(other);
            new.unit_map |= FREQUENCY_MAP;
            new.exp[FREQUENCY_INDEX] = -1;
        } else if new.exp[FREQUENCY_INDEX] == 1 {
            new.exp[FREQUENCY_INDEX] = 0;
            new.v_frequency = None;
            new.unit_map ^= FREQUENCY_MAP;
        } else {
            new.exp[FREQUENCY_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{FREQUENCY_INDEX, FREQUENCY_MAP},
        units::{Metric, UnitFrequency, UnitNone},
    };

    const USED_MAP: usize = FREQUENCY_MAP;
    const USED_INDEX: usize = FREQUENCY_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitFrequency::Hertz(Metric::None);
        assert!(v.is_frequency());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitFrequency::Hertz(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitFrequency::Hertz(Metric::None)
            * UnitFrequency::Hertz(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitFrequency::Hertz(Metric::None)
            / UnitFrequency::Hertz(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitFrequency::Hertz(Metric::None)
            / UnitFrequency::Hertz(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitFrequency::Hertz(Metric::None)
            * UnitFrequency::Hertz(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitFrequency::Hertz(Metric::Kilo) * UnitFrequency::Hertz(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitFrequency::Hertz(Metric::Kilo) / UnitFrequency::Hertz(Metric::None);
    }
}
