use std::{ops::Div, process::Output};

use crate::values::Value;

pub struct Length;
pub struct Time;
pub struct Velocity;
pub struct Acceleration;
pub struct Mass;
pub struct Default;

impl<Length> Div<Value<Time>> for Value<Length> {
    type Output = Value<Velocity>;
    fn div(self, rhs: Value<Time>) -> Output {
        
    }
}