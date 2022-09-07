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
pub const LENGTH_FT_TO_METER:f64    = 0.3048;
pub const LENGTH_IN_TO_METER:f64    = 0.0254;
pub const LENGTH_YD_TO_METER:f64    = 0.9144;
pub const LENGTH_MILE_TO_METER:f64  = 1609.344;
pub const LENGTH_AU_TO_METER:f64    = 149_597_870_700.0;
pub const LENGTH_LYR_TO_METER:f64   = 9_460_730_472_580_800.0;
pub const LENGTH_A_TO_METER:f64     = 0.000_000_000_1;
pub const LENGTH_PC_TO_METER:f64    = (648_000.0/std::f64::consts::PI)*149_597_870_700.0;
pub const MASS_LB_TO_G:f64          = 453.592_37;
pub const MASS_GR_TO_G:f64          = MASS_LB_TO_G/7000.0;
pub const MASS_OZ_TO_G:f64          = MASS_LB_TO_G/16.0;
pub const ANGLE_DEG_TO_RAD:f64      = std::f64::consts::PI/180.0;
pub const ANGLE_MOA_TO_RAD:f64      = std::f64::consts::PI/10800.0;
pub const AB_RAD_TO_GY:f64          = 0.01;
pub const AB_ROE_TO_GY:f64          = 1.0/114.025;
pub const EN_CAL_TO_J:f64           = 4.184;
pub const EN_FTLB_TO_J:f64          = 1.355818;
pub const EN_EV_TO_J:f64            = 1.602176634e-19;
pub const FC_LBF_TO_N:f64           = 4.448_221_615_260_5;
pub const PR_ATM_TO_P:f64           = 101325.0;
pub const PR_BAR_TO_P:f64           = 100000.0;
pub const PR_IN_TO_P:f64            = 3_386.388_666_6;
pub const PR_MM_TO_P:f64            = 133.322_387_415;
pub const PR_PSI_TO_P:f64           = 6894.757;
pub const PR_TORR_TO_P:f64          = 101325.0/760.0;
pub const RADIO_C_TO_BQ:f64         = 37_000_000_000.0;
pub const RADEX_REM_TO_SV:f64       = 0.01;
pub const METER3_TO_LITER:f64       = 1000.0;

// The definition for absolute zero
pub const VAL_ABS_ZERO:f64          = 0.0;

// Avogadro's Number
pub const VAL_AVOGADROS:f64         = 6.02214076e23;

// Faraday's Constant
//pub const VAL_FARADAY:f64           = 96_485.332_123_310_018_4;
pub const VAL_FARADAY:f64           = 96_485.332_123_310_01;

// Atomic Mass Constant
pub const VAL_ATOMIC_MASS:f64       = 1.66053906660e-27;

// R for the Ideal Gas Law
pub const VAL_MOLAR_GAS:f64         = 8.314_462_1;

// Coulombs constant
pub const VAL_COULOMBS:f64          = 8.987_551;

// The speed of light
pub const VAL_LIGHT_SPEED:f64       = 299_792_458.0;

// The boltzmann constant
pub const VAL_BOLTZMANN:f64         = 1.380649e-23;

// Newtonian Constant of Gravitation
pub const VAL_NEWTONIAN_GRAVITATION:f64 = 6.673015e-11;

// The average gravity of earth
pub const VAL_EARTH_GRAV:f64        = 9.806_65;

// The charge of an electron
pub const VAL_ELECTRON_CHARGE:f64   = 1.602176634e-19;

// The Rydberg constant
pub const VAL_RYDBERG:f64           = 10_973_731.568_539;

// Plank's constant
pub const VAL_PLANKS:f64            = 6.62607015e-34;

pub const VAL_VACUUM_ELECTIRC_PERMITTIVITY:f64 = 8.8541878128e-12;

/* These are the unit indexes into the Value unit array. 
 * These allow Values to keep track of what units are present
 * and indexed. 
 */
pub const LENGTH_INDEX:usize                  = 0;
pub const TIME_INDEX:usize                    = 1;
pub const MASS_INDEX:usize                    = 2;
pub const ELECTRIC_CURRENT_INDEX:usize        = 3;
pub const ELECTRIC_CHARGE_INDEX:usize         = 4;
pub const ELECTRIC_POTENTIAL_INDEX:usize      = 5;
pub const ELECTRIC_CONDUCTANCE_INDEX:usize    = 6;
pub const CAPACITANCE_INDEX:usize             = 7;
pub const RESISTANCE_INDEX:usize              = 8;
pub const INDUCTANCE_INDEX:usize              = 9;
pub const MAGNETIC_FLUX_INDEX:usize           = 10;
pub const MAGNETIC_FLUX_DENSITY_INDEX:usize   = 11;
pub const TEMPERATURE_INDEX:usize             = 12;
pub const SUBSTANCE_INDEX:usize               = 13;
pub const LUMINOUS_INTENSITY_INDEX:usize      = 14;
pub const LUMINOUS_FLUX_INDEX:usize           = 15;
pub const ILLUMINANCE_INDEX:usize             = 16;
pub const VOLUME_INDEX:usize                  = 17;
pub const PRESSURE_INDEX:usize                = 18;
pub const ANGLE_INDEX:usize                   = 19;
pub const FREQUENCY_INDEX:usize               = 20;
pub const FORCE_INDEX:usize                   = 21;
pub const ENERGY_INDEX:usize                  = 22;
pub const POWER_INDEX:usize                   = 23;
pub const RADIOACTIVITY_INDEX:usize           = 24;
pub const ABSORBED_DOSE_INDEX:usize           = 25;
pub const RADIOACTIVITY_EXPOSURE_INDEX:usize  = 26;
pub const CATALYTIC_ACTIVITY_INDEX:usize      = 27;
pub const SOUND_INDEX:usize                   = 28;
pub const INFORMATION_INDEX:usize             = 29;
pub const SOLID_ANGLE_INDEX:usize             = 30;

/* Similar to the indexes, these bit maps are for the 
 * Value bit_map field which allows quick determination 
 * of what units are currently in the Value
 */
pub const LENGTH_MAP:usize                  = 1<<LENGTH_INDEX;
pub const TIME_MAP:usize                    = 1<<TIME_INDEX;
pub const MASS_MAP:usize                    = 1<<MASS_INDEX;
pub const ELECTRIC_CURRENT_MAP:usize        = 1<<ELECTRIC_CURRENT_INDEX;
pub const ELECTRIC_CHARGE_MAP:usize         = 1<<ELECTRIC_CHARGE_INDEX;
pub const ELECTRIC_POTENTIAL_MAP:usize      = 1<<ELECTRIC_POTENTIAL_INDEX;
pub const ELECTRIC_CONDUCTANCE_MAP:usize    = 1<<ELECTRIC_CONDUCTANCE_INDEX;
pub const CAPACITANCE_MAP:usize             = 1<<CAPACITANCE_INDEX;
pub const RESISTANCE_MAP:usize              = 1<<RESISTANCE_INDEX;
pub const INDUCTANCE_MAP:usize              = 1<<INDUCTANCE_INDEX;
pub const MAGNETIC_FLUX_MAP:usize           = 1<<MAGNETIC_FLUX_INDEX;
pub const MAGNETIC_FLUX_DENSITY_MAP:usize   = 1<<MAGNETIC_FLUX_DENSITY_INDEX;
pub const TEMPERATURE_MAP:usize             = 1<<TEMPERATURE_INDEX;
pub const SUBSTANCE_MAP:usize               = 1<<SUBSTANCE_INDEX;
pub const LUMINOUS_INTENSITY_MAP:usize      = 1<<LUMINOUS_INTENSITY_INDEX;
pub const LUMINOUS_FLUX_MAP:usize           = 1<<LUMINOUS_FLUX_INDEX;
pub const ILLUMINANCE_MAP:usize             = 1<<ILLUMINANCE_INDEX;
pub const VOLUME_MAP:usize                  = 1<<VOLUME_INDEX;
pub const PRESSURE_MAP:usize                = 1<<PRESSURE_INDEX;
pub const ANGLE_MAP:usize                   = 1<<ANGLE_INDEX;
pub const FREQUENCY_MAP:usize               = 1<<FREQUENCY_INDEX;
pub const FORCE_MAP:usize                   = 1<<FORCE_INDEX;
pub const ENERGY_MAP:usize                  = 1<<ENERGY_INDEX;
pub const POWER_MAP:usize                   = 1<<POWER_INDEX;
pub const RADIOACTIVITY_MAP:usize           = 1<<RADIOACTIVITY_INDEX;
pub const ABSORBED_DOSE_MAP:usize           = 1<<ABSORBED_DOSE_INDEX;
pub const RADIOACTIVITY_EXPOSURE_MAP:usize  = 1<<RADIOACTIVITY_EXPOSURE_INDEX;
pub const CATALYTIC_ACTIVITY_MAP:usize      = 1<<CATALYTIC_ACTIVITY_INDEX;
pub const SOUND_MAP:usize                   = 1<<SOUND_INDEX;
pub const INFORMATION_MAP:usize             = 1<<INFORMATION_INDEX;
pub const SOLID_ANGLE_MAP:usize             = 1<<SOLID_ANGLE_INDEX;