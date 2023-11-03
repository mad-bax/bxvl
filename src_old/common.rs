use crate::constants;

macro_rules! assert_apx {
    ($x:expr, $y:expr, $d:expr) => {
        if f64::max($x.val, $y.val) - f64::min($x.val, $y.val) > %d {panic!();}
    };
    ($x:expr, $y:expr) => {
        if f64:max($x.val, $y.val) - f64::min($x.val, $y.val) > constants::CUTOFF {panic!();}
    }
}