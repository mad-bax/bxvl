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
mod unit_creation_tests {
    use v3::{units::{
        Metric::Milli,
        Metric,
        UnitLength,
        UnitTime,
        UnitTemperature,
        UnitPressure,
        UnitAngle,
        UnitEnergy,
        UnitRadioactivity,
        UnitRadioactivityExposure,
        UnitAbsorbedDose,
        UnitMass,
        UnitForce,
        UnitVolume,
        UnitElectricCurrent,
        UnitElectricCharge,
        UnitElectricPotential,
        UnitElectricConductance,
        UnitCapacitance,
        UnitResistance,
        UnitInductance,
        UnitMagneticFlux,
        UnitMagneticFluxDensity,
        UnitSubstance,
        UnitLuminousIntensity,
        UnitLuminousFlux,
        UnitIlluminance,
        UnitSolidAngle,
        UnitFrequency,
        UnitPower,
        UnitCatalyticActivity,
        UnitSound,
        UnitInformation}, values::Value};

    #[test]
    fn clone_testing() {
        let m1:Metric = Metric::default();
        let m2 = m1.clone();
        assert!(m2 == m1);

        let m1 = UnitLength::Meter(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitTime::Second(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitAbsorbedDose::Gray(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitAngle::Radian(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitTemperature::Celsius;
        let m2 = m1.clone();
        assert!(m2 == m1);

        let m1 = UnitPressure::Pascal(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitEnergy::Joule(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitRadioactivity::Becquerel(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitRadioactivityExposure::Sievert(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitMass::Gram(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitForce::Newton(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitVolume::Liter(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitElectricCurrent::Ampere(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitElectricCharge::Coulomb(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitElectricPotential::Volt(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitElectricConductance::Siemens(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitCapacitance::Farad(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitResistance::Ohm(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitInductance::Henry(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitMagneticFlux::Weber(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitMagneticFluxDensity::Tesla(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitSubstance::Mole(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitLuminousIntensity::Candela(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitLuminousFlux::Lumen(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitIlluminance::Lux(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitSolidAngle::Steradian(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitFrequency::Hertz(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitPower::Watt(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitCatalyticActivity::Katal(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitSound::Bel(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitInformation::Bit(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);
    }

    #[test]
    fn liter_cubed_conversion() {
        let mut v1:Value = 2.0 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None);
        v1 >>= UnitVolume::Liter(Milli);
        assert_eq!(v1, 2000000.0);

        let v1:Value = 3.0 * UnitVolume::Liter(Metric::None);
        let v2 = (v1 >> 1.0 * UnitLength::Inch * UnitLength::Inch * UnitLength::Inch).unwrap();
        println!("{:?}", v2);
        assert_apr!(v2.val, 183.07123228);
    }

    #[test]
    fn frequency_conversions() {
        let mut v1:Value = 2.0 / UnitTime::Hour;
        v1 >>= UnitFrequency::Hertz(Metric::None);
        assert_apr!(v1.val, 0.000555);

        let v1:Value = 2.0 / UnitTime::Hour;
        let v2 = (v1 >> UnitFrequency::Hertz(Metric::Kilo)).unwrap();
        assert_apr!(v2.val, 0.0000005555555);

        let mut v1:Value = 2.0 * UnitFrequency::Hertz(Metric::Kilo);
        v1 >>= UnitTime::Second(Metric::None);
        assert_apr!(v1.val, 2000.0);

        let v1:Value = 2.0 * UnitFrequency::Hertz(Metric::None);
        let v2 = (v1 >> UnitTime::Second(Metric::None)).unwrap();
        assert_apr!(v2.val, 2000.0);
    }
}