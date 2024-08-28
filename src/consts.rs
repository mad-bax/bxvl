use crate::{
    units::{
        Metric, UnitElectricCapacitance, UnitElectricCharge, UnitEnergy, UnitFrequency, UnitLength,
        UnitMass, UnitSubstance, UnitTemperature, UnitTime,
    },
    Value,
};

/// Length
///
/// The conversion numeric for a foot to a meter
pub(crate) const LENGTH_FT_TO_METER: f64 = 0.3048;
/// The conversion numeric for an inch to a meter
pub(crate) const LENGTH_IN_TO_METER: f64 = 0.0254;
/// The conversion numeric for a yard to a meter
pub(crate) const LENGTH_YD_TO_METER: f64 = 0.9144;
/// The conversion numeric for a mile to a meter
pub(crate) const LENGTH_MILE_TO_METER: f64 = 1609.344;
/// The conversion numeric for a astronomical unit to a meter
pub(crate) const LENGTH_AU_TO_METER: f64 = 149_597_870_700.0;
/// The conversion numeric for a lightyear to a meter
pub(crate) const LENGTH_LYR_TO_METER: f64 = 9_460_730_472_580_800.0;
/// The conversion numeric for an Ångström to a meter
pub(crate) const LENGTH_A_TO_METER: f64 = 0.000_000_000_1;
/// The conversion numeric for a parsec to a meter
pub(crate) const LENGTH_PC_TO_METER: f64 = (648_000.0 / std::f64::consts::PI) * LENGTH_AU_TO_METER;

/// Mass
///
/// The conversion numeric for a pound to a gram
pub(crate) const MASS_LB_TO_G: f64 = 453.592_37;
/// The conversion numeric for a grain to a gram
pub(crate) const MASS_GR_TO_G: f64 = MASS_LB_TO_G / 7000.0;
/// The conversion numeric for an ounce to a gram
pub(crate) const MASS_OZ_TO_G: f64 = MASS_LB_TO_G / 16.0;

/// Angle
///
/// The conversion numeric for a degree to a radian
pub(crate) const ANGLE_DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
/// The conversion numeric for a minute of angle to a radian
pub(crate) const ANGLE_MOA_TO_RAD: f64 = std::f64::consts::PI / 10800.0;

/// Absorbed dose of ionizing radiation
///
/// The conversion numeric for a rad to a Gray
pub(crate) const AB_RAD_TO_GY: f64 = 0.01;
/// The conversion numeric for a Röntgen to a Gray
pub(crate) const AB_ROE_TO_GY: f64 = 1.0 / 114.025;

/// Energy
///
/// The conversion numeric for a calorie to a joule
pub(crate) const EN_CAL_TO_J: f64 = 4.184;
/// The conversion numeric for a footpound to a joule
pub(crate) const EN_FTLB_TO_J: f64 = 1.355818;
/// The conversion numeric for an electron volt to a joule
pub(crate) const EN_EV_TO_J: f64 = 1.602176634e-19;

/// Force
///
/// The conversion numeric for poundforce to a newton
pub(crate) const FC_LBF_TO_N: f64 = 4.448_221_615_260_5;

/// Pressure
///
/// The conversion numeric for atmospheres to pascals
pub(crate) const PR_ATM_TO_P: f64 = 101325.0;
/// The conversion numeric for a bar to pascals
pub(crate) const PR_BAR_TO_P: f64 = 100000.0;
/// The conversion numeric for inHg to pascals
pub(crate) const PR_IN_TO_P: f64 = 3_386.388_666_6;
/// The conversion numeric for mmHg to pascals
/// This is a different value to what Torr is converted from
pub(crate) const PR_MM_TO_P: f64 = 133.322_387_415;
/// The conversion numeric for cmHg to pascals
pub(crate) const PR_CM_TO_P: f64 = 1_333.223_874_15;
/// The conversion numeric for pounds per square inch to pascals
pub(crate) const PR_PSI_TO_P: f64 = 6894.757;
/// The conversion numeric for Torr to pascals
/// This is a different value to what mmHg is converted from
pub(crate) const PR_TORR_TO_P: f64 = 101325.0 / 760.0;

/// Power
///
/// The conversion for Horsepower to Watts
pub(crate) const PW_HPWR_TO_W: f64 = 745.699872;

/// Radioactivity
///
/// The conversion numeric for curies to becquerels
pub(crate) const RADIO_C_TO_BQ: f64 = 37_000_000_000.0;

/// Equivalent dose of ionizing radiation
///
/// The conversion numeric for REM to sievert
pub(crate) const RADEX_REM_TO_SV: f64 = 0.01;

/// Temperature
///
/// The conversion numeric for Kelvin to Celsius
pub(crate) const KELVIN_TO_CELSIUS: f64 = 273.15;

/// Volume
///
/// The conversion numeric for meters cubed to liters
pub(crate) const METER3_TO_LITER: f64 = 1.0e3;

/// The definition for absolute zero
pub(crate) const VAL_ABS_ZERO: f64 = 0.0;

/// Avogadro's Number
pub(crate) const VAL_AVOGADROS: f64 = 6.02214076e23;

/// Faraday's Constant
pub(crate) const VAL_FARADAY: f64 = 96_485.332_123_310_01;

/// Atomic Mass Constant
pub(crate) const VAL_ATOMIC_MASS: f64 = 1.66053906660e-27;

/// R for the Ideal Gas Law
pub(crate) const VAL_MOLAR_GAS: f64 = 8.314_462_1;

/// Coulombs constant
pub(crate) const VAL_COULOMBS: f64 = 8.987_551;

/// The speed of light
pub(crate) const VAL_LIGHT_SPEED: f64 = 299_792_458.0;

/// The boltzmann constant
pub(crate) const VAL_BOLTZMANN: f64 = 1.380649e-23;

/// Newtonian Constant of Gravitation
pub(crate) const VAL_NEWTONIAN_GRAVITATION: f64 = 6.673015e-11;

/// The average gravity of earth
pub(crate) const VAL_EARTH_GRAV: f64 = 9.806_65;

/// The charge of an electron
pub(crate) const VAL_ELECTRON_CHARGE: f64 = 1.602176634e-19;

/// The Rydberg constant
pub(crate) const VAL_RYDBERG: f64 = 10_973_731.568_539;

/// Plank's constant
pub(crate) const VAL_PLANKS: f64 = 6.62607015e-34;

/// The vacuum electric permittivity constant
pub(crate) const VAL_VACUUM_ELECTRIC_PERMITTIVITY: f64 = 8.8541878128e-12;

/* These are the unit indexes into the Value unit array.
 * These allow Values to keep track of what units are present
 * and indexed.
 */
/// Length exponent index
pub(crate) const LENGTH_INDEX: usize = 0;
/// Time exponent index
pub(crate) const TIME_INDEX: usize = 1;
/// Mass exponent index
pub(crate) const MASS_INDEX: usize = 2;
/// Electric current exponent index
pub(crate) const ELECTRIC_CURRENT_INDEX: usize = 3;
/// Electric charge exponent index
pub(crate) const ELECTRIC_CHARGE_INDEX: usize = 4;
/// Electric potential exponent index
pub(crate) const ELECTRIC_POTENTIAL_INDEX: usize = 5;
/// Electric conductance exponent index
pub(crate) const ELECTRIC_CONDUCTANCE_INDEX: usize = 6;
/// Electric capacitance exponent index
pub(crate) const CAPACITANCE_INDEX: usize = 7;
/// Electric resistance exponent index
pub(crate) const RESISTANCE_INDEX: usize = 8;
/// Electric inductance exponent index
pub(crate) const INDUCTANCE_INDEX: usize = 9;
/// Magnetic flux exponent index
pub(crate) const MAGNETIC_FLUX_INDEX: usize = 10;
/// Magnetic flux density exponent index
pub(crate) const MAGNETIC_FLUX_DENSITY_INDEX: usize = 11;
/// Temperature exponent index
pub(crate) const TEMPERATURE_INDEX: usize = 12;
/// Substance exponent index
pub(crate) const SUBSTANCE_INDEX: usize = 13;
/// Luminous intensity exponent index
pub(crate) const LUMINOUS_INTENSITY_INDEX: usize = 14;
/// Luminous flux exponent index
pub(crate) const LUMINOUS_FLUX_INDEX: usize = 15;
/// Illuminance exponent index
pub(crate) const ILLUMINANCE_INDEX: usize = 16;
/// Volume exponent index
pub(crate) const VOLUME_INDEX: usize = 17;
/// Pressure exponent index
pub(crate) const PRESSURE_INDEX: usize = 18;
/// Angle exponent index
pub(crate) const ANGLE_INDEX: usize = 19;
/// Frequency exponent index
pub(crate) const FREQUENCY_INDEX: usize = 20;
/// Force exponent index
pub(crate) const FORCE_INDEX: usize = 21;
/// Energy exponent index
pub(crate) const ENERGY_INDEX: usize = 22;
/// Power exponent index
pub(crate) const POWER_INDEX: usize = 23;
/// Radioactivity exponent index
pub(crate) const RADIOACTIVITY_INDEX: usize = 24;
/// Absorbed dose exponent index
pub(crate) const ABSORBED_DOSE_INDEX: usize = 25;
/// Equivalent dose exponent index
pub(crate) const RADIOACTIVITY_EXPOSURE_INDEX: usize = 26;
/// Catalytic activity exponent index
pub(crate) const CATALYTIC_ACTIVITY_INDEX: usize = 27;
/// Sound exponent index
pub(crate) const SOUND_INDEX: usize = 28;
/// Information exponent index
pub(crate) const INFORMATION_INDEX: usize = 29;
/// Solid angle exponent index
pub(crate) const SOLID_ANGLE_INDEX: usize = 30;

/* Similar to the indexes, these bit maps are for the
 * Value bit_map field which allows quick determination
 * of what units are currently in the Value
 */
/// Length bitmap
pub(crate) const LENGTH_MAP: usize = 1 << LENGTH_INDEX;
/// Time bitmap
pub(crate) const TIME_MAP: usize = 1 << TIME_INDEX;
/// Mass bitmap
pub(crate) const MASS_MAP: usize = 1 << MASS_INDEX;
/// Electric current bitmap
pub(crate) const ELECTRIC_CURRENT_MAP: usize = 1 << ELECTRIC_CURRENT_INDEX;
/// Electric charge bitmap
pub(crate) const ELECTRIC_CHARGE_MAP: usize = 1 << ELECTRIC_CHARGE_INDEX;
/// Electric potential bitmap
pub(crate) const ELECTRIC_POTENTIAL_MAP: usize = 1 << ELECTRIC_POTENTIAL_INDEX;
/// Electric conductance bitmap
pub(crate) const ELECTRIC_CONDUCTANCE_MAP: usize = 1 << ELECTRIC_CONDUCTANCE_INDEX;
/// Electric capacitance bitmap
pub(crate) const CAPACITANCE_MAP: usize = 1 << CAPACITANCE_INDEX;
/// Electric resistance bitmap
pub(crate) const RESISTANCE_MAP: usize = 1 << RESISTANCE_INDEX;
/// Electric inductance bitmap
pub(crate) const INDUCTANCE_MAP: usize = 1 << INDUCTANCE_INDEX;
/// Magnetic flux bitmap
pub(crate) const MAGNETIC_FLUX_MAP: usize = 1 << MAGNETIC_FLUX_INDEX;
/// Magnetic flux density bitmap
pub(crate) const MAGNETIC_FLUX_DENSITY_MAP: usize = 1 << MAGNETIC_FLUX_DENSITY_INDEX;
/// Temperature bitmap
pub(crate) const TEMPERATURE_MAP: usize = 1 << TEMPERATURE_INDEX;
/// Substance bitmap
pub(crate) const SUBSTANCE_MAP: usize = 1 << SUBSTANCE_INDEX;
/// Luminous intensity bitmap
pub(crate) const LUMINOUS_INTENSITY_MAP: usize = 1 << LUMINOUS_INTENSITY_INDEX;
/// Luminous flux bitmap
pub(crate) const LUMINOUS_FLUX_MAP: usize = 1 << LUMINOUS_FLUX_INDEX;
/// Illuminance bitmap
pub(crate) const ILLUMINANCE_MAP: usize = 1 << ILLUMINANCE_INDEX;
/// Volume bitmap
pub(crate) const VOLUME_MAP: usize = 1 << VOLUME_INDEX;
/// Pressure bitmap
pub(crate) const PRESSURE_MAP: usize = 1 << PRESSURE_INDEX;
/// Angle bitmap
pub(crate) const ANGLE_MAP: usize = 1 << ANGLE_INDEX;
/// Frequency bitmap
pub(crate) const FREQUENCY_MAP: usize = 1 << FREQUENCY_INDEX;
/// Force bitmap
pub(crate) const FORCE_MAP: usize = 1 << FORCE_INDEX;
/// Energy bitmap
pub(crate) const ENERGY_MAP: usize = 1 << ENERGY_INDEX;
/// Power bitmap
pub(crate) const POWER_MAP: usize = 1 << POWER_INDEX;
/// Radioactivity bitmap
pub(crate) const RADIOACTIVITY_MAP: usize = 1 << RADIOACTIVITY_INDEX;
/// Absorbed dose bitmap
pub(crate) const ABSORBED_DOSE_MAP: usize = 1 << ABSORBED_DOSE_INDEX;
/// Equivalent dose bitmap
pub(crate) const RADIOACTIVITY_EXPOSURE_MAP: usize = 1 << RADIOACTIVITY_EXPOSURE_INDEX;
/// Catalytic activity bitmap
pub(crate) const CATALYTIC_ACTIVITY_MAP: usize = 1 << CATALYTIC_ACTIVITY_INDEX;
/// Sound bitmap
pub(crate) const SOUND_MAP: usize = 1 << SOUND_INDEX;
/// Information bitmap
pub(crate) const INFORMATION_MAP: usize = 1 << INFORMATION_INDEX;
/// Solid angle bitmap
pub(crate) const SOLID_ANGLE_MAP: usize = 1 << SOLID_ANGLE_INDEX;

/// Defines a `Value` preset to the value of Earth's gravitational acceleration
pub const EARTH_GRAVITY: Value = Value {
    val: VAL_EARTH_GRAV,
    unit_map: LENGTH_MAP | TIME_MAP,
    exp: {
        let mut r = [0; 31];
        r[LENGTH_INDEX] = 1;
        r[TIME_INDEX] = -2;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: None,
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: None,
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: Some(UnitLength::Meter(Metric::None)),
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: None,
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: None,
    v_temperature: None,
    v_time: Some(UnitTime::Second(Metric::None)),
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be absolute zero
pub const ABS_ZERO: Value = Value {
    val: VAL_ABS_ZERO,
    unit_map: TEMPERATURE_MAP,
    exp: {
        let mut r = [0; 31];
        r[TEMPERATURE_INDEX] = 1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: None,
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: None,
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: None,
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: None,
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: None,
    v_temperature: Some(UnitTemperature::Kelvin(Metric::None)),
    v_time: None,
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be Avogadro's Number
pub const AVOGADROS_NUMBER: Value = Value {
    val: VAL_AVOGADROS,
    unit_map: SUBSTANCE_MAP,
    exp: {
        let mut r = [0; 31];
        r[SUBSTANCE_INDEX] = -1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: None,
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: None,
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: None,
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: None,
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: Some(UnitSubstance::Mole(Metric::None)),
    v_temperature: None,
    v_time: None,
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be Faraday's Constant
pub const FARADAY: Value = Value {
    val: VAL_FARADAY,
    unit_map: SUBSTANCE_MAP | ELECTRIC_CHARGE_MAP,
    exp: {
        let mut r = [0; 31];
        r[ELECTRIC_CHARGE_INDEX] = 1;
        r[SUBSTANCE_INDEX] = -1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: Some(UnitElectricCharge::Coulomb(Metric::None)),
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: None,
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: None,
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: None,
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: Some(UnitSubstance::Mole(Metric::None)),
    v_temperature: None,
    v_time: None,
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the Atomic Mass Constant
pub const ATOMIC_MASS: Value = Value {
    val: VAL_ATOMIC_MASS,
    unit_map: MASS_MAP,
    exp: {
        let mut r = [0; 31];
        r[MASS_INDEX] = 1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: None,
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: None,
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: None,
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: Some(UnitMass::Gram(Metric::Kilo)),
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: None,
    v_temperature: None,
    v_time: None,
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the molar gas constant
pub const MOLAR_GAS: Value = Value {
    val: VAL_MOLAR_GAS,
    unit_map: SUBSTANCE_MAP | TEMPERATURE_MAP | ENERGY_MAP,
    exp: {
        let mut r = [0; 31];
        r[ENERGY_INDEX] = 1;
        r[TEMPERATURE_INDEX] = -1;
        r[SUBSTANCE_INDEX] = -1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: None,
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: Some(UnitEnergy::Joule(Metric::None)),
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: None,
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: None,
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: Some(UnitSubstance::Mole(Metric::None)),
    v_temperature: Some(UnitTemperature::Kelvin(Metric::None)),
    v_time: None,
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be Coulomb's Constant
pub const COULOMB: Value = Value {
    val: VAL_COULOMBS,
    unit_map: SUBSTANCE_MAP,
    exp: {
        let mut r = [0; 31];
        r[SUBSTANCE_INDEX] = -1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: None,
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: None,
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: None,
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: None,
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: Some(UnitSubstance::Mole(Metric::None)),
    v_temperature: None,
    v_time: None,
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the speed of light
pub const SPEED_OF_LIGHT: Value = Value {
    val: VAL_LIGHT_SPEED,
    unit_map: TIME_MAP | LENGTH_MAP,
    exp: {
        let mut r = [0; 31];
        r[LENGTH_INDEX] = 1;
        r[TIME_INDEX] = -1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: None,
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: None,
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: Some(UnitLength::Meter(Metric::None)),
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: None,
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: None,
    v_temperature: None,
    v_time: Some(UnitTime::Second(Metric::None)),
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the Boltzmann Constant
pub const BOLTZMANN: Value = Value {
    val: VAL_BOLTZMANN,
    unit_map: ENERGY_MAP | TEMPERATURE_MAP,
    exp: {
        let mut r = [0; 31];
        r[ENERGY_INDEX] = 1;
        r[TEMPERATURE_INDEX] = -1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: None,
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: Some(UnitEnergy::Joule(Metric::None)),
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: None,
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: None,
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: None,
    v_temperature: Some(UnitTemperature::Kelvin(Metric::None)),
    v_time: None,
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the newtonian gravitational constant
pub const NEWTONIAN_GRAVITATION: Value = Value {
    val: VAL_NEWTONIAN_GRAVITATION,
    unit_map: LENGTH_MAP | MASS_MAP | TIME_MAP,
    exp: {
        let mut r = [0; 31];
        r[LENGTH_INDEX] = 3;
        r[TIME_INDEX] = -2;
        r[MASS_INDEX] = -1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: None,
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: None,
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: Some(UnitLength::Meter(Metric::None)),
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: Some(UnitMass::Gram(Metric::Kilo)),
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: None,
    v_temperature: None,
    v_time: Some(UnitTime::Second(Metric::None)),
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the charge of an electron
pub const ELECTRON_CHARGE: Value = Value {
    val: VAL_ELECTRON_CHARGE,
    unit_map: ELECTRIC_CHARGE_MAP,
    exp: {
        let mut r = [0; 31];
        r[ELECTRIC_CHARGE_INDEX] = 1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: Some(UnitElectricCharge::Coulomb(Metric::None)),
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: None,
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: None,
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: None,
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: None,
    v_temperature: None,
    v_time: None,
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the Rydberg Constant
pub const RYDBERG: Value = Value {
    val: VAL_RYDBERG,
    unit_map: LENGTH_MAP,
    exp: {
        let mut r = [0; 31];
        r[LENGTH_INDEX] = -1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: None,
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: None,
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: Some(UnitLength::Meter(Metric::None)),
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: None,
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: None,
    v_temperature: None,
    v_time: None,
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the Plank Constant
pub const PLANK: Value = Value {
    val: VAL_PLANKS,
    unit_map: ENERGY_MAP | FREQUENCY_MAP,
    exp: {
        let mut r = [0; 31];
        r[ENERGY_INDEX] = 1;
        r[FREQUENCY_INDEX] = -1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: None,
    v_catalytic: None,
    v_electric_charge: None,
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: Some(UnitEnergy::Joule(Metric::None)),
    v_force: None,
    v_frequency: Some(UnitFrequency::Hertz(Metric::None)),
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: None,
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: None,
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: None,
    v_temperature: None,
    v_time: None,
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

/// Defines a `Value` preset to be the Vacuum Electric Permittivity Constant
pub const VACUUM_PERMITTIVITY: Value = Value {
    val: VAL_VACUUM_ELECTRIC_PERMITTIVITY,
    unit_map: LENGTH_MAP | CAPACITANCE_MAP,
    exp: {
        let mut r = [0; 31];
        r[CAPACITANCE_INDEX] = 1;
        r[LENGTH_INDEX] = -1;
        r
    },
    v_ab_dose: None,
    v_angle: None,
    v_capacitance: Some(UnitElectricCapacitance::Farad(Metric::None)),
    v_catalytic: None,
    v_electric_charge: None,
    v_electric_conductance: None,
    v_electric_current: None,
    v_electric_potential: None,
    v_energy: None,
    v_force: None,
    v_frequency: None,
    v_illuminance: None,
    v_inductance: None,
    v_information: None,
    v_length: Some(UnitLength::Meter(Metric::None)),
    v_luminous_flux: None,
    v_luminous_flux_intensity: None,
    v_mass: None,
    v_power: None,
    v_pressure: None,
    v_radioactivity: None,
    v_radioactivity_exposure: None,
    v_resistance: None,
    v_sound: None,
    v_substance: None,
    v_temperature: None,
    v_time: None,
    v_volume: None,
    v_magnetic_flux: None,
    v_magnetic_flux_density: None,
    v_solid_angle: None,
};

#[cfg(test)]
mod const_value_testing {

    use crate::{
        consts::{
            CAPACITANCE_INDEX, CAPACITANCE_MAP, ELECTRIC_CHARGE_INDEX, ELECTRIC_CHARGE_MAP,
            ENERGY_INDEX, ENERGY_MAP, FREQUENCY_INDEX, FREQUENCY_MAP, LENGTH_INDEX, LENGTH_MAP,
            MASS_INDEX, MASS_MAP, SUBSTANCE_INDEX, SUBSTANCE_MAP, TEMPERATURE_INDEX,
            TEMPERATURE_MAP, TIME_INDEX, TIME_MAP,
        },
        units::{
            Metric, UnitElectricCapacitance, UnitElectricCharge, UnitEnergy, UnitFrequency,
            UnitLength, UnitMass, UnitSubstance, UnitTemperature, UnitTime,
        },
    };

    use super::{
        ABS_ZERO, ATOMIC_MASS, AVOGADROS_NUMBER, BOLTZMANN, COULOMB, EARTH_GRAVITY,
        ELECTRON_CHARGE, FARADAY, MOLAR_GAS, NEWTONIAN_GRAVITATION, PLANK, RYDBERG, SPEED_OF_LIGHT,
        VACUUM_PERMITTIVITY,
    };

    #[test]
    fn const_abs_zero_test() {
        let v1 = ABS_ZERO;
        assert!(v1.val == 0.0);
        assert_eq!(v1.exp[TEMPERATURE_INDEX], 1);
        assert_eq!(v1.unit_map, TEMPERATURE_MAP);
        assert_eq!(
            v1.v_temperature,
            Some(UnitTemperature::Kelvin(Metric::None))
        );
        assert!(v1.is_temperature());
    }

    #[test]
    fn const_earth_gravity_test() {
        let v1 = EARTH_GRAVITY;
        assert!(v1.val >= 9.80665);
        assert!(v1.val <= 9.80665);
        assert_eq!(v1.exp[LENGTH_INDEX], 1);
        assert_eq!(v1.exp[TIME_INDEX], -2);
        assert_eq!(v1.unit_map, LENGTH_MAP | TIME_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
        assert_eq!(v1.v_time, Some(UnitTime::Second(Metric::None)));
        assert!(v1.is_acceleration());
    }

    #[test]
    fn const_faraday_test() {
        let v1 = FARADAY;
        assert!(v1.val >= 96485.33212331001);
        assert!(v1.val <= 96485.33212331001);
        assert_eq!(v1.exp[SUBSTANCE_INDEX], -1);
        assert_eq!(v1.exp[ELECTRIC_CHARGE_INDEX], 1);
        assert_eq!(v1.unit_map, SUBSTANCE_MAP | ELECTRIC_CHARGE_MAP);
        assert_eq!(v1.v_substance, Some(UnitSubstance::Mole(Metric::None)));
        assert_eq!(
            v1.v_electric_charge,
            Some(UnitElectricCharge::Coulomb(Metric::None))
        );
    }

    #[test]
    fn const_avogadros_number_test() {
        let v1 = AVOGADROS_NUMBER;
        assert!(v1.val >= 6.02214076e23);
        assert!(v1.val <= 6.02214076e23);
        assert_eq!(v1.exp[SUBSTANCE_INDEX], -1);
        assert_eq!(v1.unit_map, SUBSTANCE_MAP);
        assert_eq!(v1.v_substance, Some(UnitSubstance::Mole(Metric::None)));
    }

    #[test]
    fn const_atomic_mass_test() {
        let v1 = ATOMIC_MASS;
        assert!(v1.val >= 1.6605390666e-27);
        assert!(v1.val <= 1.6605390666e-27);
        assert_eq!(v1.exp[MASS_INDEX], 1);
        assert_eq!(v1.unit_map, MASS_MAP);
        assert_eq!(v1.v_mass, Some(UnitMass::Gram(Metric::Kilo)));
    }

    #[test]
    fn const_molar_gas_test() {
        let v1 = MOLAR_GAS;
        assert!(v1.val >= 8.3144621);
        assert!(v1.val <= 8.3144621);
        assert_eq!(v1.exp[SUBSTANCE_INDEX], -1);
        assert_eq!(v1.exp[TEMPERATURE_INDEX], -1);
        assert_eq!(v1.exp[ENERGY_INDEX], 1);
        assert_eq!(v1.unit_map, SUBSTANCE_MAP | TEMPERATURE_MAP | ENERGY_MAP);
        assert_eq!(v1.v_substance, Some(UnitSubstance::Mole(Metric::None)));
        assert_eq!(
            v1.v_temperature,
            Some(UnitTemperature::Kelvin(Metric::None))
        );
        assert_eq!(v1.v_energy, Some(UnitEnergy::Joule(Metric::None)));
    }

    #[test]
    fn const_coulomb_test() {
        let v1 = COULOMB;
        assert!(v1.val >= 8.987551);
        assert!(v1.val <= 8.987551);
        assert_eq!(v1.exp[SUBSTANCE_INDEX], -1);
        assert_eq!(v1.unit_map, SUBSTANCE_MAP);
        assert_eq!(v1.v_substance, Some(UnitSubstance::Mole(Metric::None)));
    }

    #[test]
    fn const_speed_of_light_test() {
        let v1 = SPEED_OF_LIGHT;
        assert!(v1.val >= 299792458.0);
        assert!(v1.val <= 299792458.0);
        assert_eq!(v1.exp[LENGTH_INDEX], 1);
        assert_eq!(v1.exp[TIME_INDEX], -1);
        assert_eq!(v1.unit_map, LENGTH_MAP | TIME_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
        assert_eq!(v1.v_time, Some(UnitTime::Second(Metric::None)));
        assert!(v1.is_velocity());
    }

    #[test]
    fn const_boltzmann_test() {
        let v1 = BOLTZMANN;
        assert!(v1.val >= 1.380649e-23);
        assert!(v1.val <= 1.380649e-23);
        assert_eq!(v1.exp[ENERGY_INDEX], 1);
        assert_eq!(v1.exp[TEMPERATURE_INDEX], -1);
        assert_eq!(v1.unit_map, ENERGY_MAP | TEMPERATURE_MAP);
        assert_eq!(v1.v_energy, Some(UnitEnergy::Joule(Metric::None)));
        assert_eq!(
            v1.v_temperature,
            Some(UnitTemperature::Kelvin(Metric::None))
        );
    }

    #[test]
    fn const_newtonian_gravitation_test() {
        let v1 = NEWTONIAN_GRAVITATION;
        assert!(v1.val >= 6.673015e-11);
        assert!(v1.val <= 6.673015e-11);
        assert_eq!(v1.exp[LENGTH_INDEX], 3);
        assert_eq!(v1.exp[TIME_INDEX], -2);
        assert_eq!(v1.exp[MASS_INDEX], -1);
        assert_eq!(v1.unit_map, LENGTH_MAP | TIME_MAP | MASS_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
        assert_eq!(v1.v_mass, Some(UnitMass::Gram(Metric::Kilo)));
        assert_eq!(v1.v_time, Some(UnitTime::Second(Metric::None)));
    }

    #[test]
    fn const_electron_charge_test() {
        let v1 = ELECTRON_CHARGE;
        assert!(v1.val >= 1.602176634e-19);
        assert!(v1.val <= 1.602176635e-19);
        assert_eq!(v1.exp[ELECTRIC_CHARGE_INDEX], 1);
        assert_eq!(v1.unit_map, ELECTRIC_CHARGE_MAP);
        assert_eq!(
            v1.v_electric_charge,
            Some(UnitElectricCharge::Coulomb(Metric::None))
        );
        assert!(v1.is_electric_charge());
    }

    #[test]
    fn const_rydberg_test() {
        let v1 = RYDBERG;
        assert!(v1.val >= 10973731.568539);
        assert!(v1.val <= 10973731.568540);
        assert_eq!(v1.exp[LENGTH_INDEX], -1);
        assert_eq!(v1.unit_map, LENGTH_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
    }

    #[test]
    fn const_plank_test() {
        let v1 = PLANK;
        assert!(v1.val >= 6.62607015e-34);
        assert!(v1.val <= 6.62607016e-34);
        assert_eq!(v1.exp[ENERGY_INDEX], 1);
        assert_eq!(v1.exp[FREQUENCY_INDEX], -1);
        assert_eq!(v1.unit_map, ENERGY_MAP | FREQUENCY_MAP);
        assert_eq!(v1.v_energy, Some(UnitEnergy::Joule(Metric::None)));
        assert_eq!(v1.v_frequency, Some(UnitFrequency::Hertz(Metric::None)));
    }

    #[test]
    fn const_vacuum_test() {
        let v1 = VACUUM_PERMITTIVITY;
        assert!(v1.val >= 8.8541878128e-12);
        assert!(v1.val <= 8.8541878129e-12);
        assert_eq!(v1.exp[LENGTH_INDEX], -1);
        assert_eq!(v1.exp[CAPACITANCE_INDEX], 1);
        assert_eq!(v1.unit_map, LENGTH_MAP | CAPACITANCE_MAP);
        assert_eq!(v1.v_length, Some(UnitLength::Meter(Metric::None)));
        assert_eq!(
            v1.v_capacitance,
            Some(UnitElectricCapacitance::Farad(Metric::None))
        );
    }
}
