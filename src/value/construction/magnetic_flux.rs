use std::ops::{Div, Mul};

use crate::{
    consts::{MAGNETIC_FLUX_INDEX, MAGNETIC_FLUX_MAP},
    units::UnitMagneticFlux,
    Value,
};

impl Mul<UnitMagneticFlux> for f64 {
    type Output = Value;
    fn mul(self, other: UnitMagneticFlux) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_magnetic_flux: Some(other),
            unit_map: MAGNETIC_FLUX_MAP,
            ..Default::default()
        };
        ret.exp[MAGNETIC_FLUX_INDEX] = 1;
        ret
    }
}

impl Div<UnitMagneticFlux> for f64 {
    type Output = Value;
    fn div(self, other: UnitMagneticFlux) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_magnetic_flux: Some(other),
            unit_map: MAGNETIC_FLUX_MAP,
            ..Default::default()
        };
        ret.exp[MAGNETIC_FLUX_INDEX] = -1;
        ret
    }
}

impl Mul<UnitMagneticFlux> for Value {
    type Output = Value;
    fn mul(self, other: UnitMagneticFlux) -> Self::Output {
        let mut new: Value = self;
        if self.exp[MAGNETIC_FLUX_INDEX] == 0 {
            new.v_magnetic_flux = Some(other);
            new.exp[MAGNETIC_FLUX_INDEX] = 1;
            new.unit_map |= MAGNETIC_FLUX_MAP;
        } else if self.exp[MAGNETIC_FLUX_INDEX] == -1 {
            new.exp[MAGNETIC_FLUX_INDEX] = 0;
            new.v_magnetic_flux = None;
            new.unit_map ^= MAGNETIC_FLUX_MAP;
        } else {
            if self.v_magnetic_flux != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_magnetic_flux.unwrap()
                );
            }
            new.exp[MAGNETIC_FLUX_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitMagneticFlux> for Value {
    type Output = Value;
    fn div(self, other: UnitMagneticFlux) -> Value {
        let mut new: Value = self;
        if self.v_magnetic_flux.is_some() && self.v_magnetic_flux != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[MAGNETIC_FLUX_INDEX] == 0 {
            new.v_magnetic_flux = Some(other);
            new.unit_map |= MAGNETIC_FLUX_MAP;
            new.exp[MAGNETIC_FLUX_INDEX] = -1;
        } else if new.exp[MAGNETIC_FLUX_INDEX] == 1 {
            new.exp[MAGNETIC_FLUX_INDEX] = 0;
            new.v_magnetic_flux = None;
            new.unit_map ^= MAGNETIC_FLUX_MAP;
        } else {
            new.exp[MAGNETIC_FLUX_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{MAGNETIC_FLUX_INDEX, MAGNETIC_FLUX_MAP},
        units::{Metric, UnitMagneticFlux, UnitNone},
    };

    const USED_MAP: usize = MAGNETIC_FLUX_MAP;
    const USED_INDEX: usize = MAGNETIC_FLUX_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitMagneticFlux::Weber(Metric::None);
        assert!(v.is_magnetic_flux());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitMagneticFlux::Weber(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitMagneticFlux::Weber(Metric::None)
            * UnitMagneticFlux::Weber(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitMagneticFlux::Weber(Metric::None)
            / UnitMagneticFlux::Weber(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitMagneticFlux::Weber(Metric::None)
            / UnitMagneticFlux::Weber(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitMagneticFlux::Weber(Metric::None)
            * UnitMagneticFlux::Weber(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1 * UnitMagneticFlux::Weber(Metric::Kilo) * UnitMagneticFlux::Weber(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1 / UnitMagneticFlux::Weber(Metric::Kilo) / UnitMagneticFlux::Weber(Metric::None);
    }
}
