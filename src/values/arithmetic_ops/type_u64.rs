use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::values::Value;

impl Add<u64> for Value {
    type Output = Value;
    fn add(self, rhs: u64) -> Value {
        let mut n: Value = self;
        n.val += rhs as f64;
        n
    }
}

impl AddAssign<u64> for Value {
    fn add_assign(&mut self, rhs: u64) {
        self.val += rhs as f64;
    }
}

impl Sub<u64> for Value {
    type Output = Value;
    fn sub(self, rhs: u64) -> Value {
        let mut n: Value = self;
        n.val -= rhs as f64;
        n
    }
}

impl SubAssign<u64> for Value {
    fn sub_assign(&mut self, rhs: u64) {
        self.val -= rhs as f64;
    }
}

impl Mul<u64> for Value {
    type Output = Value;
    fn mul(self, rhs: u64) -> Value {
        let mut n: Value = self;
        n.val *= rhs as f64;
        n
    }
}

impl MulAssign<u64> for Value {
    fn mul_assign(&mut self, rhs: u64) {
        self.val *= rhs as f64;
    }
}

impl Div<u64> for Value {
    type Output = Value;
    fn div(self, rhs: u64) -> Value {
        let mut n: Value = self;
        n.val /= rhs as f64;
        n
    }
}

impl DivAssign<u64> for Value {
    fn div_assign(&mut self, rhs: u64) {
        self.val /= rhs as f64;
    }
}

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
