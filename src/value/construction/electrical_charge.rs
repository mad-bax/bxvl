use std::ops::{Div, Mul};

use crate::{
    constants::{ELECTRIC_CHARGE_INDEX, ELECTRIC_CHARGE_MAP},
    units::UnitElectricCharge,
    value::Value,
};

impl Mul<UnitElectricCharge> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricCharge) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_charge: Some(other),
            unit_map: ELECTRIC_CHARGE_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_CHARGE_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricCharge> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricCharge) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_charge: Some(other),
            unit_map: ELECTRIC_CHARGE_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_CHARGE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricCharge> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricCharge) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ELECTRIC_CHARGE_INDEX] == 0 {
            new.v_electric_charge = Some(other);
            new.exp[ELECTRIC_CHARGE_INDEX] = 1;
            new.unit_map |= ELECTRIC_CHARGE_MAP;
        } else if self.exp[ELECTRIC_CHARGE_INDEX] == -1 {
            new.exp[ELECTRIC_CHARGE_INDEX] = 0;
            new.v_electric_charge = None;
            new.unit_map ^= ELECTRIC_CHARGE_MAP;
        } else {
            if self.v_electric_charge != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_electric_charge.unwrap()
                );
            }
            new.exp[ELECTRIC_CHARGE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricCharge> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricCharge) -> Value {
        let mut new: Value = self;
        if self.v_electric_charge.is_some() && self.v_electric_charge != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ELECTRIC_CHARGE_INDEX] == 0 {
            new.v_electric_charge = Some(other);
            new.unit_map |= ELECTRIC_CHARGE_MAP;
            new.exp[ELECTRIC_CHARGE_INDEX] = -1;
        } else if new.exp[ELECTRIC_CHARGE_INDEX] == 1 {
            new.exp[ELECTRIC_CHARGE_INDEX] = 0;
            new.v_electric_charge = None;
            new.unit_map ^= ELECTRIC_CHARGE_MAP;
        } else {
            new.exp[ELECTRIC_CHARGE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{ELECTRIC_CHARGE_INDEX, ELECTRIC_CHARGE_MAP},
        units::{Metric, UnitElectricCharge, UnitNone},
    };

    const USED_MAP: usize = ELECTRIC_CHARGE_MAP;
    const USED_INDEX: usize = ELECTRIC_CHARGE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitElectricCharge::Coulomb(Metric::None);
        assert!(v.is_electric_charge());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitElectricCharge::Coulomb(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitElectricCharge::Coulomb(Metric::None)
            * UnitElectricCharge::Coulomb(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitElectricCharge::Coulomb(Metric::None)
            / UnitElectricCharge::Coulomb(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitElectricCharge::Coulomb(Metric::None)
            / UnitElectricCharge::Coulomb(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitElectricCharge::Coulomb(Metric::None)
            * UnitElectricCharge::Coulomb(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1
            * UnitElectricCharge::Coulomb(Metric::Kilo)
            * UnitElectricCharge::Coulomb(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1
            / UnitElectricCharge::Coulomb(Metric::Kilo)
            / UnitElectricCharge::Coulomb(Metric::None);
    }
}
