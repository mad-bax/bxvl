use std::ops::{Div, Mul};

use crate::{
    constants::{MASS_INDEX, MASS_MAP},
    units::UnitMass,
    value::Value,
};

impl Mul<UnitMass> for f64 {
    type Output = Value;
    fn mul(self, other: UnitMass) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_mass: Some(other),
            unit_map: MASS_MAP,
            ..Default::default()
        };
        ret.exp[MASS_INDEX] = 1;
        ret
    }
}

impl Div<UnitMass> for f64 {
    type Output = Value;
    fn div(self, other: UnitMass) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_mass: Some(other),
            unit_map: MASS_MAP,
            ..Default::default()
        };
        ret.exp[MASS_INDEX] = -1;
        ret
    }
}

impl Mul<UnitMass> for Value {
    type Output = Value;
    fn mul(self, other: UnitMass) -> Self::Output {
        let mut new: Value = self;
        if self.exp[MASS_INDEX] == 0 {
            new.v_mass = Some(other);
            new.exp[MASS_INDEX] = 1;
            new.unit_map |= MASS_MAP;
        } else if self.exp[MASS_INDEX] == -1 {
            new.exp[MASS_INDEX] = 0;
            new.v_mass = None;
            new.unit_map ^= MASS_MAP;
        } else {
            if self.v_mass != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_mass.unwrap()
                );
            }
            new.exp[MASS_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitMass> for Value {
    type Output = Value;
    fn div(self, other: UnitMass) -> Value {
        let mut new: Value = self;
        if self.v_mass.is_some() && new.v_mass != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[MASS_INDEX] == 0 {
            new.v_mass = Some(other);
            new.unit_map |= MASS_MAP;
            new.exp[MASS_INDEX] = -1;
        } else if new.exp[MASS_INDEX] == 1 {
            new.exp[MASS_INDEX] = 0;
            new.v_mass = None;
            new.unit_map ^= MASS_MAP;
        } else {
            new.exp[MASS_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{MASS_INDEX, MASS_MAP},
        units::{Metric, UnitMass, UnitNone},
    };

    const USED_MAP: usize = MASS_MAP;
    const USED_INDEX: usize = MASS_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitMass::Gram(Metric::None);
        assert!(v.is_mass());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitMass::Gram(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1 * UnitNone::None * UnitMass::Gram(Metric::None) * UnitMass::Gram(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None / UnitMass::Gram(Metric::None) / UnitMass::Gram(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitMass::Gram(Metric::None) / UnitMass::Gram(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitMass::Gram(Metric::None) * UnitMass::Gram(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitMass::Gram(Metric::Kilo) * UnitMass::Gram(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitMass::Gram(Metric::Kilo) / UnitMass::Gram(Metric::None);
    }
}
