use std::ops::{Div, Mul};

use crate::{
    consts::{SOUND_INDEX, SOUND_MAP},
    units::UnitSound,
    Value,
};

impl Mul<UnitSound> for f64 {
    type Output = Value;
    fn mul(self, other: UnitSound) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_sound: Some(other),
            unit_map: SOUND_MAP,
            ..Default::default()
        };
        ret.exp[SOUND_INDEX] = 1;
        ret
    }
}

impl Div<UnitSound> for f64 {
    type Output = Value;
    fn div(self, other: UnitSound) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_sound: Some(other),
            unit_map: SOUND_MAP,
            ..Default::default()
        };
        ret.exp[SOUND_INDEX] = -1;
        ret
    }
}

impl Mul<UnitSound> for Value {
    type Output = Value;
    fn mul(self, other: UnitSound) -> Self::Output {
        let mut new: Value = self;
        if self.exp[SOUND_INDEX] == 0 {
            new.v_sound = Some(other);
            new.exp[SOUND_INDEX] = 1;
            new.unit_map |= SOUND_MAP;
        } else if self.exp[SOUND_INDEX] == -1 {
            new.exp[SOUND_INDEX] = 0;
            new.v_sound = None;
            new.unit_map ^= SOUND_MAP;
        } else {
            if self.v_sound != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_sound.unwrap()
                );
            }
            new.exp[SOUND_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitSound> for Value {
    type Output = Value;
    fn div(self, other: UnitSound) -> Value {
        let mut new: Value = self;
        if self.v_sound.is_some() && self.v_sound != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[SOUND_INDEX] == 0 {
            new.v_sound = Some(other);
            new.unit_map |= SOUND_MAP;
            new.exp[SOUND_INDEX] = -1;
        } else if new.exp[SOUND_INDEX] == 1 {
            new.exp[SOUND_INDEX] = 0;
            new.v_sound = None;
            new.unit_map ^= SOUND_MAP;
        } else {
            new.exp[SOUND_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{SOUND_INDEX, SOUND_MAP},
        units::{Metric, UnitNone, UnitSound},
    };

    const USED_MAP: usize = SOUND_MAP;
    const USED_INDEX: usize = SOUND_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitSound::Bel(Metric::None);
        assert!(v.is_sound());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitSound::Bel(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1 * UnitNone::None * UnitSound::Bel(Metric::None) * UnitSound::Bel(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None / UnitSound::Bel(Metric::None) / UnitSound::Bel(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitSound::Bel(Metric::None) / UnitSound::Bel(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitSound::Bel(Metric::None) * UnitSound::Bel(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitSound::Bel(Metric::Kilo) * UnitSound::Bel(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitSound::Bel(Metric::Kilo) / UnitSound::Bel(Metric::None);
    }
}
