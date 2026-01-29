use std::ops::Neg;

use super::Value;

/// Module for operations of a [`Value`] against a [`f32`]
pub(crate) mod type_f32;
/// Module for operations of a [`Value`] against a [`f64`]
pub(crate) mod type_f64;
/// Module for operations of a [`Value`] against a [`i32`]
pub(crate) mod type_i32;
/// Module for operations of a [`Value`] against a [`i64`]
pub(crate) mod type_i64;
/// Module for operations of a [`Value`] against a [`isize`]
pub(crate) mod type_isize;
/// Module for operations of a [`Value`] against a [`u32`]
pub(crate) mod type_u32;
/// Module for operations of a [`Value`] against a [`u64`]
pub(crate) mod type_u64;
/// Module for operations of a [`Value`] against a [`usize`]
pub(crate) mod type_usize;
/// Module for operations of a [`Value`] against a [`value`]
pub(crate) mod type_value;

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

/// The [`create_defs`] macro is used to define operations for primitive type operations with
/// values.
#[macro_export]
macro_rules! create_defs {
    ($type_name:ty) => {
        impl Add<$type_name> for Value {
            type Output = Value;
            fn add(self, rhs: $type_name) -> Value {
                let mut n: Value = self;
                n.val += rhs as f64;
                n
            }
        }

        impl AddAssign<$type_name> for Value {
            fn add_assign(&mut self, rhs: $type_name) {
                self.val += rhs as f64;
            }
        }

        impl Sub<$type_name> for Value {
            type Output = Value;
            fn sub(self, rhs: $type_name) -> Value {
                let mut n: Value = self;
                n.val -= rhs as f64;
                n
            }
        }

        impl SubAssign<$type_name> for Value {
            fn sub_assign(&mut self, rhs: $type_name) {
                self.val -= rhs as f64;
            }
        }

        impl Mul<$type_name> for Value {
            type Output = Value;
            fn mul(self, rhs: $type_name) -> Value {
                let mut n: Value = self;
                n.val *= rhs as f64;
                n
            }
        }

        impl MulAssign<$type_name> for Value {
            fn mul_assign(&mut self, rhs: $type_name) {
                self.val *= rhs as f64;
            }
        }

        impl Div<$type_name> for Value {
            type Output = Value;
            fn div(self, rhs: $type_name) -> Value {
                let mut n: Value = self;
                n.val /= rhs as f64;
                n
            }
        }

        impl DivAssign<$type_name> for Value {
            fn div_assign(&mut self, rhs: $type_name) {
                self.val /= rhs as f64;
            }
        }

        impl Add<$type_name> for &Value {
            type Output = Value;
            fn add(self, rhs: $type_name) -> Value {
                let mut n: Value = self.clone();
                n.val += rhs as f64;
                n
            }
        }

        impl AddAssign<$type_name> for &mut Value {
            fn add_assign(&mut self, rhs: $type_name) {
                self.val += rhs as f64;
            }
        }

        impl Sub<$type_name> for &Value {
            type Output = Value;
            fn sub(self, rhs: $type_name) -> Value {
                let mut n: Value = self.clone();
                n.val -= rhs as f64;
                n
            }
        }

        impl SubAssign<$type_name> for &mut Value {
            fn sub_assign(&mut self, rhs: $type_name) {
                self.val -= rhs as f64;
            }
        }

        impl Mul<$type_name> for &Value {
            type Output = Value;
            fn mul(self, rhs: $type_name) -> Value {
                let mut n: Value = self.clone();
                n.val *= rhs as f64;
                n
            }
        }

        impl MulAssign<$type_name> for &mut Value {
            fn mul_assign(&mut self, rhs: $type_name) {
                self.val *= rhs as f64;
            }
        }

        impl Div<$type_name> for &Value {
            type Output = Value;
            fn div(self, rhs: $type_name) -> Value {
                let mut n: Value = self.clone();
                n.val /= rhs as f64;
                n
            }
        }

        impl DivAssign<$type_name> for &mut Value {
            fn div_assign(&mut self, rhs: $type_name) {
                self.val /= rhs as f64;
            }
        }
    };
}

#[cfg(test)]
mod testing {
    use std::ops::Neg;

    use crate::units::{Metric, UnitForce};

    #[test]
    fn negative_value() {
        let x = 4.0 * UnitForce::Newton(Metric::None);

        assert_eq!(x.neg(), -x);
    }
}
