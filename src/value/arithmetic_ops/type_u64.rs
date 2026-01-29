use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::value::Value;

use crate::create_defs;

create_defs!(u64);

#[cfg(test)]
mod arithmetic_ops_testing {
    use crate::units::{Metric, UnitLength};

    #[test]
    fn add() {
        let v1 = 12.0 * UnitLength::Meter(Metric::None);
        let v2 = v1 + 4_u64;
        assert_eq!(v1 + 4.0, v2);
    }

    #[test]
    fn add_assign() {
        let mut v1 = 12.0 * UnitLength::Meter(Metric::None);
        v1 += 4_u64;
        assert_eq!(v1, 16.0);
    }

    #[test]
    fn sub() {
        let v1 = 12.0 * UnitLength::Meter(Metric::None);
        let v2 = v1 - 4_u64;
        assert_eq!(v1 - 4.0, v2);
    }

    #[test]
    fn sub_assign() {
        let mut v1 = 12.0 * UnitLength::Meter(Metric::None);
        v1 -= 4_u64;
        assert_eq!(v1, 8.0);
    }

    #[test]
    fn mul() {
        let v1 = 12.0 * UnitLength::Meter(Metric::None);
        let v2 = v1 * 4_u64;
        assert_eq!(v1 * 4.0, v2);
    }

    #[test]
    fn mul_assign() {
        let mut v1 = 12.0 * UnitLength::Meter(Metric::None);
        v1 *= 4_u64;
        assert_eq!(v1, 48.0);
    }

    #[test]
    fn div() {
        let v1 = 12.0 * UnitLength::Meter(Metric::None);
        let v2 = v1 / 4_u64;
        assert_eq!(v1 / 4.0, v2);
    }

    #[test]
    fn div_assign() {
        let mut v1 = 12.0 * UnitLength::Meter(Metric::None);
        v1 /= 4_u64;
        assert_eq!(v1, 3.0);
    }
}
