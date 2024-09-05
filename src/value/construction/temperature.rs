use std::ops::{Div, Mul};

use crate::{
    consts::{TEMPERATURE_INDEX, TEMPERATURE_MAP},
    units::UnitTemperature,
    value::Value,
};

impl Mul<UnitTemperature> for f64 {
    type Output = Value;
    fn mul(self, other: UnitTemperature) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_temperature: Some(other),
            unit_map: TEMPERATURE_MAP,
            ..Default::default()
        };
        ret.exp[TEMPERATURE_INDEX] = 1;
        ret
    }
}

impl Div<UnitTemperature> for f64 {
    type Output = Value;
    fn div(self, other: UnitTemperature) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_temperature: Some(other),
            unit_map: TEMPERATURE_MAP,
            ..Default::default()
        };
        ret.exp[TEMPERATURE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitTemperature> for Value {
    type Output = Value;
    fn mul(self, other: UnitTemperature) -> Self::Output {
        let mut new: Value = self;
        if self.exp[TEMPERATURE_INDEX] == 0 {
            new.v_temperature = Some(other);
            new.exp[TEMPERATURE_INDEX] = 1;
            new.unit_map |= TEMPERATURE_MAP;
        } else if self.exp[TEMPERATURE_INDEX] == -1 {
            new.exp[TEMPERATURE_INDEX] = 0;
            new.v_temperature = None;
            new.unit_map ^= TEMPERATURE_MAP;
        } else {
            if self.v_temperature != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_temperature.unwrap()
                );
            }
            new.exp[TEMPERATURE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitTemperature> for Value {
    type Output = Value;
    fn div(self, other: UnitTemperature) -> Value {
        let mut new: Value = self;
        if self.v_temperature.is_some() && self.v_temperature != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[TEMPERATURE_INDEX] == 0 {
            new.v_temperature = Some(other);
            new.unit_map |= TEMPERATURE_MAP;
            new.exp[TEMPERATURE_INDEX] = -1;
        } else if new.exp[TEMPERATURE_INDEX] == 1 {
            new.exp[TEMPERATURE_INDEX] = 0;
            new.v_temperature = None;
            new.unit_map ^= TEMPERATURE_MAP;
        } else {
            new.exp[TEMPERATURE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{TEMPERATURE_INDEX, TEMPERATURE_MAP},
        units::{Metric, UnitNone, UnitTemperature},
    };

    const USED_MAP: usize = TEMPERATURE_MAP;
    const USED_INDEX: usize = TEMPERATURE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitTemperature::Kelvin(Metric::None);
        assert!(v.is_temperature());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitTemperature::Kelvin(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitTemperature::Kelvin(Metric::None)
            * UnitTemperature::Kelvin(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitTemperature::Kelvin(Metric::None)
            / UnitTemperature::Kelvin(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitTemperature::Kelvin(Metric::None)
            / UnitTemperature::Kelvin(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitTemperature::Kelvin(Metric::None)
            * UnitTemperature::Kelvin(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitTemperature::Kelvin(Metric::Kilo) * UnitTemperature::Kelvin(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitTemperature::Kelvin(Metric::Kilo) / UnitTemperature::Kelvin(Metric::None);
    }
}
