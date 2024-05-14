use std::ops::{Div, Mul};

use crate::{
    constants::{ENERGY_INDEX, ENERGY_MAP},
    units::UnitEnergy,
    values::Value,
};

impl Mul<UnitEnergy> for f64 {
    type Output = Value;
    fn mul(self, other: UnitEnergy) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_energy: Some(other),
            unit_map: ENERGY_MAP,
            ..Default::default()
        };
        ret.exp[ENERGY_INDEX] = 1;
        ret
    }
}

impl Div<UnitEnergy> for f64 {
    type Output = Value;
    fn div(self, other: UnitEnergy) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_energy: Some(other),
            unit_map: ENERGY_MAP,
            ..Default::default()
        };
        ret.exp[ENERGY_INDEX] = -1;
        ret
    }
}

impl Mul<UnitEnergy> for Value {
    type Output = Value;
    fn mul(self, other: UnitEnergy) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ENERGY_INDEX] == 0 {
            new.v_energy = Some(other);
            new.exp[ENERGY_INDEX] = 1;
            new.unit_map |= ENERGY_MAP;
        } else if self.exp[ENERGY_INDEX] == -1 {
            new.exp[ENERGY_INDEX] = 0;
            new.v_energy = None;
            new.unit_map ^= ENERGY_MAP;
        } else {
            if self.v_energy != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_energy.unwrap()
                );
            }
            new.exp[ENERGY_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitEnergy> for Value {
    type Output = Value;
    fn div(self, other: UnitEnergy) -> Value {
        let mut new: Value = self;
        if self.v_energy.is_some() && self.v_energy != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ENERGY_INDEX] == 0 {
            new.v_energy = Some(other);
            new.unit_map |= ENERGY_MAP;
            new.exp[ENERGY_INDEX] = -1;
        } else if new.exp[ENERGY_INDEX] == 1 {
            new.exp[ENERGY_INDEX] = 0;
            new.v_energy = None;
            new.unit_map ^= ENERGY_MAP;
        } else {
            new.exp[ENERGY_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{ENERGY_INDEX, ENERGY_MAP},
        units::{Metric, UnitEnergy, UnitNone},
    };

    const USED_MAP: usize = ENERGY_MAP;
    const USED_INDEX: usize = ENERGY_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitEnergy::Joule(Metric::None);
        assert!(v.is_energy());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitEnergy::Joule(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitEnergy::Joule(Metric::None)
            * UnitEnergy::Joule(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitEnergy::Joule(Metric::None)
            / UnitEnergy::Joule(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitEnergy::Joule(Metric::None)
            / UnitEnergy::Joule(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitEnergy::Joule(Metric::None)
            * UnitEnergy::Joule(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitEnergy::Joule(Metric::Kilo) * UnitEnergy::Joule(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitEnergy::Joule(Metric::Kilo) / UnitEnergy::Joule(Metric::None);
    }
}
