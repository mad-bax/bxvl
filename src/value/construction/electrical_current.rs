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
    consts::{ELECTRIC_CURRENT_INDEX, ELECTRIC_CURRENT_MAP},
    units::UnitElectricCurrent,
    value::Value,
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
            panic!("[div] Cannot decrement unit: {other} from Value {self}");
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
        consts::{ELECTRIC_CURRENT_INDEX, ELECTRIC_CURRENT_MAP},
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
