use std::ops::{Div, Mul};

use crate::{
    consts::{ELECTRIC_CONDUCTANCE_INDEX, ELECTRIC_CONDUCTANCE_MAP},
    units::UnitElectricConductance,
    Value,
};

impl Mul<UnitElectricConductance> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricConductance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_conductance: Some(other),
            unit_map: ELECTRIC_CONDUCTANCE_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_CONDUCTANCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricConductance> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricConductance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_electric_conductance: Some(other),
            unit_map: ELECTRIC_CONDUCTANCE_MAP,
            ..Default::default()
        };
        ret.exp[ELECTRIC_CONDUCTANCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricConductance> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricConductance) -> Self::Output {
        let mut new: Value = self;
        if self.exp[ELECTRIC_CONDUCTANCE_INDEX] == 0 {
            new.v_electric_conductance = Some(other);
            new.exp[ELECTRIC_CONDUCTANCE_INDEX] = 1;
            new.unit_map |= ELECTRIC_CONDUCTANCE_MAP;
        } else if self.exp[ELECTRIC_CONDUCTANCE_INDEX] == -1 {
            new.exp[ELECTRIC_CONDUCTANCE_INDEX] = 0;
            new.v_electric_conductance = None;
            new.unit_map ^= ELECTRIC_CONDUCTANCE_MAP;
        } else {
            if self.v_electric_conductance != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_electric_conductance.unwrap()
                );
            }
            new.exp[ELECTRIC_CONDUCTANCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricConductance> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricConductance) -> Value {
        let mut new: Value = self;
        if self.v_electric_conductance.is_some() && self.v_electric_conductance != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[ELECTRIC_CONDUCTANCE_INDEX] == 0 {
            new.v_electric_conductance = Some(other);
            new.unit_map |= ELECTRIC_CONDUCTANCE_MAP;
            new.exp[ELECTRIC_CONDUCTANCE_INDEX] = -1;
        } else if self.exp[ELECTRIC_CONDUCTANCE_INDEX] == 1 {
            new.exp[ELECTRIC_CONDUCTANCE_INDEX] = 0;
            new.v_electric_conductance = None;
            new.unit_map ^= ELECTRIC_CONDUCTANCE_MAP;
        } else {
            new.exp[ELECTRIC_CONDUCTANCE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{ELECTRIC_CONDUCTANCE_INDEX, ELECTRIC_CONDUCTANCE_MAP},
        units::{Metric, UnitElectricConductance, UnitNone},
    };

    const USED_MAP: usize = ELECTRIC_CONDUCTANCE_MAP;
    const USED_INDEX: usize = ELECTRIC_CONDUCTANCE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitElectricConductance::Siemens(Metric::None);
        assert!(v.is_conductance());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitElectricConductance::Siemens(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitElectricConductance::Siemens(Metric::None)
            * UnitElectricConductance::Siemens(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitElectricConductance::Siemens(Metric::None)
            / UnitElectricConductance::Siemens(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitElectricConductance::Siemens(Metric::None)
            / UnitElectricConductance::Siemens(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitElectricConductance::Siemens(Metric::None)
            * UnitElectricConductance::Siemens(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1
            * UnitElectricConductance::Siemens(Metric::Kilo)
            * UnitElectricConductance::Siemens(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1
            / UnitElectricConductance::Siemens(Metric::Kilo)
            / UnitElectricConductance::Siemens(Metric::None);
    }
}
