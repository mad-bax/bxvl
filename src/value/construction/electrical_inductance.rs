use std::ops::{Div, Mul};

use crate::{
    consts::{INDUCTANCE_INDEX, INDUCTANCE_MAP},
    units::UnitElectricInductance,
    value::Value,
};

impl Mul<UnitElectricInductance> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricInductance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_inductance: Some(other),
            unit_map: INDUCTANCE_MAP,
            ..Default::default()
        };
        ret.exp[INDUCTANCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricInductance> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricInductance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_inductance: Some(other),
            unit_map: INDUCTANCE_MAP,
            ..Default::default()
        };
        ret.exp[INDUCTANCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricInductance> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricInductance) -> Self::Output {
        let mut new: Value = self;
        if self.exp[INDUCTANCE_INDEX] == 0 {
            new.v_inductance = Some(other);
            new.exp[INDUCTANCE_INDEX] = 1;
            new.unit_map |= INDUCTANCE_MAP;
        } else if self.exp[INDUCTANCE_INDEX] == -1 {
            new.exp[INDUCTANCE_INDEX] = 0;
            new.v_inductance = None;
            new.unit_map ^= INDUCTANCE_MAP;
        } else {
            if self.v_inductance != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_inductance.unwrap()
                );
            }
            new.exp[INDUCTANCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricInductance> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricInductance) -> Value {
        let mut new: Value = self;
        if self.v_inductance.is_some() && self.v_inductance != Some(other) {
            panic!("[div] Cannot decrement unit: {other} from Value {self}");
        }
        if new.exp[INDUCTANCE_INDEX] == 0 {
            new.v_inductance = Some(other);
            new.unit_map |= INDUCTANCE_MAP;
            new.exp[INDUCTANCE_INDEX] = -1;
        } else if new.exp[INDUCTANCE_INDEX] == 1 {
            new.exp[INDUCTANCE_INDEX] = 0;
            new.v_inductance = None;
            new.unit_map ^= INDUCTANCE_MAP;
        } else {
            new.exp[INDUCTANCE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{INDUCTANCE_INDEX, INDUCTANCE_MAP},
        units::{Metric, UnitElectricInductance, UnitNone},
    };

    const USED_MAP: usize = INDUCTANCE_MAP;
    const USED_INDEX: usize = INDUCTANCE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitElectricInductance::Henry(Metric::None);
        assert!(v.is_inductance());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitElectricInductance::Henry(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitElectricInductance::Henry(Metric::None)
            * UnitElectricInductance::Henry(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitElectricInductance::Henry(Metric::None)
            / UnitElectricInductance::Henry(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitElectricInductance::Henry(Metric::None)
            / UnitElectricInductance::Henry(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitElectricInductance::Henry(Metric::None)
            * UnitElectricInductance::Henry(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1
            * UnitElectricInductance::Henry(Metric::Kilo)
            * UnitElectricInductance::Henry(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1
            / UnitElectricInductance::Henry(Metric::Kilo)
            / UnitElectricInductance::Henry(Metric::None);
    }
}
