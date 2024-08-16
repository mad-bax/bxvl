use std::ops::{Div, Mul};

use crate::{
    constants::{ELECTRIC_POTENTIAL_INDEX, ELECTRIC_POTENTIAL_MAP},
    units::UnitElectricPotential,
    value::Value,
};

impl Mul<UnitElectricPotential> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricPotential) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_potential: Some(other),
            unit_map: ELECTRIC_POTENTIAL_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricPotential> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricPotential) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_potential: Some(other),
            unit_map: ELECTRIC_POTENTIAL_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_POTENTIAL_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricPotential> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricPotential) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ELECTRIC_POTENTIAL_INDEX] == 0 {
            new.v_electric_potential = Some(other);
            new.exp[ELECTRIC_POTENTIAL_INDEX] = 1;
            new.unit_map |= ELECTRIC_POTENTIAL_MAP;
        } else if self.exp[ELECTRIC_POTENTIAL_INDEX] == -1 {
            new.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
            new.v_electric_potential = None;
            new.unit_map ^= ELECTRIC_POTENTIAL_MAP;
        } else {
            if self.v_electric_potential != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_electric_potential.unwrap()
                );
            }
            new.exp[ELECTRIC_POTENTIAL_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricPotential> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricPotential) -> Value {
        let mut new: Value = self;
        if self.v_electric_potential.is_some() && self.v_electric_potential != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ELECTRIC_POTENTIAL_INDEX] == 0 {
            new.v_electric_potential = Some(other);
            new.unit_map |= ELECTRIC_POTENTIAL_MAP;
            new.exp[ELECTRIC_POTENTIAL_INDEX] = -1;
        } else if new.exp[ELECTRIC_POTENTIAL_INDEX] == 1 {
            new.exp[ELECTRIC_POTENTIAL_INDEX] = 0;
            new.v_electric_potential = None;
            new.unit_map ^= ELECTRIC_POTENTIAL_MAP;
        } else {
            new.exp[ELECTRIC_POTENTIAL_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{ELECTRIC_POTENTIAL_INDEX, ELECTRIC_POTENTIAL_MAP},
        units::{Metric, UnitElectricPotential, UnitNone},
    };

    const USED_MAP: usize = ELECTRIC_POTENTIAL_MAP;
    const USED_INDEX: usize = ELECTRIC_POTENTIAL_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitElectricPotential::Volt(Metric::None);
        assert!(v.is_electric_potential());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitElectricPotential::Volt(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitElectricPotential::Volt(Metric::None)
            * UnitElectricPotential::Volt(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitElectricPotential::Volt(Metric::None)
            / UnitElectricPotential::Volt(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitElectricPotential::Volt(Metric::None)
            / UnitElectricPotential::Volt(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitElectricPotential::Volt(Metric::None)
            * UnitElectricPotential::Volt(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1
            * UnitElectricPotential::Volt(Metric::Kilo)
            * UnitElectricPotential::Volt(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1
            / UnitElectricPotential::Volt(Metric::Kilo)
            / UnitElectricPotential::Volt(Metric::None);
    }
}
