extern crate v3;

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