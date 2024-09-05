use std::ops::{Div, Mul};

use crate::{
    consts::{PRESSURE_INDEX, PRESSURE_MAP},
    units::UnitPressure,
    value::Value,
};

impl Mul<UnitPressure> for f64 {
    type Output = Value;
    fn mul(self, other: UnitPressure) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_pressure: Some(other),
            unit_map: PRESSURE_MAP,
            ..Default::default()
        };
        ret.exp[PRESSURE_INDEX] = 1;
        ret
    }
}

impl Div<UnitPressure> for f64 {
    type Output = Value;
    fn div(self, other: UnitPressure) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_pressure: Some(other),
            unit_map: PRESSURE_MAP,
            ..Default::default()
        };
        ret.exp[PRESSURE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitPressure> for Value {
    type Output = Value;
    fn mul(self, other: UnitPressure) -> Self::Output {
        let mut new: Value = self;
        if self.exp[PRESSURE_INDEX] == 0 {
            new.v_pressure = Some(other);
            new.exp[PRESSURE_INDEX] = 1;
            new.unit_map |= PRESSURE_MAP;
        } else if self.exp[PRESSURE_INDEX] == -1 {
            new.exp[PRESSURE_INDEX] = 0;
            new.v_pressure = None;
            new.unit_map ^= PRESSURE_MAP;
        } else {
            if self.v_pressure != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_pressure.unwrap()
                );
            }
            new.exp[PRESSURE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitPressure> for Value {
    type Output = Value;
    fn div(self, other: UnitPressure) -> Value {
        let mut new: Value = self;
        if self.v_pressure.is_some() && self.v_pressure != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[PRESSURE_INDEX] == 0 {
            new.v_pressure = Some(other);
            new.unit_map |= PRESSURE_MAP;
            new.exp[PRESSURE_INDEX] = -1;
        } else if new.exp[PRESSURE_INDEX] == 1 {
            new.exp[PRESSURE_INDEX] = 0;
            new.v_pressure = None;
            new.unit_map ^= PRESSURE_MAP;
        } else {
            new.exp[PRESSURE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{PRESSURE_INDEX, PRESSURE_MAP},
        units::{Metric, UnitNone, UnitPressure},
    };

    const USED_MAP: usize = PRESSURE_MAP;
    const USED_INDEX: usize = PRESSURE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitPressure::Pascal(Metric::None);
        assert!(v.is_pressure());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitPressure::Pascal(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitPressure::Pascal(Metric::None)
            * UnitPressure::Pascal(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitPressure::Pascal(Metric::None)
            / UnitPressure::Pascal(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitPressure::Pascal(Metric::None)
            / UnitPressure::Pascal(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitPressure::Pascal(Metric::None)
            * UnitPressure::Pascal(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitPressure::Pascal(Metric::Kilo) * UnitPressure::Pascal(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitPressure::Pascal(Metric::Kilo) / UnitPressure::Pascal(Metric::None);
    }
}
