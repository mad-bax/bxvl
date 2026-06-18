/*  BXVL
 *  Copyright (C) 2026 Bax Bradley
 *
 *  This library is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Lesser General Public License as published
 *  by the Free Software Foundation, either version 3 of the License, or
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
    consts::{CAPACITANCE_INDEX, CAPACITANCE_MAP},
    units::UnitElectricCapacitance,
    value::Value,
};

impl Mul<UnitElectricCapacitance> for f64 {
    type Output = Value;
    fn mul(self, other: UnitElectricCapacitance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_capacitance: Some(other),
            unit_map: CAPACITANCE_MAP,
            ..Default::default()
        };
        ret.exp[CAPACITANCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitElectricCapacitance> for f64 {
    type Output = Value;
    fn div(self, other: UnitElectricCapacitance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_capacitance: Some(other),
            unit_map: CAPACITANCE_MAP,
            ..Default::default()
        };
        ret.exp[CAPACITANCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitElectricCapacitance> for Value {
    type Output = Value;
    fn mul(self, other: UnitElectricCapacitance) -> Self::Output {
        let mut new: Value = self;
        if self.exp[CAPACITANCE_INDEX] == 0 {
            new.v_capacitance = Some(other);
            new.exp[CAPACITANCE_INDEX] = 1;
            new.unit_map |= CAPACITANCE_MAP;
        } else if self.exp[CAPACITANCE_INDEX] == -1 {
            new.exp[CAPACITANCE_INDEX] = 0;
            new.v_capacitance = None;
            new.unit_map ^= CAPACITANCE_MAP;
        } else {
            if self.v_capacitance != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_capacitance.unwrap()
                );
            }
            new.exp[CAPACITANCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitElectricCapacitance> for Value {
    type Output = Value;
    fn div(self, other: UnitElectricCapacitance) -> Value {
        let mut new: Value = self;
        if self.v_capacitance.is_some() && self.v_capacitance != Some(other) {
            panic!("[div] Cannot decrement unit: {other} from Value {self}");
        }
        if self.exp[CAPACITANCE_INDEX] == 0 {
            new.v_capacitance = Some(other);
            new.unit_map |= CAPACITANCE_MAP;
            new.exp[CAPACITANCE_INDEX] = -1;
        } else if self.exp[CAPACITANCE_INDEX] == 1 {
            new.exp[CAPACITANCE_INDEX] = 0;
            new.v_capacitance = None;
            new.unit_map ^= CAPACITANCE_MAP;
        } else {
            new.exp[CAPACITANCE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{CAPACITANCE_INDEX, CAPACITANCE_MAP},
        units::{Metric, UnitElectricCapacitance, UnitNone},
    };

    const USED_MAP: usize = CAPACITANCE_MAP;
    const USED_INDEX: usize = CAPACITANCE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitElectricCapacitance::Farad(Metric::None);
        assert!(v.is_capacitance());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitElectricCapacitance::Farad(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitElectricCapacitance::Farad(Metric::None)
            * UnitElectricCapacitance::Farad(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitElectricCapacitance::Farad(Metric::None)
            / UnitElectricCapacitance::Farad(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitElectricCapacitance::Farad(Metric::None)
            / UnitElectricCapacitance::Farad(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitElectricCapacitance::Farad(Metric::None)
            * UnitElectricCapacitance::Farad(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1
            * UnitElectricCapacitance::Farad(Metric::Kilo)
            * UnitElectricCapacitance::Farad(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1
            / UnitElectricCapacitance::Farad(Metric::Kilo)
            / UnitElectricCapacitance::Farad(Metric::None);
    }
}
