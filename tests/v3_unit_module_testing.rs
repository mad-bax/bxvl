extern crate v3;

macro_rules! assert_apr {
    ($x:expr, $y:expr, $d:expr) => {
        if f64::max($x, $y) - f64::min($x, $y) > $d {
            panic!("{:?} {:?}", $x, $y);
        }
    };
    ($x:expr, $y:expr) => {
        if f64::max($x, $y) - f64::min($x, $y) > 0.000001 {
            panic!("{:?} {:?}", $x, $y);
        }
    };
}

#[cfg(test)]
mod unit_creation_tests {
    use v3::{
        units::{
            BaseUnit, Metric, UnitAbsorbedDose, UnitAngle, UnitCatalyticActivity,
            UnitElectricCapacitance, UnitElectricCharge, UnitElectricConductance,
            UnitElectricCurrent, UnitElectricInductance, UnitElectricPotential,
            UnitElectricResistance, UnitEnergy, UnitForce, UnitFrequency, UnitIlluminance,
            UnitInformation, UnitLength, UnitLuminousFlux, UnitLuminousIntensity, UnitMagneticFlux,
            UnitMagneticFluxDensity, UnitMass, UnitPower, UnitPressure, UnitRadioactivity,
            UnitRadioactivityExposure, UnitSolidAngle, UnitSound, UnitSubstance, UnitTemperature,
            UnitTime, UnitVolume,
        },
        values::Value,
    };

    const TEST_METRIC: [(Metric, &str); 22] = [
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
        (Metric::Yocto, "y"),
    ];

    #[test]
    fn clone_testing() {
        let m1: Metric = Metric::default();
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

        let m1 = UnitElectricCapacitance::Farad(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitElectricResistance::Ohm(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitElectricInductance::Henry(Metric::Atto);
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

        let m1 = UnitInformation::Byte(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitTime::Second(Metric::Atto);
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::Atto);

        let m1 = UnitLength::Inch;
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::None);

        let m1 = UnitTime::Day;
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::None);

        let m1 = UnitMass::Grain;
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::None);

        let m1 = UnitTemperature::Fahrenheit;
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::None);

        let m1 = UnitPressure::Psi;
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::None);

        let m1 = UnitAngle::Moa;
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::None);

        let m1 = UnitForce::PoundForce;
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::None);

        let m1 = UnitEnergy::FootPound;
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::None);

        let m1 = UnitRadioactivity::Curie;
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::None);

        let m1 = UnitAbsorbedDose::Roentgen;
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::None);

        let m1 = UnitRadioactivityExposure::Rem;
        let m2 = m1.clone();
        assert!(m2 == m1);
        assert!(m2.get_metric() == Metric::None);
    }

    #[test]
    fn exception_volume_conversions() {
        let mut v1: Value = 2.0
            * UnitLength::Meter(Metric::None)
            * UnitLength::Meter(Metric::None)
            * UnitLength::Meter(Metric::None);
        v1 >>= UnitVolume::Liter(Metric::Milli);
        assert_eq!(v1, 2000000.0);

        let v1: Value = 3.0 * UnitVolume::Liter(Metric::None);
        let v2 = (v1 >> 1.0 * UnitLength::Inch * UnitLength::Inch * UnitLength::Inch).unwrap();
        assert_apr!(v2.val, 183.07123228);

        let mut v1: Value = 2.0 * UnitVolume::Liter(Metric::Kilo);
        v1 >>= UnitLength::Inch;
        assert_apr!(v1.val, 122047.488189);

        let v1: Value = 2.0 * UnitLength::Inch * UnitLength::Inch * UnitLength::Inch;
        let v2 = (v1 >> UnitVolume::Liter(Metric::Milli)).unwrap();
        assert_apr!(v2.val, 32.77412799);
    }

    #[test]
    fn exception_frequency_conversions() {
        let mut v1: Value = 2.0 / UnitTime::Hour;
        v1 >>= UnitFrequency::Hertz(Metric::None);
        assert_apr!(v1.val, 0.000555);

        let v1: Value = 2.0 / UnitTime::Hour;
        let v2 = (v1 >> UnitFrequency::Hertz(Metric::Kilo)).unwrap();
        assert_apr!(v2.val, 0.0000005555555);

        let mut v1: Value = 2.0 * UnitFrequency::Hertz(Metric::Kilo);
        v1 >>= UnitTime::Second(Metric::None);
        assert_apr!(v1.val, 2000.0);

        let v1: Value = 2.0 * UnitFrequency::Hertz(Metric::Kilo);
        let v2 = (v1 >> UnitTime::Second(Metric::None)).unwrap();
        assert_apr!(v2.val, 2000.0);
    }

    #[test]
    fn temperature_conversions() {
        let mut v1: Value = 27.0 * UnitTemperature::Celsius;
        v1 >>= UnitTemperature::Fahrenheit;
        assert_apr!(v1.val, 80.6);

        let mut v1: Value = 27.0 * UnitTemperature::Celsius;
        v1 >>= UnitTemperature::Kelvin(Metric::None);
        assert_apr!(v1.val, 300.15);

        let mut v1: Value = 27.0 * UnitTemperature::Celsius;
        v1 >>= UnitTemperature::Kelvin(Metric::Nano);
        assert_apr!(v1.val, 300149999999.99994);

        let mut v1: Value = 27.0 * UnitTemperature::Celsius;
        v1 >>= UnitTemperature::Celsius;
        assert_apr!(v1.val, 27.0);

        let mut v1: Value = 27.0 * UnitTemperature::Fahrenheit;
        v1 >>= UnitTemperature::Celsius;
        assert_apr!(v1.val, -2.777777);

        let mut v1: Value = 27.0 * UnitTemperature::Fahrenheit;
        v1 >>= UnitTemperature::Kelvin(Metric::None);
        assert_apr!(v1.val, 270.372222);

        let mut v1: Value = 27.0 * UnitTemperature::Fahrenheit;
        v1 >>= UnitTemperature::Kelvin(Metric::Kilo);
        assert_apr!(v1.val, 0.270372222);

        let mut v1: Value = 27.0 * UnitTemperature::Fahrenheit;
        v1 >>= UnitTemperature::Fahrenheit;
        assert_apr!(v1.val, 27.0);

        let mut v1: Value = 335.0 * UnitTemperature::Kelvin(Metric::None);
        v1 >>= UnitTemperature::Celsius;
        assert_apr!(v1.val, 61.85);

        let mut v1: Value = 335.0 * UnitTemperature::Kelvin(Metric::None);
        v1 >>= UnitTemperature::Fahrenheit;
        assert_apr!(v1.val, 143.330000);

        let mut v1: Value = 335.0 * UnitTemperature::Kelvin(Metric::None);
        v1 >>= UnitTemperature::Kelvin(Metric::None);
        assert_apr!(v1.val, 335.0);

        let mut v1: Value = 335.0 * UnitTemperature::Kelvin(Metric::None);
        v1 >>= UnitTemperature::Kelvin(Metric::Micro);
        assert_apr!(v1.val, 335000000.0);

        let mut v1: Value = 335.0 * UnitTemperature::Kelvin(Metric::None);
        v1 >>= UnitTemperature::Kelvin(Metric::Mega);
        assert_apr!(v1.val, 0.000335);
    }

    #[test]
    fn length_conversions() {
        let mut t: Value = 1.5 * UnitLength::Meter(Metric::Yotta);
        for m in TEST_METRIC {
            t >>= UnitLength::Meter(m.0);
        }
        for m in (0..TEST_METRIC.len()).rev() {
            t >>= UnitLength::Meter(TEST_METRIC[m].0);
        }
        assert_apr!(t.val, 1.5);

        let mut imp: Value = 1.0 * UnitLength::Mile;
        imp >>= UnitLength::Foot;
        assert_apr!(imp.val, 5280.0);
        assert_apr!(
            (imp >> UnitLength::Meter(Metric::None)).unwrap().val,
            1609.344
        );

        imp >>= UnitLength::Yard;
        assert_apr!(imp.val, 1760.0);
        assert_apr!(
            (imp >> UnitLength::Meter(Metric::None)).unwrap().val,
            1609.344
        );

        imp >>= UnitLength::Meter(Metric::None);
        assert_apr!(imp.val, 1609.344);

        let temp: Value = 1.0 * UnitLength::Inch;

        assert_apr!(
            (temp >> UnitLength::Meter(Metric::Kilo)).unwrap().val,
            0.0000254
        );
        assert!(
            (temp >> UnitLength::Meter(Metric::Kilo)).unwrap()
                < 1.0 * UnitLength::Meter(Metric::Kilo)
        );
        assert_apr!(
            (imp >> UnitLength::AstronomicalUnit).unwrap().val,
            1.0757872089633223e-08
        );
        assert_apr!(
            (imp >> UnitLength::LightYear(Metric::None)).unwrap().val,
            1.7010779502325107e-13
        );
        assert_apr!(
            (imp >> UnitLength::Angstrom).unwrap().val,
            16093440000000.002
        );
        assert_apr!(
            (imp >> UnitLength::Parsec(Metric::None)).unwrap().val,
            5.2227236183508554e-14
        );
    }

    #[test]
    fn mass_conversions() {
        let mut t: Value = 1.5 * UnitMass::Gram(Metric::Yotta);
        for m in TEST_METRIC {
            t >>= UnitMass::Gram(m.0);
        }
        for m in (0..TEST_METRIC.len()).rev() {
            t >>= UnitMass::Gram(TEST_METRIC[m].0);
        }
        assert_apr!(t.val, 1.5);

        let mut imp: Value = 1.0 * UnitMass::Pound;
        imp >>= UnitMass::Ounce;
        assert_apr!(imp.val, 16.0);
        assert_apr!(
            (imp >> UnitMass::Gram(Metric::Kilo)).unwrap().val,
            0.45359237
        );
        imp >>= UnitMass::Grain;
        assert_apr!(imp.val, 7000.0);
        assert_apr!(
            (imp >> UnitMass::Gram(Metric::Kilo)).unwrap().val,
            0.45359237
        );
    }

    #[test]
    fn time_conversions() {
        let mut t: Value = 1.5 * UnitTime::Second(Metric::Yotta);
        for m in TEST_METRIC {
            t >>= UnitTime::Second(m.0);
        }
        for m in (0..TEST_METRIC.len()).rev() {
            t >>= UnitTime::Second(TEST_METRIC[m].0);
        }
        assert_apr!(t.val, 1.5);

        let mut imp: Value = 1.0 * UnitTime::Day;
        imp >>= UnitTime::Hour;
        assert_apr!(imp.val, 24.0);
        assert_apr!((imp >> UnitTime::Second(Metric::Kilo)).unwrap().val, 86.4);
        imp >>= UnitTime::Minute;
        assert_apr!(imp.val, 1440.0);
        assert_apr!((imp >> UnitTime::Second(Metric::Kilo)).unwrap().val, 86.4);
    }

    #[test]
    fn energy_conversions() {
        let mut t: Value = 1.5 * UnitEnergy::Joule(Metric::Yotta);
        for m in TEST_METRIC {
            t >>= UnitEnergy::Joule(m.0);
        }
        for m in (0..TEST_METRIC.len()).rev() {
            t >>= UnitEnergy::Joule(TEST_METRIC[m].0);
        }
        assert_apr!(t.val, 1.5);

        let mut t: Value = 1.5 * UnitEnergy::GramCalorie(Metric::Yotta);
        for m in TEST_METRIC {
            t >>= UnitEnergy::GramCalorie(m.0);
        }
        for m in (0..TEST_METRIC.len()).rev() {
            t >>= UnitEnergy::GramCalorie(TEST_METRIC[m].0);
        }
        assert_apr!(t.val, 1.5);

        let mut imp: Value = 10000.0 * UnitEnergy::FootPound;
        assert_apr!(
            (imp >> UnitEnergy::Joule(Metric::Kilo)).unwrap().val,
            13.55818
        );
        assert_apr!(
            (imp >> UnitEnergy::GramCalorie(Metric::Kilo)).unwrap().val,
            3.24048278158
        );
        imp >>= UnitEnergy::ElectronVolt(Metric::None);
        assert_apr!(
            (imp >> UnitEnergy::Joule(Metric::Kilo)).unwrap().val,
            13.55818
        );
        assert_apr!(
            (imp >> UnitEnergy::GramCalorie(Metric::Kilo)).unwrap().val,
            3.24048278158
        );
    }

    #[test]
    fn force_conversions() {
        let mut t: Value = 1.5 * UnitForce::Newton(Metric::Yotta);
        for m in TEST_METRIC {
            t >>= UnitForce::Newton(m.0);
        }
        for m in (0..TEST_METRIC.len()).rev() {
            t >>= UnitForce::Newton(TEST_METRIC[m].0);
        }
        assert_apr!(t.val, 1.5);

        let imp: Value = 1.0 * UnitForce::PoundForce;
        assert_apr!(
            (imp >> UnitForce::Newton(Metric::None)).unwrap().val,
            4.4482216
        );
    }

    #[test]
    fn pressure_conversions() {
        let mut t: Value = 1.5 * UnitPressure::Pascal(Metric::Yotta);
        for m in TEST_METRIC {
            t >>= UnitPressure::Pascal(m.0);
        }
        for m in (0..TEST_METRIC.len()).rev() {
            t >>= UnitPressure::Pascal(TEST_METRIC[m].0);
        }
        assert_apr!(t.val, 1.5);

        let mut t: Value = 1.5 * UnitPressure::Bar(Metric::Yotta);
        for m in TEST_METRIC {
            t >>= UnitPressure::Bar(m.0);
        }
        for m in (0..TEST_METRIC.len()).rev() {
            t >>= UnitPressure::Bar(TEST_METRIC[m].0);
        }
        assert_apr!(t.val, 1.5);

        let mut imp: Value = 1.0 * UnitPressure::Atm;
        assert_apr!(
            (imp >> UnitPressure::Bar(Metric::None)).unwrap().val,
            1.01325
        );
        assert_apr!(
            (imp >> UnitPressure::Pascal(Metric::None)).unwrap().val,
            101325.0
        );
        imp >>= UnitPressure::Psi;
        assert_apr!(imp.val, 14.695949);
        assert_apr!(
            (imp >> UnitPressure::Bar(Metric::None)).unwrap().val,
            1.01325
        );
        assert_apr!(
            (imp >> UnitPressure::Pascal(Metric::None)).unwrap().val,
            101325.0
        );
        imp >>= UnitPressure::Hgin;
        assert_apr!(imp.val, 29.92125534);
        assert_apr!(
            (imp >> UnitPressure::Bar(Metric::None)).unwrap().val,
            1.01325
        );
        assert_apr!(
            (imp >> UnitPressure::Pascal(Metric::None)).unwrap().val,
            101325.0
        );
        imp >>= UnitPressure::Torr;
        assert_apr!(imp.val, 760.0); // Yes, they are different
        assert_apr!(
            (imp >> UnitPressure::Bar(Metric::None)).unwrap().val,
            1.01325
        );
        assert_apr!(
            (imp >> UnitPressure::Pascal(Metric::None)).unwrap().val,
            101325.0
        );
        imp >>= UnitPressure::Hgmm;
        assert_apr!(imp.val, 759.9998917); // Yes, they are different
        assert_apr!(
            (imp >> UnitPressure::Bar(Metric::None)).unwrap().val,
            1.01325
        );
        assert_apr!(
            (imp >> UnitPressure::Pascal(Metric::None)).unwrap().val,
            101325.0
        );
    }

    #[test]
    fn radioactivity_conversions() {
        let mut t: Value = 1.5 * UnitRadioactivity::Becquerel(Metric::Yotta);
        for m in TEST_METRIC {
            t >>= UnitRadioactivity::Becquerel(m.0);
        }
        for m in (0..TEST_METRIC.len()).rev() {
            t >>= UnitRadioactivity::Becquerel(TEST_METRIC[m].0);
        }
        assert_apr!(t.val, 1.5);

        let imp: Value = 0.000001 * UnitRadioactivity::Curie;
        assert_apr!(
            (imp >> UnitRadioactivity::Becquerel(Metric::None))
                .unwrap()
                .val,
            37000.0
        );
    }

    #[test]
    fn absorbed_conversions() {
        let mut t: Value = 1.5 * UnitAbsorbedDose::Gray(Metric::Yotta);
        for m in TEST_METRIC {
            t >>= UnitAbsorbedDose::Gray(m.0);
        }
        for m in (0..TEST_METRIC.len()).rev() {
            t >>= UnitAbsorbedDose::Gray(TEST_METRIC[m].0);
        }

        assert_apr!(t.val, 1.5);

        let mut imp: Value = 1.0 * UnitAbsorbedDose::Rad;
        assert_apr!((imp >> UnitAbsorbedDose::Roentgen).unwrap().val, 1.14025);
        assert_apr!(
            (imp >> UnitAbsorbedDose::Gray(Metric::None)).unwrap().val,
            0.01
        );
        imp >>= UnitAbsorbedDose::Roentgen;
        assert_apr!((imp >> UnitAbsorbedDose::Rad).unwrap().val, 1.0);
        assert_apr!(
            (imp >> UnitAbsorbedDose::Gray(Metric::None)).unwrap().val,
            0.01
        );
    }

    #[test]
    fn radioactivity_exposure_conversions() {
        let mut t: Value = 1.5 * UnitRadioactivityExposure::Sievert(Metric::Yotta);
        for m in TEST_METRIC {
            t >>= UnitRadioactivityExposure::Sievert(m.0);
        }
        for m in (0..TEST_METRIC.len()).rev() {
            t >>= UnitRadioactivityExposure::Sievert(TEST_METRIC[m].0);
        }

        assert_apr!(t.val, 1.5);

        let imp: Value = 1.0 * UnitRadioactivityExposure::Rem;
        assert_apr!(
            (imp >> UnitRadioactivityExposure::Sievert(Metric::None))
                .unwrap()
                .val,
            0.01
        );
    }

    #[test]
    fn information_conversions() {
        let info_metric: [(Metric, &str); 8] = [
            (Metric::Yotta, "Y"),
            (Metric::Zetta, "Z"),
            (Metric::Exa, "E"),
            (Metric::Peta, "P"),
            (Metric::Tera, "T"),
            (Metric::Giga, "G"),
            (Metric::Mega, "M"),
            (Metric::Kilo, "k"),
        ];

        let mut t: Value = 1.5 * UnitInformation::Byte(Metric::Yotta);
        for m in info_metric {
            t >>= UnitInformation::Byte(m.0);
        }
        for m in (0..info_metric.len()).rev() {
            t >>= UnitInformation::Byte(info_metric[m].0);
        }

        assert_apr!(t.val, 1.5);

        let mut t: Value = 1.5 * UnitInformation::Bit(Metric::Yotta);
        for m in info_metric {
            t >>= UnitInformation::Bit(m.0);
        }
        for m in (0..info_metric.len()).rev() {
            t >>= UnitInformation::Bit(info_metric[m].0);
        }

        assert_eq!(format!("{}", t), String::from("1.5 Ybits"));
        assert_apr!(t.val, 1.5);
    }
}
