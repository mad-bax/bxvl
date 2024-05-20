/**
 * File    :> constants.rs
 * Author  :> Bax
 * Version :> 0.0.1
 * Details :>
 */

/* These constants are for converting between likewise units.
 * They all convert to the official SI unit in question.
 * All unit relations are based off of SI standard units.
 *
 * Some units like temperature and time are left out as they
 * have special conversion methods that are done programmatically.
 */

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
/// The conversion numeric for pounds per square inch to pascals
pub(crate) const PR_PSI_TO_P: f64 = 6894.757;
/// The conversion numeric for Torr to pascals
/// This is a different value to what mmHg is converted from
pub(crate) const PR_TORR_TO_P: f64 = 101325.0 / 760.0;

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
