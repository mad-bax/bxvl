use std::ops::Neg;

use super::Value;

pub(crate) mod type_f32;
pub(crate) mod type_f64;
pub(crate) mod type_i32;
pub(crate) mod type_i64;
pub(crate) mod type_isize;
pub(crate) mod type_u32;
pub(crate) mod type_u64;
pub(crate) mod type_usize;
pub(crate) mod type_value;

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}
