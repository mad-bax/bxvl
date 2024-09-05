use std::ops::{Div, Mul};

use crate::{
    consts::{MAGNETIC_FLUX_DENSITY_INDEX, MAGNETIC_FLUX_DENSITY_MAP},
    units::UnitMagneticFluxDensity,
    value::Value,
};

impl Mul<UnitMagneticFluxDensity> for f64 {
    type Output = Value;
    fn mul(self, other: UnitMagneticFluxDensity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_magnetic_flux_density: Some(other),
            unit_map: MAGNETIC_FLUX_DENSITY_MAP,
            ..Default::default()
        };
        ret.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 1;
        ret
    }
}

impl Div<UnitMagneticFluxDensity> for f64 {
    type Output = Value;
    fn div(self, other: UnitMagneticFluxDensity) -> Self::Output {
        let mut ret = Value {
            val: self,
            v_magnetic_flux_density: Some(other),
            unit_map: MAGNETIC_FLUX_DENSITY_MAP,
            ..Default::default()
        };
        ret.exp[MAGNETIC_FLUX_DENSITY_INDEX] = -1;
        ret
    }
}

impl Mul<UnitMagneticFluxDensity> for Value {
    type Output = Value;
    fn mul(self, other: UnitMagneticFluxDensity) -> Self::Output {
        let mut new: Value = self;
        if self.exp[MAGNETIC_FLUX_DENSITY_INDEX] == 0 {
            new.v_magnetic_flux_density = Some(other);
            new.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 1;
            new.unit_map |= MAGNETIC_FLUX_DENSITY_MAP;
        } else if self.exp[MAGNETIC_FLUX_DENSITY_INDEX] == -1 {
            new.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 0;
            new.v_magnetic_flux_density = None;
            new.unit_map ^= MAGNETIC_FLUX_DENSITY_MAP;
        } else {
            if self.v_magnetic_flux_density != Some(other) {
                panic!(
                    "[mul] Cannot increment unit: {} while unit {} is present",
                    other,
                    self.v_magnetic_flux_density.unwrap()
                );
            }
            new.exp[MAGNETIC_FLUX_DENSITY_INDEX] += 1;
        }
        new
    }
}

impl Div<UnitMagneticFluxDensity> for Value {
    type Output = Value;
    fn div(self, other: UnitMagneticFluxDensity) -> Value {
        let mut new: Value = self;
        if self.v_magnetic_flux_density.is_some() && self.v_magnetic_flux_density != Some(other) {
            panic!("[div] Cannot decrement unit: {} from Value {}", other, self);
        }
        if new.exp[MAGNETIC_FLUX_DENSITY_INDEX] == 0 {
            new.v_magnetic_flux_density = Some(other);
            new.unit_map |= MAGNETIC_FLUX_DENSITY_MAP;
            new.exp[MAGNETIC_FLUX_DENSITY_INDEX] = -1;
        } else if new.exp[MAGNETIC_FLUX_DENSITY_INDEX] == 1 {
            new.exp[MAGNETIC_FLUX_DENSITY_INDEX] = 0;
            new.v_magnetic_flux_density = None;
            new.unit_map ^= MAGNETIC_FLUX_DENSITY_MAP;
        } else {
            new.exp[MAGNETIC_FLUX_DENSITY_INDEX] -= 1;
        }
        new
    }
}

#[cfg(test)]
mod construction_testing {
    use crate::{
        consts::{MAGNETIC_FLUX_DENSITY_INDEX, MAGNETIC_FLUX_DENSITY_MAP},
        units::{Metric, UnitMagneticFluxDensity, UnitNone},
    };

    const USED_MAP: usize = MAGNETIC_FLUX_DENSITY_MAP;
    const USED_INDEX: usize = MAGNETIC_FLUX_DENSITY_INDEX;

    #[test]
    fn mul_f64() {
        let v = 1.1 * UnitMagneticFluxDensity::Tesla(Metric::None);
        assert!(v.is_magnetic_flux_density());
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_f64() {
        let v = 1.1 / UnitMagneticFluxDensity::Tesla(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_val() {
        let v = 1.1
            * UnitNone::None
            * UnitMagneticFluxDensity::Tesla(Metric::None)
            * UnitMagneticFluxDensity::Tesla(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], 2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_val() {
        let v = 1.1 * UnitNone::None
            / UnitMagneticFluxDensity::Tesla(Metric::None)
            / UnitMagneticFluxDensity::Tesla(Metric::None);
        assert_eq!(v.unit_map, USED_MAP);
        assert_eq!(v.exp[USED_INDEX], -2);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn mul_div_val() {
        let v = 1.1 * UnitNone::None * UnitMagneticFluxDensity::Tesla(Metric::None)
            / UnitMagneticFluxDensity::Tesla(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn div_mul_val() {
        let v = 1.1 * UnitNone::None / UnitMagneticFluxDensity::Tesla(Metric::None)
            * UnitMagneticFluxDensity::Tesla(Metric::None);
        assert_eq!(v.unit_map, 0);
        assert_eq!(v.exp[USED_INDEX], 0);
        assert_eq!(v, 1.1);
    }

    #[test]
    #[should_panic]
    fn mul_val_err() {
        let _ = 1.1
            * UnitMagneticFluxDensity::Tesla(Metric::Kilo)
            * UnitMagneticFluxDensity::Tesla(Metric::None);
    }

    #[test]
    #[should_panic]
    fn div_val_err() {
        let _ = 1.1
            / UnitMagneticFluxDensity::Tesla(Metric::Kilo)
            / UnitMagneticFluxDensity::Tesla(Metric::None);
    }
}
