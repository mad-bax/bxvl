/// Module used to display and parsing a [`Value`]
pub mod strings;

/// Module used to implement basic operations on a [`Value`]
pub mod value_impl;

/// Module used for arithmetic operations.
mod arithmetic_ops;

/// Module used for constructing a [`Value`].
mod construction;

/// Module used for manipulating a [`Value`].
mod manipulation;

/// Module used for std operations of a [`Value`].
mod value_std_ops;

/// Module used to define various [`Value`] constants
pub mod consts;

use crate::units::{
    UnitAbsorbedDose, UnitAngle, UnitCatalyticActivity, UnitElectricCapacitance,
    UnitElectricCharge, UnitElectricConductance, UnitElectricCurrent, UnitElectricInductance,
    UnitElectricPotential, UnitElectricResistance, UnitEnergy, UnitForce, UnitFrequency,
    UnitIlluminance, UnitInformation, UnitLength, UnitLuminousFlux, UnitLuminousIntensity,
    UnitMagneticFlux, UnitMagneticFluxDensity, UnitMass, UnitPower, UnitPressure,
    UnitRadioactivity, UnitRadioactivityExposure, UnitSolidAngle, UnitSound, UnitSubstance,
    UnitTemperature, UnitTime, UnitVolume,
};
use serde::{Deserialize, Serialize};

/// The [`Value`] struct definition
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
    v_capacitance: Option<UnitElectricCapacitance>,

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
    v_inductance: Option<UnitElectricInductance>,

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
    v_resistance: Option<UnitElectricResistance>,

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

/// Macro to create a new [`Value`]
#[macro_export]
macro_rules! value {
    ($v:expr, $u:expr) => {
        Value::new($v as f64, &$u.to_string()).unwrap()
    };
}

#[cfg(test)]
mod value_serialization_testing {
    use crate::{
        units::{
            Metric, UnitAbsorbedDose, UnitAngle, UnitCatalyticActivity, UnitElectricCapacitance,
            UnitElectricCharge, UnitElectricConductance, UnitElectricCurrent,
            UnitElectricInductance, UnitElectricPotential, UnitElectricResistance, UnitEnergy,
            UnitForce, UnitFrequency, UnitIlluminance, UnitInformation, UnitLength,
            UnitLuminousFlux, UnitLuminousIntensity, UnitMagneticFlux, UnitMagneticFluxDensity,
            UnitMass, UnitNone, UnitPower, UnitPressure, UnitRadioactivity,
            UnitRadioactivityExposure, UnitSolidAngle, UnitSound, UnitSubstance, UnitTemperature,
            UnitTime, UnitVolume,
        },
        value::{Value, consts},
    };
    use toml;

    #[test]
    fn serialize_testing_const() {
        let t = consts::ATOMIC_MASS;

        let s = toml::to_string(&t).unwrap();

        let v: Value = toml::from_str(&s).unwrap();

        assert_eq!(v, t);
    }

    #[test]
    fn serialize_testing() {
        let metrics = vec![
            Metric::Quecto,
            Metric::Ronto,
            Metric::Yocto,
            Metric::Zepto,
            Metric::Atto,
            Metric::Femto,
            Metric::Pico,
            Metric::Nano,
            Metric::Micro,
            Metric::Milli,
            Metric::Centi,
            Metric::Deci,
            Metric::None,
            Metric::Deca,
            Metric::Hecto,
            Metric::Kilo,
            Metric::Mega,
            Metric::Giga,
            Metric::Tera,
            Metric::Peta,
            Metric::Exa,
            Metric::Zetta,
            Metric::Yotta,
            Metric::Ronna,
            Metric::Quetta,
        ];

        for m in metrics {
            let mut vals = vec![
                1.5 * UnitAbsorbedDose::Gray(m),
                1.5 * UnitAngle::Radian(m),
                1.5 * UnitCatalyticActivity::Katal(m),
                1.5 * UnitElectricCapacitance::Farad(m),
                1.5 * UnitElectricCharge::Coulomb(m),
                1.5 * UnitElectricConductance::Siemens(m),
                1.5 * UnitElectricCurrent::Ampere(m),
                1.5 * UnitElectricInductance::Henry(m),
                1.5 * UnitElectricPotential::Volt(m),
                1.5 * UnitElectricResistance::Ohm(m),
                1.5 * UnitEnergy::Joule(m),
                1.5 * UnitForce::Newton(m),
                1.5 * UnitFrequency::Hertz(m),
                1.5 * UnitIlluminance::Lux(m),
                1.5 * UnitLength::Meter(m),
                1.5 * UnitLuminousFlux::Lumen(m),
                1.5 * UnitLuminousIntensity::Candela(m),
                1.5 * UnitMagneticFlux::Weber(m),
                1.5 * UnitMagneticFluxDensity::Tesla(m),
                1.5 * UnitMass::Gram(m),
                1.5 * UnitNone::None,
                1.5 * UnitPower::Watt(m),
                1.5 * UnitPressure::Pascal(m),
                1.5 * UnitRadioactivity::Becquerel(m),
                1.5 * UnitRadioactivityExposure::Sievert(m),
                1.5 * UnitSolidAngle::Steradian(m),
                1.5 * UnitSound::Bel(m),
                1.5 * UnitSubstance::Mole(m),
                1.5 * UnitTemperature::Celsius(m),
                1.5 * UnitTime::Second(m),
                1.5 * UnitVolume::Liter(m),
            ];

            if m >= Metric::None && m != Metric::Deca && m != Metric::Hecto {
                vals.push(1.5 * UnitInformation::Byte(m));
            }

            for t in vals {
                let s = toml::to_string(&t).unwrap();
                let v: Value = toml::from_str(&s).unwrap();
                assert_eq!(v, t);
            }
        }
    }

    #[test]
    #[should_panic]
    fn failed_deserialize() {
        let _: Value = toml::from_str(
            "[v_sound]\nBel = \"Rand\"\n\nval = 1.5\nunit_map = 8192\nexp = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]"
        ).unwrap();
    }
}
