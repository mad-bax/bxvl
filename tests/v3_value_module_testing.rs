extern crate v3;

macro_rules! assert_apr {
    ($x:expr, $y:expr, $d:expr) => {
        if f64::max($x, $y) - f64::min($x, $y) > $d {panic!("{:?} {:?}", $x, $y);}
    };
    ($x:expr, $y:expr) => {
        if f64::max($x, $y) - f64::min($x, $y) > 0.000001 {panic!("{:?} {:?}", $x, $y);}
    }
}

#[cfg(test)]
mod value_constant_tests {
    use v3::{
        values::value_consts,
        units::{
            Metric,
            UnitMass,
            UnitTemperature,
            UnitSubstance,
            UnitEnergy,
            UnitLength,
            UnitTime,
            UnitElectricCharge,
            UnitFrequency,
            UnitCapacitance,
        }};

    #[test]
    fn value_consts() {
        assert_eq!(value_consts::const_abs_zero(), 0.0 * UnitTemperature::Kelvin);
        assert_eq!(value_consts::const_atomic_mass(), 1.66053906660e-27 * UnitMass::Gram(Metric::Kilo));
        assert_eq!(value_consts::const_avogadros_number(), 6.02214076e23 / UnitSubstance::Mole(Metric::None));
        assert_eq!(value_consts::const_boltzmann(), 1.380649e-23 * UnitEnergy::Joule(Metric::None) / UnitTemperature::Kelvin);
        assert_eq!(value_consts::const_coulomb(), 8.987551/UnitSubstance::Mole(Metric::None));
        assert_eq!(value_consts::const_earth_gravity(), 9.80665*UnitLength::Meter(Metric::None)/UnitTime::Second(Metric::None)/UnitTime::Second(Metric::None));
        assert_eq!(value_consts::const_electron_charge(), 1.602176634e-19*UnitElectricCharge::Coulomb(Metric::None));
        assert_eq!(value_consts::const_faraday(), 96485.33212331001*UnitElectricCharge::Coulomb(Metric::None)/UnitSubstance::Mole(Metric::None));
        assert_eq!(value_consts::const_molar_gas(), 8.3144621*UnitEnergy::Joule(Metric::None)/UnitTemperature::Kelvin/UnitSubstance::Mole(Metric::None));
        assert_eq!(value_consts::const_newtonian_gravitation(), 6.673015e-11*UnitLength::Meter(Metric::None)*UnitLength::Meter(Metric::None)*UnitLength::Meter(Metric::None)/UnitMass::Gram(Metric::Kilo)/UnitTime::Second(Metric::None)/UnitTime::Second(Metric::None));
        assert_eq!(value_consts::const_plank(), 6.62607015e-34*UnitEnergy::Joule(Metric::None)/UnitFrequency::Hertz(Metric::None));
        assert_eq!(value_consts::const_vacuum_permittivity(), 8.8541878128e-12*UnitCapacitance::Farad(Metric::None)/UnitLength::Meter(Metric::None));
        assert_eq!(value_consts::const_rydberg(), 10973731.568539/UnitLength::Meter(Metric::None));
        assert_eq!(value_consts::const_speed_of_light(), 299792458.0*UnitLength::Meter(Metric::None)/UnitTime::Second(Metric::None));
    }
}

#[cfg(test)]
mod value_creation_tests {

    use v3::units::{Metric, UnitLength, UnitTime, UnitTemperature, UnitPressure, UnitAngle, UnitEnergy};

    const V1:f64 = 3.5;
    const V2:f64 = 0.5;

    const TEST_METRIC_UNITS:[&str;34] = ["g", "m", "l", "s", "A", "V", "C", "S", "F", "Ω", "O", "H", "Wb", "T", "mol", "cd", "lm", "lx", "bar", "Pa", "rad", "sr", "Hz", "N", "J", "cal", "W", "Bq", "Gy", "Sv", "kat", "B", "bits", "b"];

    const TEST_IMPERIAL_LENGTH_UNITS:[&str;35] = ["in", "feet", "ft", "inch", "inches", "yards", "yard", "yds", "yd", "mile", "miles", "gr", "grains", "grain", "ounces", "oz", "ounce", "pound", "lb", "pounds", "lbs", "f", "°f", "psi", "inHg", "pounds force", "pound force", "poundsforce", "poundforce", "lbfr", "lbsfr", "footpounds", "foot pounds", "footpound", "foot pound"];

    const TEST_OTHER_LENGTH_UNITS:[(&str, UnitLength);27] = [
        ("AU", UnitLength::AstronomicalUnit),
        ("pc", UnitLength::Parsec),
        ("lightyear", UnitLength::LightYear),
        ("lyr", UnitLength::LightYear),
        ("microns", UnitLength::Meter(Metric::Micro)), 
        ("micron", UnitLength::Meter(Metric::Micro)),
        ("Å", UnitLength::Angstrom)];

    const TEST_OTHER_TIME_UNITS:[(&str, UnitTime);6] = [
        ("minutes", UnitTime::Minute),
        ("min", UnitTIme::Minute),
        ("hours", UnitTime::Hour),
        ("hr", UnitTime::Hour),
        ("day", UnitTime::Day)
        ("days", UnitTime::Day)];
        
    const TEST_OTHER_TEMPERATURE_UNITS:[(&str, UnitTemperature);3] = [
        ("°c", UnitTemperature::Celsius),
        ("c", UnitTemperature::Celsius),
        ("K", UnitTemperature::Kelvin)];
        
    const TEST_OTHER_PRESSURE_UNITS:[(&str, UnitPressure);3] = [
        ("torr", UnitPressure::Torr),
        ("mmHg", UnitPressure::Hgmm),
        ("atm", UnitPressure::Atm)];

    const TEST_OTHER_ANGLE_UNITS:[(&str, UnitAngle);2] = [
        ("°", UnitAngle::Degree),
        ("moa", UnitAngle::Moa)];
        
    const TEST_OTHER_ENERGY_UNITS:[(&str, UnitEnergy);2] = [
        ("Cal", UnitEnergy::GramCalorie(Metric::Kilo)),
        ("eV", UnitEnergy::ElectronVolt)];
        
    "Ci", "R", "rads", "rem"];

    const TEST_METRIC:[(Metric, &str);22] = [
        (Metric::Yotta, "Y"),
        (Metric::Zetta, "Z"),
        (Metric::Exa, "E"),
        (Metric::Peta, "P"),
        (Metric::Tera, "T"),
        (Metric::Giga, "G"),
        (Metric::Mega, "M"),
        (Metric::Kilo, "k"),
        (Metric::Hecto, "h"),
        (Metric::Deca, "da"),
        (Metric::None, ""),
        (Metric::Deci, "d"),
        (Metric::Centi, "c"),
        (Metric::Milli, "m"),
        (Metric::Micro, "μ"), 
        (Metric::Micro, "u"), 
        (Metric::Nano, "n"), 
        (Metric::Pico, "p"),
        (Metric::Femto, "f"),
        (Metric::Atto, "a"),
        (Metric::Zepto, "z"),
        (Metric::Yocto, "y")];

    #[test]
    fn string_creation() {
    }
}

#[cfg(test)]
mod value_conversion_tests {}

#[cfg(test)]
mod value_operation_tests {}

#[cfg(test)]
mod value_display_tests {}