/*  BXVL
 *  Copyright (C) 2026 Bax Bradley
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
            panic!("[div] Cannot decrement unit: {other} from Value {self}");
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
