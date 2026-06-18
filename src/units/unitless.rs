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

use std::fmt::Display;

use super::{BaseUnit, Convert, Metric, UnitNone};

impl Display for UnitNone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "",
                Self::Percentage => "%",
            }
        )
    }
}

impl From<UnitNone> for String {
    fn from(val: UnitNone) -> Self {
        val.to_string()
    }
}

impl<T> Convert<T> for UnitNone
where
    T: BaseUnit,
{
    fn convert(&self, other: &T) -> f64 {
        other.scale() * other.base()
    }
}

impl BaseUnit for UnitNone {
    fn scale(&self) -> f64 {
        1.0
    }

    fn base(&self) -> f64 {
        1.0
    }

    fn get_metric(&self) -> Metric {
        Metric::None
    }
}

#[cfg(test)]
mod unitless_testing {
    use crate::{
        consts::LENGTH_MILE_TO_METER,
        units::{BaseUnit, Convert, Metric, UnitLength, UnitNone},
    };

    /// Unit Time Comparison Base
    ///
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_time_base_comparison() {
        assert_eq!(UnitNone::None.base(), 1.0);
        assert_eq!(UnitNone::Percentage.base(), 1.0);
    }

    #[test]
    fn unit_angle_to_string() {
        for i in [(UnitNone::None, ""), (UnitNone::Percentage, "%")] {
            assert_eq!(&i.0.to_string(), i.1);
            let t: String = i.0.into();
            assert_eq!(t, i.1.to_string());
        }
    }

    #[test]
    fn unit_angle_scale() {
        for i in [
            (UnitNone::None, Metric::None),
            (UnitNone::None, Metric::None),
        ] {
            assert_eq!(i.0.get_metric(), i.1);
        }

        for i in [(UnitNone::None, 1.0), (UnitNone::None, 1.0)] {
            assert_eq!(i.0.scale(), i.1);
        }
    }

    #[test]
    fn unitless_convert() {
        assert_eq!(
            (UnitNone::None).convert(&UnitLength::Mile),
            LENGTH_MILE_TO_METER
        );
    }
}
