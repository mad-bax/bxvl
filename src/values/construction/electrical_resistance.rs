use std::ops::{Div, Mul};

use crate::{
    constants::{RESISTANCE_INDEX, RESISTANCE_MAP},
    units::UnitElectricResistance,
    values::Value,
};

impl Mul<UnitElectricResistance> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricResistance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_resistance: Some(other),
            unit_map: RESISTANCE_MAP,
            ..Default::default()
        };
        ret.exp[RESISTANCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricResistance> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricResistance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_resistance: Some(other),
            unit_map: RESISTANCE_MAP,
            ..Default::default()
        };
        ret.exp[RESISTANCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricResistance> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricResistance) -> Self::Output {
        let mut new: Value = self;
        if self.exp[RESISTANCE_INDEX] == 0 {
            new.v_resistance = Some(other);
            new.exp[RESISTANCE_INDEX] = 1;
            new.unit_map |= RESISTANCE_MAP;
        } else if self.exp[RESISTANCE_INDEX] == -1 {
            new.exp[RESISTANCE_INDEX] = 0;
            new.v_resistance = None;
            new.unit_map ^= RESISTANCE_MAP;
        } else {
            if self.v_resistance != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_resistance.unwrap()
                );
            }
            new.exp[RESISTANCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricResistance> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricResistance) -> Value {
        let mut new: Value = self;
        if self.v_resistance.is_some() && self.v_resistance != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[RESISTANCE_INDEX] == 0 {
            new.v_resistance = Some(other);
            new.unit_map |= RESISTANCE_MAP;
            new.exp[RESISTANCE_INDEX] = -1;
        } else if new.exp[RESISTANCE_INDEX] == 1 {
            new.exp[RESISTANCE_INDEX] = 0;
            new.v_resistance = None;
            new.unit_map ^= RESISTANCE_MAP;
        } else {
            new.exp[RESISTANCE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{RESISTANCE_INDEX, RESISTANCE_MAP},
        units::{Metric, UnitElectricResistance, UnitNone},
    };

    const USED_MAP: usize = RESISTANCE_MAP;
    const USED_INDEX: usize = RESISTANCE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitElectricResistance::Ohm(Metric::None);
        assert!(v.is_resistance());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitElectricResistance::Ohm(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitElectricResistance::Ohm(Metric::None)
            * UnitElectricResistance::Ohm(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitElectricResistance::Ohm(Metric::None)
            / UnitElectricResistance::Ohm(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitElectricResistance::Ohm(Metric::None)
            / UnitElectricResistance::Ohm(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitElectricResistance::Ohm(Metric::None)
            * UnitElectricResistance::Ohm(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1
            * UnitElectricResistance::Ohm(Metric::Kilo)
            * UnitElectricResistance::Ohm(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1
            / UnitElectricResistance::Ohm(Metric::Kilo)
            / UnitElectricResistance::Ohm(Metric::None);
    }
}
