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

use std::ops::{Shr, ShrAssign};

use crate::{
    consts::{INFORMATION_INDEX, INFORMATION_MAP},
    errors::V3Error,
    units::{BaseUnit, Convert, Metric, UnitInformation},
    value::Value,
};

impl Shr<UnitInformation> for Value {
    type Output = Result<Value, V3Error>;
    fn shr(self, other: UnitInformation) -> Self::Output {
        let m = other.get_metric();

        if m < Metric::None || m == Metric::Deca || m == Metric::Hecto {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }

        let mut n: Value = self;
        if self.unit_map & INFORMATION_MAP == 0 {
            return Err(V3Error::ValueConversionError("[shr] Incompatible types"));
        }
        n.val *= n
            .v_information
            .unwrap()
            .convert(&other)
            .powi(self.exp[INFORMATION_INDEX]);
        n.v_information = Some(other);
        Ok(n)
    }
}

impl ShrAssign<UnitInformation> for Value {
    fn shr_assign(&mut self, other: UnitInformation) {
        let m = other.get_metric();

        if m < Metric::None || m == Metric::Deca || m == Metric::Hecto {
            panic!("[shr_assign] Incompatible value types");
        }

        if self.unit_map & INFORMATION_MAP == 0 {
            panic!("[shr_assign] Incompatible value types");
        }
        self.val *= self
            .v_information
            .unwrap()
            .convert(&other)
            .powi(self.exp[INFORMATION_INDEX]);
        self.v_information = Some(other);
    }
}

#[cfg(test)]
#[macro_use]
mod conversion_testing {

    use crate::units::{Metric, UnitInformation, UnitLength};

    const TEST_METRIC: [(Metric, &str); 25] = [
        (Metric::Quetta, "Q"),
        (Metric::Ronna, "R"),
        (Metric::Yotta, "Y"),
        (Metric::Zetta, "Z"),
        (Metric::Exa, "E"),
        (Metric::Peta, "P"),
        (Metric::Tera, "T"),
        (Metric::Giga, "G"),
        (Metric::Mega, "M"),
        (Metric::Kilo, "k"),
        (Metric::Hecto, "h"),
        (Metric::Deca, "da"),
        (Metric::None, ""),
        (Metric::Deci, "d"),
        (Metric::Centi, "c"),
        (Metric::Milli, "m"),
        (Metric::Micro, "μ"),
        (Metric::Nano, "n"),
        (Metric::Pico, "p"),
        (Metric::Femto, "f"),
        (Metric::Atto, "a"),
        (Metric::Zepto, "z"),
        (Metric::Yocto, "y"),
        (Metric::Ronto, "r"),
        (Metric::Quecto, "q"),
    ];

    #[test]
    #[should_panic]
    fn covert_mut_fail() {
        let mut x = 1.0 * UnitLength::Foot;
        x >>= UnitInformation::Byte(Metric::None);
    }

    #[test]
    fn covert_stat_fail() {
        assert!(((1.0 * UnitLength::Foot) >> UnitInformation::Byte(Metric::None)).is_err());
    }

    #[test]
    fn unit_conversions_stat_exp1() {
        let t1 = 4.0 * UnitInformation::Byte(Metric::None);

        for i in TEST_METRIC {
            if i.0 < Metric::Kilo && i.0 != Metric::None {
                assert!((t1 >> UnitInformation::Byte(i.0)).is_err());
                continue;
            }

            // Scale the metric value by converting
            let t2 = (t1 >> UnitInformation::Byte(i.0)).unwrap();

            // Verify that the string units are correct.
            assert_eq!(
                t2.to_string().split(' ').collect::<Vec<&str>>()[1],
                format!("{}b", i.1)
            );
        }
    }

    #[test]
    fn unit_conversions_mut_exp1() {
        let mut t1 = 4.0 * UnitInformation::Byte(Metric::None);

        for i in TEST_METRIC {
            if i.0 < Metric::Kilo {
                continue; // The rest will panic
            }

            // Scale the metric value by converting
            t1 >>= UnitInformation::Byte(i.0);

            // Reset a little bit
            t1 = 4.0 * UnitInformation::Byte(Metric::None);
        }
    }

    #[test]
    #[should_panic]
    fn unit_convert_panic() {
        let mut t1 = 4.0 * UnitInformation::Bit(Metric::None);

        t1 >>= UnitInformation::Bit(Metric::Hecto);
    }
}
