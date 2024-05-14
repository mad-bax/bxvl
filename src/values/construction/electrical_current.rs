use std::ops::{Div, Mul};

use crate::{
    constants::{ELECTRIC_CURRENT_INDEX, ELECTRIC_CURRENT_MAP},
    units::UnitElectricCurrent,
    values::Value,
};

impl Mul<UnitElectricCurrent> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricCurrent) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_current: Some(other),
            unit_map: ELECTRIC_CURRENT_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_CURRENT_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricCurrent> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricCurrent) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_current: Some(other),
            unit_map: ELECTRIC_CURRENT_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_CURRENT_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricCurrent> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricCurrent) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ELECTRIC_CURRENT_INDEX] == 0 {
            new.v_electric_current = Some(other);
            new.exp[ELECTRIC_CURRENT_INDEX] = 1;
            new.unit_map |= ELECTRIC_CURRENT_MAP;
        } else if self.exp[ELECTRIC_CURRENT_INDEX] == -1 {
            new.exp[ELECTRIC_CURRENT_INDEX] = 0;
            new.v_electric_current = None;
            new.unit_map ^= ELECTRIC_CURRENT_MAP;
        } else {
            if self.v_electric_current != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_electric_current.unwrap()
                );
            }
            new.exp[ELECTRIC_CURRENT_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricCurrent> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricCurrent) -> Value {
        let mut new: Value = self;
        if self.v_electric_current.is_some() && self.v_electric_current != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ELECTRIC_CURRENT_INDEX] == 0 {
            new.v_electric_current = Some(other);
            new.unit_map |= ELECTRIC_CURRENT_MAP;
            new.exp[ELECTRIC_CURRENT_INDEX] = -1;
        } else if new.exp[ELECTRIC_CURRENT_INDEX] == 1 {
            new.exp[ELECTRIC_CURRENT_INDEX] = 0;
            new.v_electric_current = None;
            new.unit_map ^= ELECTRIC_CURRENT_MAP;
        } else {
            new.exp[ELECTRIC_CURRENT_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{ELECTRIC_CURRENT_INDEX, ELECTRIC_CURRENT_MAP},
        units::{Metric, UnitElectricCurrent, UnitNone},
    };

    const USED_MAP: usize = ELECTRIC_CURRENT_MAP;
    const USED_INDEX: usize = ELECTRIC_CURRENT_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitElectricCurrent::Ampere(Metric::None);
        assert!(v.is_electric_current());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitElectricCurrent::Ampere(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitElectricCurrent::Ampere(Metric::None)
            * UnitElectricCurrent::Ampere(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitElectricCurrent::Ampere(Metric::None)
            / UnitElectricCurrent::Ampere(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitElectricCurrent::Ampere(Metric::None)
            / UnitElectricCurrent::Ampere(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitElectricCurrent::Ampere(Metric::None)
            * UnitElectricCurrent::Ampere(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1
            * UnitElectricCurrent::Ampere(Metric::Kilo)
            * UnitElectricCurrent::Ampere(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1
            / UnitElectricCurrent::Ampere(Metric::Kilo)
            / UnitElectricCurrent::Ampere(Metric::None);
    }
}
