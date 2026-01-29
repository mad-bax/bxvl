use std::ops::{Div, Mul};

use crate::{
    consts::{INFORMATION_INDEX, INFORMATION_MAP},
    units::{Metric, UnitInformation},
    value::Value,
};

impl Mul<UnitInformation> for f64 {
    type Output = Value;
    fn mul(self, other: UnitInformation) -> Self::Output {
        match other {
            UnitInformation::Bit(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Bit(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Bit(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
            UnitInformation::Byte(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Byte(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Byte(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
        }

        let mut ret = Value {
            val: self,
            v_information: Some(other),
            unit_map: INFORMATION_MAP,
            ..Default::default()
        };
        ret.exp[INFORMATION_INDEX] = 1;
        ret
    }
}

impl Div<UnitInformation> for f64 {
    type Output = Value;
    fn div(self, other: UnitInformation) -> Self::Output {
        match other {
            UnitInformation::Bit(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Bit(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Bit(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
            UnitInformation::Byte(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Byte(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Byte(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
        }

        let mut ret = Value {
            val: self,
            v_information: Some(other),
            unit_map: INFORMATION_MAP,
            ..Default::default()
        };
        ret.exp[INFORMATION_INDEX] = -1;
        ret
    }
}

impl Mul<UnitInformation> for Value {
    type Output = Value;
    fn mul(self, other: UnitInformation) -> Self::Output {
        match other {
            UnitInformation::Bit(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Bit(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Bit(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
            UnitInformation::Byte(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Byte(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Byte(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
        }

        let mut new: Value = self;
        if self.exp[INFORMATION_INDEX] == 0 {
            new.v_information = Some(other);
            new.exp[INFORMATION_INDEX] = 1;
            new.unit_map |= INFORMATION_MAP;
        } else if self.exp[INFORMATION_INDEX] == -1 {
            new.exp[INFORMATION_INDEX] = 0;
            new.v_information = None;
            new.unit_map ^= INFORMATION_MAP;
        } else {
            if self.v_information != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_information.unwrap()
                );
            }
            new.exp[INFORMATION_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitInformation> for Value {
    type Output = Value;
    fn div(self, other: UnitInformation) -> Value {
        match other {
            UnitInformation::Bit(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Bit(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Bit(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
            UnitInformation::Byte(Metric::Hecto) => panic!("Unsupported information metric"),
            UnitInformation::Byte(Metric::Deca) => panic!("Unsupported information metric"),
            UnitInformation::Byte(m) => {
                if m < Metric::None {
                    panic!("Unsupported information metric")
                }
            }
        }

        let mut new: Value = self;
        if self.v_information.is_some() && self.v_information != Some(other) {
            panic!("[div] Cannot decrement unit: {other} from Value {self}");
        }
        if new.exp[INFORMATION_INDEX] == 0 {
            new.v_information = Some(other);
            new.unit_map |= INFORMATION_MAP;
            new.exp[INFORMATION_INDEX] = -1;
        } else if new.exp[INFORMATION_INDEX] == 1 {
            new.exp[INFORMATION_INDEX] = 0;
            new.v_information = None;
            new.unit_map ^= INFORMATION_MAP;
        } else {
            new.exp[INFORMATION_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{INFORMATION_INDEX, INFORMATION_MAP},
        units::{Metric, UnitInformation, UnitNone},
    };

    const USED_MAP: usize = INFORMATION_MAP;
    const USED_INDEX: usize = INFORMATION_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitInformation::Byte(Metric::None);
        assert!(v.is_information());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitInformation::Byte(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitInformation::Byte(Metric::None)
            * UnitInformation::Byte(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitInformation::Byte(Metric::None)
            / UnitInformation::Byte(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitInformation::Byte(Metric::None)
            / UnitInformation::Byte(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitInformation::Byte(Metric::None)
            * UnitInformation::Byte(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val_2() {
        let v = 1.1
            * UnitNone::None
            * UnitInformation::Bit(Metric::None)
            * UnitInformation::Bit(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val_2() {
        let v = 1.1 * UnitNone::None
            / UnitInformation::Bit(Metric::None)
            / UnitInformation::Bit(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val_2() {
        let v = 1.1 * UnitNone::None * UnitInformation::Bit(Metric::None)
            / UnitInformation::Bit(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val_2() {
        let v = 1.1 * UnitNone::None / UnitInformation::Bit(Metric::None)
            * UnitInformation::Bit(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitInformation::Byte(Metric::Kilo) * UnitInformation::Byte(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitInformation::Byte(Metric::Kilo) / UnitInformation::Byte(Metric::None);
    }

    #[test]
    #[should_panic]
    fn info_metric_01() {
        let _ = 1.1 * UnitInformation::Byte(Metric::Hecto);
    }

    #[test]
    #[should_panic]
    fn info_metric_02() {
        let _ = 1.1 * UnitInformation::Byte(Metric::Deca);
    }

    #[test]
    #[should_panic]
    fn info_metric_03() {
        let _ = 1.1 * UnitInformation::Byte(Metric::Pico);
    }

    #[test]
    #[should_panic]
    fn info_metric_04() {
        let _ = 1.1 / UnitInformation::Byte(Metric::Hecto);
    }

    #[test]
    #[should_panic]
    fn info_metric_05() {
        let _ = 1.1 / UnitInformation::Byte(Metric::Deca);
    }

    #[test]
    #[should_panic]
    fn info_metric_06() {
        let _ = 1.1 / UnitInformation::Byte(Metric::Pico);
    }

    #[test]
    #[should_panic]
    fn info_metric_07() {
        let _ = 1.1 * UnitInformation::Bit(Metric::Hecto);
    }

    #[test]
    #[should_panic]
    fn info_metric_08() {
        let _ = 1.1 * UnitInformation::Bit(Metric::Deca);
    }

    #[test]
    #[should_panic]
    fn info_metric_09() {
        let _ = 1.1 * UnitInformation::Bit(Metric::Pico);
    }

    #[test]
    #[should_panic]
    fn info_metric_10() {
        let _ = 1.1 / UnitInformation::Bit(Metric::Hecto);
    }

    #[test]
    #[should_panic]
    fn info_metric_11() {
        let _ = 1.1 / UnitInformation::Bit(Metric::Deca);
    }

    #[test]
    #[should_panic]
    fn info_metric_12() {
        let _ = 1.1 / UnitInformation::Bit(Metric::Pico);
    }

    #[test]
    #[should_panic]
    fn info_metric_13() {
        let _ = 1.1 * UnitInformation::Byte(Metric::None) * UnitInformation::Byte(Metric::Hecto);
    }

    #[test]
    #[should_panic]
    fn info_metric_14() {
        let _ = 1.1 * UnitInformation::Byte(Metric::None) * UnitInformation::Byte(Metric::Deca);
    }

    #[test]
    #[should_panic]
    fn info_metric_15() {
        let _ = 1.1 * UnitInformation::Byte(Metric::None) * UnitInformation::Byte(Metric::Pico);
    }

    #[test]
    #[should_panic]
    fn info_metric_16() {
        let _ = 1.1 / UnitInformation::Byte(Metric::None) / UnitInformation::Byte(Metric::Hecto);
    }

    #[test]
    #[should_panic]
    fn info_metric_17() {
        let _ = 1.1 / UnitInformation::Byte(Metric::None) / UnitInformation::Byte(Metric::Deca);
    }

    #[test]
    #[should_panic]
    fn info_metric_18() {
        let _ = 1.1 / UnitInformation::Byte(Metric::None) / UnitInformation::Byte(Metric::Pico);
    }

    #[test]
    #[should_panic]
    fn info_metric_19() {
        let _ = 1.1 * UnitInformation::Bit(Metric::None) * UnitInformation::Bit(Metric::Hecto);
    }

    #[test]
    #[should_panic]
    fn info_metric_20() {
        let _ = 1.1 * UnitInformation::Bit(Metric::None) * UnitInformation::Bit(Metric::Deca);
    }

    #[test]
    #[should_panic]
    fn info_metric_21() {
        let _ = 1.1 * UnitInformation::Bit(Metric::None) * UnitInformation::Bit(Metric::Pico);
    }

    #[test]
    #[should_panic]
    fn info_metric_22() {
        let _ = 1.1 / UnitInformation::Bit(Metric::None) / UnitInformation::Bit(Metric::Hecto);
    }

    #[test]
    #[should_panic]
    fn info_metric_23() {
        let _ = 1.1 / UnitInformation::Bit(Metric::None) / UnitInformation::Bit(Metric::Deca);
    }

    #[test]
    #[should_panic]
    fn info_metric_24() {
        let _ = 1.1 / UnitInformation::Bit(Metric::None) / UnitInformation::Bit(Metric::Pico);
    }
}
