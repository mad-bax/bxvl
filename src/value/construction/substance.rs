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
    consts::{SUBSTANCE_INDEX, SUBSTANCE_MAP},
    units::UnitSubstance,
    value::Value,
};

impl Mul<UnitSubstance> for f64 {
    type Output = Value;
    fn mul(self, other: UnitSubstance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_substance: Some(other),
            unit_map: SUBSTANCE_MAP,
            ..Default::default()
        };
        ret.exp[SUBSTANCE_INDEX] = 1;
        ret
    }
}

impl Div<UnitSubstance> for f64 {
    type Output = Value;
    fn div(self, other: UnitSubstance) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_substance: Some(other),
            unit_map: SUBSTANCE_MAP,
            ..Default::default()
        };
        ret.exp[SUBSTANCE_INDEX] = -1;
        ret
    }
}

impl Mul<UnitSubstance> for Value {
    type Output = Value;
    fn mul(self, other: UnitSubstance) -> Self::Output {
        let mut new: Value = self;
        if self.exp[SUBSTANCE_INDEX] == 0 {
            new.v_substance = Some(other);
            new.exp[SUBSTANCE_INDEX] = 1;
            new.unit_map |= SUBSTANCE_MAP;
        } else if self.exp[SUBSTANCE_INDEX] == -1 {
            new.exp[SUBSTANCE_INDEX] = 0;
            new.v_substance = None;
            new.unit_map ^= SUBSTANCE_MAP;
        } else {
            if self.v_substance != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_substance.unwrap()
                );
            }
            new.exp[SUBSTANCE_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitSubstance> for Value {
    type Output = Value;
    fn div(self, other: UnitSubstance) -> Value {
        let mut new: Value = self;
        if self.v_substance.is_some() && self.v_substance != Some(other) {
            panic!("[div] Cannot decrement unit: {other} from Value {self}");
        }
        if self.exp[SUBSTANCE_INDEX] == 0 {
            new.v_substance = Some(other);
            new.unit_map |= SUBSTANCE_MAP;
            new.exp[SUBSTANCE_INDEX] = -1;
        } else if new.exp[SUBSTANCE_INDEX] == 1 {
            new.exp[SUBSTANCE_INDEX] = 0;
            new.v_substance = None;
            new.unit_map ^= SUBSTANCE_MAP;
        } else {
            new.exp[SUBSTANCE_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{SUBSTANCE_INDEX, SUBSTANCE_MAP},
        units::{Metric, UnitNone, UnitSubstance},
    };

    const USED_MAP: usize = SUBSTANCE_MAP;
    const USED_INDEX: usize = SUBSTANCE_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitSubstance::Mole(Metric::None);
        assert!(v.is_substance());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitSubstance::Mole(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitSubstance::Mole(Metric::None)
            * UnitSubstance::Mole(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitSubstance::Mole(Metric::None)
            / UnitSubstance::Mole(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitSubstance::Mole(Metric::None)
            / UnitSubstance::Mole(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitSubstance::Mole(Metric::None)
            * UnitSubstance::Mole(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitSubstance::Mole(Metric::Kilo) * UnitSubstance::Mole(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitSubstance::Mole(Metric::Kilo) / UnitSubstance::Mole(Metric::None);
    }
}
