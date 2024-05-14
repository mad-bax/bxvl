use std::ops::{Div, Mul};

use crate::{
    constants::{CATALYTIC_ACTIVITY_INDEX, CATALYTIC_ACTIVITY_MAP},
    units::UnitCatalyticActivity,
    values::Value,
};

impl Mul<UnitCatalyticActivity> for f64 {
    type Output = Value;
    fn mul(self, other: UnitCatalyticActivity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_catalytic: Some(other),
            unit_map: CATALYTIC_ACTIVITY_MAP,
            ..Default::default()
        };
        ret.exp[CATALYTIC_ACTIVITY_INDEX] = 1;
        ret
    }
}

impl Div<UnitCatalyticActivity> for f64 {
    type Output = Value;
    fn div(self, other: UnitCatalyticActivity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_catalytic: Some(other),
            unit_map: CATALYTIC_ACTIVITY_MAP,
            ..Default::default()
        };
        ret.exp[CATALYTIC_ACTIVITY_INDEX] = -1;
        ret
    }
}

impl Mul<UnitCatalyticActivity> for Value {
    type Output = Value;
    fn mul(self, other: UnitCatalyticActivity) -> Self::Output {
        let mut new: Value = self;
        if self.exp[CATALYTIC_ACTIVITY_INDEX] == 0 {
            new.v_catalytic = Some(other);
            new.exp[CATALYTIC_ACTIVITY_INDEX] = 1;
            new.unit_map |= CATALYTIC_ACTIVITY_MAP;
        } else if self.exp[CATALYTIC_ACTIVITY_INDEX] == -1 {
            new.exp[CATALYTIC_ACTIVITY_INDEX] = 0;
            new.v_catalytic = None;
            new.unit_map ^= CATALYTIC_ACTIVITY_MAP;
        } else {
            if self.v_catalytic != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_catalytic.unwrap()
                );
            }
            new.exp[CATALYTIC_ACTIVITY_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitCatalyticActivity> for Value {
    type Output = Value;
    fn div(self, other: UnitCatalyticActivity) -> Value {
        let mut new: Value = self;
        if self.v_catalytic.is_some() && self.v_catalytic != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if self.exp[CATALYTIC_ACTIVITY_INDEX] == 0 {
            new.v_catalytic = Some(other);
            new.unit_map |= CATALYTIC_ACTIVITY_MAP;
            new.exp[CATALYTIC_ACTIVITY_INDEX] = -1;
        } else if new.exp[CATALYTIC_ACTIVITY_INDEX] == 1 {
            new.exp[CATALYTIC_ACTIVITY_INDEX] = 0;
            new.v_catalytic = None;
            new.unit_map ^= CATALYTIC_ACTIVITY_MAP;
        } else {
            new.exp[CATALYTIC_ACTIVITY_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        constants::{CATALYTIC_ACTIVITY_INDEX, CATALYTIC_ACTIVITY_MAP},
        units::{Metric, UnitCatalyticActivity, UnitNone},
    };

    const USED_MAP: usize = CATALYTIC_ACTIVITY_MAP;
    const USED_INDEX: usize = CATALYTIC_ACTIVITY_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitCatalyticActivity::Katal(Metric::None);
        assert!(v.is_catalytic_activity());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitCatalyticActivity::Katal(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitCatalyticActivity::Katal(Metric::None)
            * UnitCatalyticActivity::Katal(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitCatalyticActivity::Katal(Metric::None)
            / UnitCatalyticActivity::Katal(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitCatalyticActivity::Katal(Metric::None)
            / UnitCatalyticActivity::Katal(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitCatalyticActivity::Katal(Metric::None)
            * UnitCatalyticActivity::Katal(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1
            * UnitCatalyticActivity::Katal(Metric::Kilo)
            * UnitCatalyticActivity::Katal(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1
            / UnitCatalyticActivity::Katal(Metric::Kilo)
            / UnitCatalyticActivity::Katal(Metric::None);
    }
}
