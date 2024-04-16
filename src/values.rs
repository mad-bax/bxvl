/// The `Value` struct and native `impl` definitions
pub mod value_impl;

/// The `Value` constant constructors
pub mod value_consts;

/// Arithmetic definitions for `Value`s
mod value_arithmetic_ops;

/// Constructor definitions for `Value`s
mod value_construction_ops;

/// Display and from_str definitions for `Values`
mod value_string;

/// Ordering operations (`==`, `!=`, `>=`, ...) definitions for `Value`s
mod value_std_ops;

/// `Value` conversion definitions
mod value_conversion_ops;

use serde::{Deserialize, Serialize};

use crate::units::{
    UnitAbsorbedDose, UnitAngle, UnitCapacitance, UnitCatalyticActivity, UnitElectricCharge,
    UnitElectricConductance, UnitElectricCurrent, UnitElectricPotential, UnitEnergy, UnitForce,
    UnitFrequency, UnitIlluminance, UnitInductance, UnitInformation, length::UnitLength, UnitLuminousFlux,
    UnitLuminousIntensity, UnitMagneticFlux, UnitMagneticFluxDensity, UnitMass, UnitPower,
    UnitPressure, UnitRadioactivity, UnitRadioactivityExposure, UnitResistance, UnitSolidAngle,
    UnitSound, UnitSubstance, UnitTemperature, UnitTime, UnitVolume,
};

/// The `Value` struct definition
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Value {
    /// The numerical value for the `Value` struct
    pub val: f64,
    /// The unit map which specifies which units are present in the `Value`
    unit_map: usize,
    /// The exponent storage of all the units within a `Value`
    exp: [i32; 31],
    /// the absorbed dose of ionizing radiation measure
    v_ab_dose: Option<UnitAbsorbedDose>,
    /// the angle measure
    v_angle: Option<UnitAngle>,
    /// the capacitance measure
    v_capacitance: Option<UnitCapacitance>,
    /// the catalytic activity measure
    v_catalytic: Option<UnitCatalyticActivity>,
    /// the electric charge measure
    v_electric_charge: Option<UnitElectricCharge>,
    /// the electric conductance measure
    v_electric_conductance: Option<UnitElectricConductance>,
    /// the electric current measure
    v_electric_current: Option<UnitElectricCurrent>,
    /// the electric potential measure
    v_electric_potential: Option<UnitElectricPotential>,
    /// the energy measure
    v_energy: Option<UnitEnergy>,
    /// the force measure
    v_force: Option<UnitForce>,
    /// the frequency measure
    v_frequency: Option<UnitFrequency>,
    /// the illuminance measure
    v_illuminance: Option<UnitIlluminance>,
    /// the inductance measure
    v_inductance: Option<UnitInductance>,
    /// the information measure
    v_information: Option<UnitInformation>,
    /// the length measure
    v_length: Option<UnitLength>,
    /// the luminous flux measure
    v_luminous_flux: Option<UnitLuminousFlux>,
    /// the luminous intensity measure
    v_luminous_flux_intensity: Option<UnitLuminousIntensity>,
    /// the mass measure
    v_mass: Option<UnitMass>,
    /// the power measure
    v_power: Option<UnitPower>,
    /// the pressure measure
    v_pressure: Option<UnitPressure>,
    /// the radioactivity measure
    v_radioactivity: Option<UnitRadioactivity>,
    /// the equivalent dose measure
    v_radioactivity_exposure: Option<UnitRadioactivityExposure>,
    /// the resistance measure
    v_resistance: Option<UnitResistance>,
    /// the sound measure
    v_sound: Option<UnitSound>,
    /// the substance measure
    v_substance: Option<UnitSubstance>,
    /// the temperature measure
    v_temperature: Option<UnitTemperature>,
    /// the time measure
    v_time: Option<UnitTime>,
    /// the volume measure
    v_volume: Option<UnitVolume>,
    /// the magnetic flux measure
    v_magnetic_flux: Option<UnitMagneticFlux>,
    /// The magnetic flux density measure
    v_magnetic_flux_density: Option<UnitMagneticFluxDensity>,
    /// The solid angle measure
    v_solid_angle: Option<UnitSolidAngle>,
}

/// Macro to create a new `Value`
#[macro_export]
macro_rules! value {
    ($v:expr, $u:expr) => {
        Value::new($v as f64, &$u.to_string()).unwrap()
    };
}

#[cfg(test)]
mod value_unit_tests {

    use crate::constants::{MASS_INDEX, SUBSTANCE_INDEX};
    use crate::constants::{MASS_MAP, SUBSTANCE_MAP};
    use crate::units::length::UnitLength;
    use crate::units::metric::Metric;
    use crate::units::UnitMass;
    use crate::units::UnitSubstance;
    use crate::value;
    use crate::values::Value;

    #[test]
    fn value_create_1() {
        let v: Value = Value::new(4.5, "kg/mol").unwrap();
        assert_eq!(v.unit_map, MASS_MAP | SUBSTANCE_MAP);
        assert_eq!(v.val, 4.5);
        assert_eq!(v.v_mass, Some(UnitMass::Gram(Metric::Kilo)));
        assert_eq!(v.v_substance, Some(UnitSubstance::Mole(Metric::None)));
        assert_eq!(v.exp[MASS_INDEX], 1);
        assert_eq!(v.exp[SUBSTANCE_INDEX], -1);
    }

    #[test]
    fn value_1() {
        let result: Value = Value::new(3.4, &"kg").unwrap();
        assert_eq!(result.unit_map, MASS_MAP);
        assert_eq!(result.val, 3.4);
    }

    #[test]
    fn value_2() {
        let mut e1: Value = Value::new(5.0, "bytes").unwrap();
        let e2: Value = Value::new(1.0, "Gb").unwrap();
        let expected: Value = Value::new(0.000000004656613, "Gb").unwrap();
        let _ = e1._convert(&e2).unwrap();

        assert!(f64::max(e1.val, expected.val) - f64::min(e1.val, expected.val) <= 0.000001);
    }

    #[test]
    fn value_3() {
        let v1: Value = Value::new(3.4, "kg").unwrap();
        let v2: Value = Value::new(3.4, "kg").unwrap();
        let result: Value = v1 / v2;
        assert_eq!(result.unit_map, 0);
        assert_eq!(result.val, 1.0);
    }

    #[test]
    fn value_4() {
        let v1: Value = Value::new(1.0, "kg^-1").unwrap();
        let v2: Value = Value::new(2.0, "kg").unwrap();
        let result: Value = v1 * v2;
        assert_eq!(result.unit_map, 0);
        assert_eq!(result.val, 2.0);
    }

    #[test]
    fn value_5() {
        let mut v1: Value = Value::new(1.0, "kg").unwrap();
        v1 += 1.0_f64;
        assert_eq!(v1.val, 2.0);
    }

    #[test]
    fn value_6() {
        let mut v1: Value = Value::new(5.0, "g").unwrap();
        v1 -= 3.5;
        assert_eq!(v1.val, 1.5);
    }

    #[test]
    fn value_7() {
        let mut v1: Value = Value::new(7.0, "g").unwrap();
        v1 *= 3.0;
        assert_eq!(v1.val, 21.0);
    }

    #[test]
    fn value_8() {
        let mut v1: Value = Value::new(8.0, "m").unwrap();
        v1 /= 2.0;
        assert_eq!(v1.val, 4.0);
    }

    #[test]
    fn value_9() {
        let v1: Value = value!(5, "ft/s^2");
        let v2: Value = value!(1.524, "m/s^2");
        let v3: Value = value!(5486.4, "m/min^2");
        assert_eq!((v1 >> "m/s^2").unwrap(), v2);
        assert_eq!((v1 >> "m/min^2").unwrap(), v3);
    }

    #[test]
    fn value_10() {
        let v1: Value = 4.5 * UnitLength::Meter(Metric::None);
        let v2: Value = Value::new(4.5, "m").unwrap();
        assert_eq!(v1, v2);

        let v3: Value = 4.5 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None);
        let v4: Value = Value::new(4.5, "m^2").unwrap();
        assert_eq!(v3, v4);
    }

    #[test]
    fn value_11() {
        let v1: Value = Value::new(356.27, "μK").unwrap();
        assert_eq!(v1.to_string(), "356.27 μK");
        assert_eq!(v1.val, 356.27);
    }
}
