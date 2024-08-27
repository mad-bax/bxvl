use std::ops::{Div, Mul};

use crate::{
    consts::{LUMINOUS_FLUX_INDEX, LUMINOUS_FLUX_MAP},
    units::UnitLuminousFlux,
    Value,
};

impl Mul<UnitLuminousFlux> for f64 {
    type Output = Value;
    fn mul(self, other: UnitLuminousFlux) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_luminous_flux: Some(other),
            unit_map: LUMINOUS_FLUX_MAP,
            ..Default::default()
        };
        ret.exp[LUMINOUS_FLUX_INDEX] = 1;
        ret
    }
}

impl Div<UnitLuminousFlux> for f64 {
    type Output = Value;
    fn div(self, other: UnitLuminousFlux) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_luminous_flux: Some(other),
            unit_map: LUMINOUS_FLUX_MAP,
            ..Default::default()
        };
        ret.exp[LUMINOUS_FLUX_INDEX] = -1;
        ret
    }
}

impl Mul<UnitLuminousFlux> for Value {
    type Output = Value;
    fn mul(self, other: UnitLuminousFlux) -> Self::Output {
        let mut new: Value = self;
        if self.exp[LUMINOUS_FLUX_INDEX] == 0 {
            new.v_luminous_flux = Some(other);
            new.exp[LUMINOUS_FLUX_INDEX] = 1;
            new.unit_map |= LUMINOUS_FLUX_MAP;
        } else if self.exp[LUMINOUS_FLUX_INDEX] == -1 {
            new.exp[LUMINOUS_FLUX_INDEX] = 0;
            new.v_luminous_flux = None;
            new.unit_map ^= LUMINOUS_FLUX_MAP;
        } else {
            if self.v_luminous_flux != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_luminous_flux.unwrap()
                );
            }
            new.exp[LUMINOUS_FLUX_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitLuminousFlux> for Value {
    type Output = Value;
    fn div(self, other: UnitLuminousFlux) -> Value {
        let mut new: Value = self;
        if self.v_luminous_flux.is_some() && self.v_luminous_flux != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[LUMINOUS_FLUX_INDEX] == 0 {
            new.v_luminous_flux = Some(other);
            new.unit_map |= LUMINOUS_FLUX_MAP;
            new.exp[LUMINOUS_FLUX_INDEX] = -1;
        } else if new.exp[LUMINOUS_FLUX_INDEX] == 1 {
            new.exp[LUMINOUS_FLUX_INDEX] = 0;
            new.v_luminous_flux = None;
            new.unit_map ^= LUMINOUS_FLUX_MAP;
        } else {
            new.exp[LUMINOUS_FLUX_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{LUMINOUS_FLUX_INDEX, LUMINOUS_FLUX_MAP},
        units::{Metric, UnitLuminousFlux, UnitNone},
    };

    const USED_MAP: usize = LUMINOUS_FLUX_MAP;
    const USED_INDEX: usize = LUMINOUS_FLUX_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitLuminousFlux::Lumen(Metric::None);
        assert!(v.is_luminous_flux());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitLuminousFlux::Lumen(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitLuminousFlux::Lumen(Metric::None)
            * UnitLuminousFlux::Lumen(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitLuminousFlux::Lumen(Metric::None)
            / UnitLuminousFlux::Lumen(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitLuminousFlux::Lumen(Metric::None)
            / UnitLuminousFlux::Lumen(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitLuminousFlux::Lumen(Metric::None)
            * UnitLuminousFlux::Lumen(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitLuminousFlux::Lumen(Metric::Kilo) * UnitLuminousFlux::Lumen(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitLuminousFlux::Lumen(Metric::Kilo) / UnitLuminousFlux::Lumen(Metric::None);
    }
}
