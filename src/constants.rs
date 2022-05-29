/**
 * File    :> constants.rs
 * Author  :> Bax
 * Version :> 0.0.1
 * Details :>
 */

/* This value is used for exponent equivalency
 * When a unit's or values's exponent needs to be compared to another 
 * units exponent, this 'cut off' is where exp are deemed 'equal'. 
 */ 
pub const CUTOFF:f64 = 0.0000001;

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
pub const LENGTH_AU_TO_METER:f64    = 149597870700.0;
pub const LENGTH_LYR_TO_METER:f64   = 9460730472580800.0;
pub const LENGTH_A_TO_METER:f64     = 0.0000000001;
pub const LENGTH_PC_TO_METER:f64    = 96939420213600000.0/std::f64::consts::PI;
pub const MASS_LB_TO_G:f64          = 453.59237;
pub const MASS_GR_TO_G:f64          = MASS_LB_TO_G/7000.0;
pub const MASS_OZ_TO_G:f64          = MASS_LB_TO_G/16.0;
pub const ANGLE_DEG_TO_RAD:f64      = std::f64::consts::PI/180.0;
pub const ANGLE_MOA_TO_RAD:f64      = std::f64::consts::PI/10800.0;
pub const AB_RAD_TO_GY:f64          = 0.01;
pub const AB_ROE_TO_GY:f64          = 1.0/114.025;
pub const EN_CAL_TO_J:f64           = 4.184;
pub const EN_FTLB_TO_J:f64          = 1.355818;
pub const EN_EV_TO_J:f64            = 0.0000000000000000001602176634;
pub const FC_LBF_TO_N:f64           = 4.4482216152605;
pub const PR_ATM_TO_P:f64           = 101325.0;
pub const PR_BAR_TO_P:f64           = 100000.0;
pub const PR_IN_TO_P:f64            = 3386.3886666;
pub const PR_MM_TO_P:f64            = 133.322387415;
pub const PR_PSI_TO_P:f64           = 6894.757;
pub const PR_TORR_TO_P:f64          = 101325.0/760.0;
pub const RADIO_C_TO_BQ:f64         = 37000000000.0;
pub const RADEX_REM_TO_SV:f64       = 0.01;

// The definition for absolute zero
pub const VAL_ABS_ZERO:f64          = 0.0;

// Avogadro's Number
pub const VAL_AVOGADROS:f64         = 602214129000000000000000.0;

// Faraday's Constant
pub const VAL_FARADAY:f64           = 96485.3365;

// Atomic Mass Constant
pub const VAL_ATOMIC_MASS:f64       = 0.000000000000000000000000001660538;

// R for the Ideal Gas Law
pub const VAL_MOLAR_GAS:f64         = 8.3144621;

// Coulombs constant
pub const VAL_COULOMBS:f64          = 8.987551;

// The speed of light
pub const VAL_LIGHT_SPEED:f64       = 299792458.0;

// The boltzmann constant
pub const VAL_BOLTZMANN:f64         = 0.000000000000000000000013806488;

// Newtonian Constant of Gravitation
pub const VAL_BIG_G:f64             = 6.67384;

// The average gravity of earth
pub const VAL_EARTH_GRAV:f64        = 9.80665;

// The charge of an electron
pub const VAL_ELECTRON_CHARGE:f64   = 0.0000000000000000001602176;

// The Rydberg constant
pub const VAL_RYDBERG:f64           = 10973731.568539;

// Plank's constant
pub const VAL_PLANKS:f64 = 0.000000000000000000000000000000000662607004;

// The heat capacity of liquid water
pub const VAL_WATER_HEAT_CAP:f64    = 4.18;

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
pub const MAGNETRIC_FLUX_INDEX:usize          = 10;
pub const MAGNETRIC_FLUX_DENSITY_INDEX:usize  = 11;
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
pub const MAGNETRIC_FLUX_MAP:usize          = 1<<MAGNETRIC_FLUX_INDEX;
pub const MAGNETRIC_FLUX_DENSITY_MAP:usize  = 1<<MAGNETRIC_FLUX_DENSITY_INDEX;
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