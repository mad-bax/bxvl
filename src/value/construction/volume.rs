use std::ops::{Div, Mul};

use crate::{
    constants::{VOLUME_INDEX, VOLUME_MAP},
    units::UnitVolume,
    value::Value,
};

impl Mul<UnitVolume> for f64 {
    type Output = Value;
    fn mul(self, other: UnitVolume) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_volume: Some(other),
            unit_map: VOLUME_MAP,
            ..Default::default()
        };
        ret.exp[VOLUME_INDEX] = 1;
        ret
    }
}

impl Div<UnitVolume> for f64 {
    type Output = Value;
    fn div(self, other: UnitVolume) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_volume: Some(other),
            unit_map: VOLUME_MAP,
            ..Default::default()
        };
        ret.exp[VOLUME_INDEX] = -1;
        ret
    }
}

impl Mul<UnitVolume> for Value {
    type Output = Value;
    fn mul(self, other: UnitVolume) -> Self::Output {
        let mut new: Value = self;
        if self.exp[VOLUME_INDEX] == 0 {
            new.v_volume = Some(other);
            new.exp[VOLUME_INDEX] = 1;
            new.unit_map |= VOLUME_MAP;
        } else if self.exp[VOLUME_INDEX] == -1 {
            new.exp[VOLUME_INDEX] = 0;
            new.v_volume = None;
            new.unit_map ^= VOLUME_MAP;
        } else {
            if self.v_volume != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_volume.unwrap()
                );
            }
            new.exp[VOLUME_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitVolume> for Value {
    type Output = Value;
    fn div(self, other: UnitVolume) -> Value {
        let mut new: Value = self;
        if self.v_volume.is_some() && self.v_volume != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[VOLUME_INDEX] == 0 {
            new.v_volume = Some(other);
            new.unit_map |= VOLUME_MAP;
            new.exp[VOLUME_INDEX] = -1;
        } else if new.exp[VOLUME_INDEX] == 1 {
            new.exp[VOLUME_INDEX] = 0;
            new.v_volume = None;
            new.unit_map ^= VOLUME_MAP;
        } else {
            new.exp[VOLUME_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{VOLUME_INDEX, VOLUME_MAP},
        units::{Metric, UnitNone, UnitVolume},
    };

    const USED_MAP: usize = VOLUME_MAP;
    const USED_INDEX: usize = VOLUME_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitVolume::Liter(Metric::None);
        assert!(v.is_volume());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitVolume::Liter(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitVolume::Liter(Metric::None)
            * UnitVolume::Liter(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitVolume::Liter(Metric::None)
            / UnitVolume::Liter(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitVolume::Liter(Metric::None)
            / UnitVolume::Liter(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitVolume::Liter(Metric::None)
            * UnitVolume::Liter(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitVolume::Liter(Metric::Kilo) * UnitVolume::Liter(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitVolume::Liter(Metric::Kilo) / UnitVolume::Liter(Metric::None);
    }
}
