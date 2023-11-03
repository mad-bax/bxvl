use crate::units::{
    UnitAbsorbedDose,
    UnitAngle,
    UnitCapacitance,
    UnitCatalyticActivity,
    UnitElectricCharge,
    UnitElectricConductance,
    UnitElectricCurrent,
    UnitElectricPotential,
    UnitEnergy,
    UnitForce,
    UnitFrequency,
    UnitIlluminance,
    UnitInductance,
    UnitInformation,
    UnitLength,
    UnitLuminousFlux,
    UnitLuminousIntensity,
    UnitMass,
    UnitPower,
    UnitPressure,
    UnitRadioactivity,
    UnitRadioactivityExposure,
    UnitResistance,
    UnitSound,
    UnitSubstance,
    UnitTemperature,
    UnitTime,
    UnitVolume,
    UnitMagneticFlux,
    UnitMagneticFluxDensity,
    UnitSolidAngle
};

/// The `Value` struct definition
#[derive(Debug, Clone, Copy)]
pub struct Value<T> {
    /// The numerical value for the `Value` struct
    pub val:f64,
    /// The unit map which specifies which units are present in the `Value`
    pub(crate) unit_map:usize,
    /// The exponent storage of all the units within a `Value`
    pub(crate) exp:[i32;31],
    /// The measurement of this value
    pub(crate) measure:T,
    /// the absorbed dose of ionizing radiation measure
    pub(crate) v_ab_dose :                 Option<UnitAbsorbedDose>,
    /// the angle measure
    pub(crate) v_angle :                   Option<UnitAngle>,
    /// the capacitance measure
    pub(crate) v_capacitance :             Option<UnitCapacitance>,
    /// the catalytic activity measure
    pub(crate) v_catalytic :               Option<UnitCatalyticActivity>,
    /// the electric charge measure
    pub(crate) v_electric_charge :         Option<UnitElectricCharge>,
    /// the electric conductance measure
    pub(crate) v_electric_conductance :    Option<UnitElectricConductance>,
    /// the electric current measure
    pub(crate) v_electric_current :        Option<UnitElectricCurrent>,
    /// the electric potential measure
    pub(crate) v_electric_potential :      Option<UnitElectricPotential>,
    /// the energy measure
    pub(crate) v_energy :                  Option<UnitEnergy>,
    /// the force measure
    pub(crate) v_force :                   Option<UnitForce>,
    /// the frequency measure
    pub(crate) v_frequency :               Option<UnitFrequency>,
    /// the illuminance measure
    pub(crate) v_illuminance :             Option<UnitIlluminance>,
    /// the inductance measure
    pub(crate) v_inductance :              Option<UnitInductance>,
    /// the information measure
    pub(crate) v_information :             Option<UnitInformation>,
    /// the length measure
    pub(crate) v_length :                  Option<UnitLength>,
    /// the luminous flux measure
    pub(crate) v_luminous_flux :           Option<UnitLuminousFlux>,
    /// the luminous intensity measure
    pub(crate) v_luminous_flux_intensity : Option<UnitLuminousIntensity>,
    /// the mass measure
    pub(crate) v_mass :                    Option<UnitMass>,
    /// the power measure
    pub(crate) v_power :                   Option<UnitPower>,
    /// the pressure measure
    pub(crate) v_pressure :                Option<UnitPressure>,
    /// the radioactivity measure
    pub(crate) v_radioactivity :           Option<UnitRadioactivity>,
    /// the equivalent dose measure
    pub(crate) v_radioactivity_exposure :  Option<UnitRadioactivityExposure>,
    /// the resistance measure
    pub(crate) v_resistance :              Option<UnitResistance>,
    /// the sound measure
    pub(crate) v_sound :                   Option<UnitSound>,
    /// the substance measure
    pub(crate) v_substance :               Option<UnitSubstance>,
    /// the temperature measure
    pub(crate) v_temperature :             Option<UnitTemperature>,
    /// the time measure
    pub(crate) v_time :                    Option<UnitTime>,
    /// the volume measure
    pub(crate) v_volume :                  Option<UnitVolume>,
    /// the magnetic flux measure
    pub(crate) v_magnetic_flux :           Option<UnitMagneticFlux>,
    /// The magnetic flux density measure
    pub(crate) v_magnetic_flux_density :   Option<UnitMagneticFluxDensity>,
    /// The solid angle measure
    pub(crate) v_solid_angle :             Option<UnitSolidAngle>
}