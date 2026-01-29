use std::ops::{Div, Mul};

use crate::{
    consts::{RADIOACTIVITY_INDEX, RADIOACTIVITY_MAP},
    units::UnitRadioactivity,
    value::Value,
};

impl Mul<UnitRadioactivity> for f64 {
    type Output = Value;
    fn mul(self, other: UnitRadioactivity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_radioactivity: Some(other),
            unit_map: RADIOACTIVITY_MAP,
            ..Default::default()
        };
        ret.exp[RADIOACTIVITY_INDEX] = 1;
        ret
    }
}

impl Div<UnitRadioactivity> for f64 {
    type Output = Value;
    fn div(self, other: UnitRadioactivity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_radioactivity: Some(other),
            unit_map: RADIOACTIVITY_MAP,
            ..Default::default()
        };
        ret.exp[RADIOACTIVITY_INDEX] = -1;
        ret
    }
}

impl Mul<UnitRadioactivity> for Value {
    type Output = Value;
    fn mul(self, other: UnitRadioactivity) -> Self::Output {
        let mut new: Value = self;
        if self.exp[RADIOACTIVITY_INDEX] == 0 {
            new.v_radioactivity = Some(other);
            new.exp[RADIOACTIVITY_INDEX] = 1;
            new.unit_map |= RADIOACTIVITY_MAP;
        } else if self.exp[RADIOACTIVITY_INDEX] == -1 {
            new.exp[RADIOACTIVITY_INDEX] = 0;
            new.v_radioactivity = None;
            new.unit_map ^= RADIOACTIVITY_MAP;
        } else {
            if self.v_radioactivity != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_radioactivity.unwrap()
                );
            }
            new.exp[RADIOACTIVITY_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitRadioactivity> for Value {
    type Output = Value;
    fn div(self, other: UnitRadioactivity) -> Value {
        let mut new: Value = self;
        if self.v_radioactivity.is_some() && self.v_radioactivity != Some(other) {
            panic!("[div] Cannot decrement unit: {other} from Value {self}");
        }
        if new.exp[RADIOACTIVITY_INDEX] == 0 {
            new.v_radioactivity = Some(other);
            new.unit_map |= RADIOACTIVITY_MAP;
            new.exp[RADIOACTIVITY_INDEX] = -1;
        } else if new.exp[RADIOACTIVITY_INDEX] == 1 {
            new.exp[RADIOACTIVITY_INDEX] = 0;
            new.v_radioactivity = None;
            new.unit_map ^= RADIOACTIVITY_MAP;
        } else {
            new.exp[RADIOACTIVITY_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{RADIOACTIVITY_INDEX, RADIOACTIVITY_MAP},
        units::{Metric, UnitNone, UnitRadioactivity},
    };

    const USED_MAP: usize = RADIOACTIVITY_MAP;
    const USED_INDEX: usize = RADIOACTIVITY_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitRadioactivity::Becquerel(Metric::None);
        assert!(v.is_radioactivity());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitRadioactivity::Becquerel(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitRadioactivity::Becquerel(Metric::None)
            * UnitRadioactivity::Becquerel(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitRadioactivity::Becquerel(Metric::None)
            / UnitRadioactivity::Becquerel(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitRadioactivity::Becquerel(Metric::None)
            / UnitRadioactivity::Becquerel(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitRadioactivity::Becquerel(Metric::None)
            * UnitRadioactivity::Becquerel(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1
            * UnitRadioactivity::Becquerel(Metric::Kilo)
            * UnitRadioactivity::Becquerel(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1
            / UnitRadioactivity::Becquerel(Metric::Kilo)
            / UnitRadioactivity::Becquerel(Metric::None);
    }
}
