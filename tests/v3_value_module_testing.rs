extern crate v3;

#[cfg(test)]
mod value_constant_tests {
    use v3::{
        units::{
            Metric, UnitElectricCapacitance, UnitElectricCharge, UnitEnergy, UnitFrequency,
            UnitLength, UnitMass, UnitSubstance, UnitTemperature, UnitTime,
        },
        values::value_consts,
    };

    #[test]
    fn value_consts() {
        assert_eq!(
            value_consts::const_abs_zero(),
            0.0 * UnitTemperature::Kelvin(Metric::None)
        );
        assert_eq!(
            value_consts::const_atomic_mass(),
            1.66053906660e-27 * UnitMass::Gram(Metric::Kilo)
        );
        assert_eq!(
            value_consts::const_avogadros_number(),
            6.02214076e23 / UnitSubstance::Mole(Metric::None)
        );
        assert_eq!(
            value_consts::const_boltzmann(),
            1.380649e-23 * UnitEnergy::Joule(Metric::None) / UnitTemperature::Kelvin(Metric::None)
        );
        assert_eq!(
            value_consts::const_coulomb(),
            8.987551 / UnitSubstance::Mole(Metric::None)
        );
        assert_eq!(
            value_consts::const_earth_gravity(),
            9.80665 * UnitLength::Meter(Metric::None)
                / UnitTime::Second(Metric::None)
                / UnitTime::Second(Metric::None)
        );
        assert_eq!(
            value_consts::const_electron_charge(),
            1.602176634e-19 * UnitElectricCharge::Coulomb(Metric::None)
        );
        assert_eq!(
            value_consts::const_faraday(),
            96485.33212331001 * UnitElectricCharge::Coulomb(Metric::None)
                / UnitSubstance::Mole(Metric::None)
        );
        assert_eq!(
            value_consts::const_molar_gas(),
            8.3144621 * UnitEnergy::Joule(Metric::None)
                / UnitTemperature::Kelvin(Metric::None)
                / UnitSubstance::Mole(Metric::None)
        );
        assert_eq!(
            value_consts::const_newtonian_gravitation(),
            6.673015e-11
                * UnitLength::Meter(Metric::None)
                * UnitLength::Meter(Metric::None)
                * UnitLength::Meter(Metric::None)
                / UnitMass::Gram(Metric::Kilo)
                / UnitTime::Second(Metric::None)
                / UnitTime::Second(Metric::None)
        );
        assert_eq!(
            value_consts::const_plank(),
            6.62607015e-34 * UnitEnergy::Joule(Metric::None) / UnitFrequency::Hertz(Metric::None)
        );
        assert_eq!(
            value_consts::const_vacuum_permittivity(),
            8.8541878128e-12 * UnitElectricCapacitance::Farad(Metric::None)
                / UnitLength::Meter(Metric::None)
        );
        assert_eq!(
            value_consts::const_rydberg(),
            10973731.568539 / UnitLength::Meter(Metric::None)
        );
        assert_eq!(
            value_consts::const_speed_of_light(),
            299792458.0 * UnitLength::Meter(Metric::None) / UnitTime::Second(Metric::None)
        );
    }
}

#[cfg(test)]
mod value_creation_tests {

    use v3::{
        units::{
            Metric, UnitAbsorbedDose, UnitAngle, UnitCatalyticActivity, UnitElectricCapacitance,
            UnitElectricCharge, UnitElectricConductance, UnitElectricCurrent,
            UnitElectricInductance, UnitElectricPotential, UnitElectricResistance, UnitEnergy,
            UnitForce, UnitFrequency, UnitIlluminance, UnitInformation, UnitLength,
            UnitLuminousFlux, UnitLuminousIntensity, UnitMagneticFlux, UnitMagneticFluxDensity,
            UnitMass, UnitPower, UnitPressure, UnitRadioactivity, UnitRadioactivityExposure,
            UnitSolidAngle, UnitSound, UnitSubstance, UnitTemperature, UnitTime, UnitVolume,
        },
        values::Value,
    };

    const V1: f64 = 3.5;
    const V2: f64 = 0.5;

    const TEST_IMPERIAL_LENGTH_UNITS: [(&str, UnitLength); 11] = [
        ("in", UnitLength::Inch),
        ("feet", UnitLength::Foot),
        ("ft", UnitLength::Foot),
        ("inch", UnitLength::Inch),
        ("inches", UnitLength::Inch),
        ("yards", UnitLength::Yard),
        ("yard", UnitLength::Yard),
        ("yds", UnitLength::Yard),
        ("yd", UnitLength::Yard),
        ("mile", UnitLength::Mile),
        ("miles", UnitLength::Mile),
    ];

    const TEST_IMPERIAL_MASS_UNITS: [(&str, UnitMass); 9] = [
        ("gr", UnitMass::Grain),
        ("grains", UnitMass::Grain),
        ("grain", UnitMass::Grain),
        ("ounces", UnitMass::Ounce),
        ("oz", UnitMass::Ounce),
        ("ounce", UnitMass::Ounce),
        ("lb", UnitMass::Pound),
        ("pounds", UnitMass::Pound),
        ("lbs", UnitMass::Pound),
    ];

    const TEST_IMPERIAL_TEMPERATURE_UNITS: [(&str, UnitTemperature); 2] = [
        ("f", UnitTemperature::Fahrenheit),
        ("°f", UnitTemperature::Fahrenheit),
    ];

    const TEST_IMPERIAL_PRESSURE_UNITS: [(&str, UnitPressure); 2] =
        [("psi", UnitPressure::Psi), ("inHg", UnitPressure::Hgin)];

    const TEST_IMPERIAL_FORCE_UNITS: [(&str, UnitForce); 4] = [
        ("poundsforce", UnitForce::PoundForce),
        ("poundforce", UnitForce::PoundForce),
        ("lbfr", UnitForce::PoundForce),
        ("lbsfr", UnitForce::PoundForce),
    ];

    const TEST_IMPERIAL_ENERGY_UNITS: [(&str, UnitEnergy); 2] = [
        ("footpounds", UnitEnergy::FootPound),
        ("footpound", UnitEnergy::FootPound),
    ];

    const TEST_OTHER_LENGTH_UNITS: [(&str, UnitLength); 7] = [
        ("AU", UnitLength::AstronomicalUnit),
        ("au", UnitLength::AstronomicalUnit),
        ("pc", UnitLength::Parsec(Metric::None)),
        ("lyr", UnitLength::LightYear(Metric::None)),
        ("microns", UnitLength::Meter(Metric::Micro)),
        ("micron", UnitLength::Meter(Metric::Micro)),
        ("Å", UnitLength::Angstrom),
    ];

    const TEST_OTHER_TIME_UNITS: [(&str, UnitTime); 6] = [
        ("minutes", UnitTime::Minute),
        ("min", UnitTime::Minute),
        ("hours", UnitTime::Hour),
        ("hr", UnitTime::Hour),
        ("day", UnitTime::Day),
        ("days", UnitTime::Day),
    ];

    const TEST_OTHER_TEMPERATURE_UNITS: [(&str, UnitTemperature); 3] = [
        ("°c", UnitTemperature::Celsius),
        ("c", UnitTemperature::Celsius),
        ("K", UnitTemperature::Kelvin(Metric::None)),
    ];

    const TEST_OTHER_PRESSURE_UNITS: [(&str, UnitPressure); 3] = [
        ("torr", UnitPressure::Torr),
        ("mmHg", UnitPressure::Hgmm),
        ("atm", UnitPressure::Atm),
    ];

    const TEST_OTHER_ANGLE_UNITS: [(&str, UnitAngle); 2] =
        [("°", UnitAngle::Degree), ("moa", UnitAngle::Moa)];

    const TEST_OTHER_ENERGY_UNITS: [(&str, UnitEnergy); 2] = [
        ("Cal", UnitEnergy::GramCalorie(Metric::Kilo)),
        ("eV", UnitEnergy::ElectronVolt(Metric::None)),
    ];

    const TEST_OTHER_RADIOACTIVITY_UNITS: [(&str, UnitRadioactivity); 1] =
        [("Ci", UnitRadioactivity::Curie)];

    const TEST_OTHER_RAD_EXPOSURE_UNITS: [(&str, UnitRadioactivityExposure); 1] =
        [("Rem", UnitRadioactivityExposure::Rem)];

    const TEST_OTHER_ABSORBED_DOSE_UNITS: [(&str, UnitAbsorbedDose); 2] = [
        ("R", UnitAbsorbedDose::Roentgen),
        ("rads", UnitAbsorbedDose::Rad),
    ];

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

    pub const TEST_METRIC_UNITS: [&str; 40] = [
        "g", "m", "l", "s", "A", "V", "C", "S", "F", "Ω", "O", "H", "Wb", "T", "mol", "cd", "lm",
        "lx", "bar", "Pa", "rad", "sr", "Hz", "N", "J", "cal", "W", "Bq", "Gy", "Sv", "kat", "B",
        "bits", "b", "byte", "bytes", "bit", "lyr", "pc", "eV",
    ];

    #[test]
    fn string_parse() {
        // First test metric

        for m in TEST_METRIC {
            for u in TEST_METRIC_UNITS {
                let t: Value = match u {
                    "g" => V1 * UnitMass::Gram(m.0),
                    "m" => V1 * UnitLength::Meter(m.0),
                    "l" => V1 * UnitVolume::Liter(m.0),
                    "s" => V1 * UnitTime::Second(m.0),
                    "A" => V1 * UnitElectricCurrent::Ampere(m.0),
                    "C" => V1 * UnitElectricCharge::Coulomb(m.0),
                    "V" => V1 * UnitElectricPotential::Volt(m.0),
                    "S" => V1 * UnitElectricConductance::Siemens(m.0),
                    "F" => V1 * UnitElectricCapacitance::Farad(m.0),
                    "Ω" | "O" => V1 * UnitElectricResistance::Ohm(m.0),
                    "H" => V1 * UnitElectricInductance::Henry(m.0),
                    "Wb" => V1 * UnitMagneticFlux::Weber(m.0),
                    "T" => V1 * UnitMagneticFluxDensity::Tesla(m.0),
                    "mol" => V1 * UnitSubstance::Mole(m.0),
                    "cd" => V1 * UnitLuminousIntensity::Candela(m.0),
                    "lm" => V1 * UnitLuminousFlux::Lumen(m.0),
                    "lx" => V1 * UnitIlluminance::Lux(m.0),
                    "bar" => V1 * UnitPressure::Bar(m.0),
                    "Pa" => V1 * UnitPressure::Pascal(m.0),
                    "rad" => V1 * UnitAngle::Radian(m.0),
                    "sr" => V1 * UnitSolidAngle::Steradian(m.0),
                    "Hz" => V1 * UnitFrequency::Hertz(m.0),
                    "N" => V1 * UnitForce::Newton(m.0),
                    "J" => V1 * UnitEnergy::Joule(m.0),
                    "cal" => V1 * UnitEnergy::GramCalorie(m.0),
                    "W" => V1 * UnitPower::Watt(m.0),
                    "Bq" => V1 * UnitRadioactivity::Becquerel(m.0),
                    "Gy" => V1 * UnitAbsorbedDose::Gray(m.0),
                    "Sv" => V1 * UnitRadioactivityExposure::Sievert(m.0),
                    "kat" => V1 * UnitCatalyticActivity::Katal(m.0),
                    "lyr" => V1 * UnitLength::LightYear(m.0),
                    "pc" => V1 * UnitLength::Parsec(m.0),
                    "eV" => V1 * UnitEnergy::ElectronVolt(m.0),

                    "b" => {
                        if m.0 < Metric::None {
                            continue;
                        } else if m.0 == Metric::Hecto {
                            continue;
                        } else if m.0 == Metric::Deca {
                            continue;
                        }
                        V1 * UnitInformation::Byte(m.0)
                    }
                    "B" => V1 * UnitSound::Bel(m.0),
                    "bits" | "bit" => {
                        continue; // These are special
                    }
                    "byte" | "bytes" => {
                        continue; // These are special
                    }
                    e => panic!("Malformed test [{}]!", e),
                };
                let string_val = format!("{} {}{}", V1, m.1, u).parse::<Value>().unwrap();
                assert_eq!(string_val, t);
                assert_eq!(string_val.to_string(), t.to_string());
            }
        }

        for m in TEST_METRIC {
            for u in TEST_METRIC_UNITS {
                let t: Value = match u {
                    "g" => V1 / UnitMass::Gram(m.0),
                    "m" => V1 / UnitLength::Meter(m.0),
                    "l" => V1 / UnitVolume::Liter(m.0),
                    "s" => V1 / UnitTime::Second(m.0),
                    "A" => V1 / UnitElectricCurrent::Ampere(m.0),
                    "C" => V1 / UnitElectricCharge::Coulomb(m.0),
                    "V" => V1 / UnitElectricPotential::Volt(m.0),
                    "S" => V1 / UnitElectricConductance::Siemens(m.0),
                    "F" => V1 / UnitElectricCapacitance::Farad(m.0),
                    "Ω" | "O" => V1 / UnitElectricResistance::Ohm(m.0),
                    "H" => V1 / UnitElectricInductance::Henry(m.0),
                    "Wb" => V1 / UnitMagneticFlux::Weber(m.0),
                    "T" => V1 / UnitMagneticFluxDensity::Tesla(m.0),
                    "mol" => V1 / UnitSubstance::Mole(m.0),
                    "cd" => V1 / UnitLuminousIntensity::Candela(m.0),
                    "lm" => V1 / UnitLuminousFlux::Lumen(m.0),
                    "lx" => V1 / UnitIlluminance::Lux(m.0),
                    "bar" => V1 / UnitPressure::Bar(m.0),
                    "Pa" => V1 / UnitPressure::Pascal(m.0),
                    "rad" => V1 / UnitAngle::Radian(m.0),
                    "sr" => V1 / UnitSolidAngle::Steradian(m.0),
                    "Hz" => V1 / UnitFrequency::Hertz(m.0),
                    "N" => V1 / UnitForce::Newton(m.0),
                    "J" => V1 / UnitEnergy::Joule(m.0),
                    "cal" => V1 / UnitEnergy::GramCalorie(m.0),
                    "W" => V1 / UnitPower::Watt(m.0),
                    "Bq" => V1 / UnitRadioactivity::Becquerel(m.0),
                    "Gy" => V1 / UnitAbsorbedDose::Gray(m.0),
                    "Sv" => V1 / UnitRadioactivityExposure::Sievert(m.0),
                    "kat" => V1 / UnitCatalyticActivity::Katal(m.0),
                    "lyr" => V1 / UnitLength::LightYear(m.0),
                    "pc" => V1 / UnitLength::Parsec(m.0),
                    "eV" => V1 / UnitEnergy::ElectronVolt(m.0),

                    "b" => {
                        if m.0 < Metric::None {
                            continue;
                        } else if m.0 == Metric::Hecto {
                            continue;
                        } else if m.0 == Metric::Deca {
                            continue;
                        }
                        V1 / UnitInformation::Byte(m.0)
                    }
                    "B" => V1 / UnitSound::Bel(m.0),
                    "bits" | "bit" => {
                        continue; // These are special
                    }
                    "byte" | "bytes" => {
                        continue; // These are special
                    }
                    e => panic!("Malformed test [{}]!", e),
                };
                let string_val1 = format!("{} 1/{}{}", V1, m.1, u).parse::<Value>().unwrap();
                let string_val2 = format!("{} {}{}^-1", V1, m.1, u).parse::<Value>().unwrap();
                assert_eq!(string_val1, t);
                assert_eq!(string_val2, t);
                assert_eq!(string_val1.to_string(), t.to_string());
                assert_eq!(string_val2.to_string(), t.to_string());
            }
        }

        for u in TEST_IMPERIAL_ENERGY_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }

        for u in TEST_IMPERIAL_FORCE_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }

        for u in TEST_IMPERIAL_LENGTH_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }

        for u in TEST_IMPERIAL_MASS_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }

        for u in TEST_IMPERIAL_PRESSURE_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }

        for u in TEST_IMPERIAL_TEMPERATURE_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }
        for u in TEST_OTHER_ABSORBED_DOSE_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }
        for u in TEST_OTHER_ANGLE_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }
        for u in TEST_OTHER_ENERGY_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }
        for u in TEST_OTHER_LENGTH_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }
        for u in TEST_OTHER_PRESSURE_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }
        for u in TEST_OTHER_RADIOACTIVITY_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }
        for u in TEST_OTHER_RAD_EXPOSURE_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }
        for u in TEST_OTHER_TEMPERATURE_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }
        for u in TEST_OTHER_TIME_UNITS {
            let string_val = format!("{} {}", V2, u.0).parse::<Value>().unwrap();
            let temp = V2 * u.1;
            assert_eq!(string_val, temp);
            assert_eq!(string_val.to_string(), temp.to_string());
        }

        // Bad parsing
    }

    #[test]
    #[should_panic]
    fn bad_parse() {
        "1.4 sdkf".parse::<Value>().unwrap();
    }

    #[test]
    #[should_panic]
    fn bad_f64_parse() {
        "ghf ml".parse::<Value>().unwrap();
    }

    #[test]
    #[should_panic]
    fn bad_f64_only_parse() {
        "hdfj".parse::<Value>().unwrap();
    }

    #[test]
    fn good_f64_only_parse() {
        "45".parse::<Value>().unwrap();
    }
}

#[cfg(test)]
mod value_operation_tests {
    use crate::value_creation_tests::TEST_METRIC_UNITS;
    use v3::{
        units::{
            Metric, UnitAbsorbedDose, UnitAngle, UnitCatalyticActivity, UnitElectricCapacitance,
            UnitElectricCharge, UnitElectricConductance, UnitElectricCurrent,
            UnitElectricInductance, UnitElectricPotential, UnitElectricResistance, UnitEnergy,
            UnitForce, UnitFrequency, UnitIlluminance, UnitInformation, UnitLength,
            UnitLuminousFlux, UnitLuminousIntensity, UnitMagneticFlux, UnitMagneticFluxDensity,
            UnitMass, UnitPower, UnitPressure, UnitRadioactivity, UnitRadioactivityExposure,
            UnitSolidAngle, UnitSound, UnitSubstance, UnitTime, UnitVolume,
        },
        values::Value,
    };

    #[test]
    fn math_tests_f64() {
        let mut t1: Value = 4.0 * UnitLength::Meter(Metric::None);
        let t2: Value = 4.0 * UnitLength::Meter(Metric::None);

        t1 += 1.0;
        assert_eq!(t1, 5.0);
        t1 -= 1.0;
        assert_eq!(t1, 4.0);
        t1 *= 2.0;
        assert_eq!(t1, 8.0);
        t1 /= 4.0;
        assert_eq!(t1, 2.0);

        assert_eq!(t2 + 1.0, 5.0);
        assert_eq!(t2 - 1.0, 3.0);
        assert_eq!(t2 * 2.0, 8.0);
        assert_eq!(t2 / 2.0, 2.0);
    }

    #[test]
    fn math_tests_f32() {
        let mut t1: Value = 4.0 * UnitLength::Meter(Metric::None);
        let t2: Value = 4.0 * UnitLength::Meter(Metric::None);

        t1 += 1.0_f32;
        assert_eq!(t1, 5.0);
        t1 -= 1.0_f32;
        assert_eq!(t1, 4.0);
        t1 *= 2.0_f32;
        assert_eq!(t1, 8.0);
        t1 /= 4.0_f32;
        assert_eq!(t1, 2.0);

        assert_eq!(t2 + 1.0_f32, 5.0);
        assert_eq!(t2 - 1.0_f32, 3.0);
        assert_eq!(t2 * 2.0_f32, 8.0);
        assert_eq!(t2 / 2.0_f32, 2.0);
    }

    #[test]
    fn math_tests_usize() {
        let mut t1: Value = 4.0 * UnitLength::Meter(Metric::None);
        let t2: Value = 4.0 * UnitLength::Meter(Metric::None);

        t1 += 1_usize;
        assert_eq!(t1, 5.0);
        t1 -= 1_usize;
        assert_eq!(t1, 4.0);
        t1 *= 2_usize;
        assert_eq!(t1, 8.0);
        t1 /= 4_usize;
        assert_eq!(t1, 2.0);

        assert_eq!(t2 + 1_usize, 5.0);
        assert_eq!(t2 - 1_usize, 3.0);
        assert_eq!(t2 * 2_usize, 8.0);
        assert_eq!(t2 / 2_usize, 2.0);
    }

    #[test]
    fn math_tests_isize() {
        let mut t1: Value = 4.0 * UnitLength::Meter(Metric::None);
        let t2: Value = 4.0 * UnitLength::Meter(Metric::None);

        t1 += 1_isize;
        assert_eq!(t1, 5.0);
        t1 -= 1_isize;
        assert_eq!(t1, 4.0);
        t1 *= 2_isize;
        assert_eq!(t1, 8.0);
        t1 /= 4_isize;
        assert_eq!(t1, 2.0);

        assert_eq!(t2 + 1_isize, 5.0);
        assert_eq!(t2 - 1_isize, 3.0);
        assert_eq!(t2 * 2_isize, 8.0);
        assert_eq!(t2 / 2_isize, 2.0);
    }

    #[test]
    fn math_tests_u32() {
        let mut t1: Value = 4.0 * UnitLength::Meter(Metric::None);
        let t2: Value = 4.0 * UnitLength::Meter(Metric::None);

        t1 += 1_u32;
        assert_eq!(t1, 5.0);
        t1 -= 1_u32;
        assert_eq!(t1, 4.0);
        t1 *= 2_u32;
        assert_eq!(t1, 8.0);
        t1 /= 4_u32;
        assert_eq!(t1, 2.0);

        assert_eq!(t2 + 1_u32, 5.0);
        assert_eq!(t2 - 1_u32, 3.0);
        assert_eq!(t2 * 2_u32, 8.0);
        assert_eq!(t2 / 2_u32, 2.0);
    }

    #[test]
    fn math_tests_i32() {
        let mut t1: Value = 4.0 * UnitLength::Meter(Metric::None);
        let t2: Value = 4.0 * UnitLength::Meter(Metric::None);

        t1 += 1_i32;
        assert_eq!(t1, 5.0);
        t1 -= 1_i32;
        assert_eq!(t1, 4.0);
        t1 *= 2_i32;
        assert_eq!(t1, 8.0);
        t1 /= 4_i32;
        assert_eq!(t1, 2.0);

        assert_eq!(t2 + 1_i32, 5.0);
        assert_eq!(t2 - 1_i32, 3.0);
        assert_eq!(t2 * 2_i32, 8.0);
        assert_eq!(t2 / 2_i32, 2.0);
    }

    #[test]
    fn math_tests_u64() {
        let mut t1: Value = 4.0 * UnitLength::Meter(Metric::None);
        let t2: Value = 4.0 * UnitLength::Meter(Metric::None);

        t1 += 1_u64;
        assert_eq!(t1, 5.0);
        t1 -= 1_u64;
        assert_eq!(t1, 4.0);
        t1 *= 2_u64;
        assert_eq!(t1, 8.0);
        t1 /= 4_u64;
        assert_eq!(t1, 2.0);

        assert_eq!(t2 + 1_u64, 5.0);
        assert_eq!(t2 - 1_u64, 3.0);
        assert_eq!(t2 * 2_u64, 8.0);
        assert_eq!(t2 / 2_u64, 2.0);
    }

    #[test]
    fn math_tests_i64() {
        let mut t1: Value = 4.0 * UnitLength::Meter(Metric::None);
        let t2: Value = 4.0 * UnitLength::Meter(Metric::None);

        t1 += 1_i64;
        assert_eq!(t1, 5.0);
        t1 -= 1_i64;
        assert_eq!(t1, 4.0);
        t1 *= 2_i64;
        assert_eq!(t1, 8.0);
        t1 /= 4_i64;
        assert_eq!(t1, 2.0);

        assert_eq!(t2 + 1_i64, 5.0);
        assert_eq!(t2 - 1_i64, 3.0);
        assert_eq!(t2 * 2_i64, 8.0);
        assert_eq!(t2 / 2_i64, 2.0);
    }

    #[test]
    fn math_tests_values() {
        for u2 in TEST_METRIC_UNITS {
            for u1 in TEST_METRIC_UNITS {
                let t1: Value = match u1 {
                    "g" => 4.0 * UnitMass::Gram(Metric::None),
                    "m" => 4.0 * UnitLength::Meter(Metric::None),
                    "l" => 4.0 * UnitVolume::Liter(Metric::None),
                    "s" => 4.0 * UnitTime::Second(Metric::None),
                    "A" => 4.0 * UnitElectricCurrent::Ampere(Metric::None),
                    "C" => 4.0 * UnitElectricCharge::Coulomb(Metric::None),
                    "V" => 4.0 * UnitElectricPotential::Volt(Metric::None),
                    "S" => 4.0 * UnitElectricConductance::Siemens(Metric::None),
                    "F" => 4.0 * UnitElectricCapacitance::Farad(Metric::None),
                    "Ω" => 4.0 * UnitElectricResistance::Ohm(Metric::None),
                    "O" => continue,
                    "H" => 4.0 * UnitElectricInductance::Henry(Metric::None),
                    "Wb" => 4.0 * UnitMagneticFlux::Weber(Metric::None),
                    "T" => 4.0 * UnitMagneticFluxDensity::Tesla(Metric::None),
                    "mol" => 4.0 * UnitSubstance::Mole(Metric::None),
                    "cd" => 4.0 * UnitLuminousIntensity::Candela(Metric::None),
                    "lm" => 4.0 * UnitLuminousFlux::Lumen(Metric::None),
                    "lx" => 4.0 * UnitIlluminance::Lux(Metric::None),
                    "bar" => continue, // skip this for now
                    "Pa" => 4.0 * UnitPressure::Pascal(Metric::None),
                    "rad" => 4.0 * UnitAngle::Radian(Metric::None),
                    "sr" => 4.0 * UnitSolidAngle::Steradian(Metric::None),
                    "Hz" => 4.0 * UnitFrequency::Hertz(Metric::None),
                    "N" => 4.0 * UnitForce::Newton(Metric::None),
                    "J" => 4.0 * UnitEnergy::Joule(Metric::None),
                    "cal" => continue, // skip for now
                    "W" => 4.0 * UnitPower::Watt(Metric::None),
                    "Bq" => 4.0 * UnitRadioactivity::Becquerel(Metric::None),
                    "Gy" => 4.0 * UnitAbsorbedDose::Gray(Metric::None),
                    "Sv" => 4.0 * UnitRadioactivityExposure::Sievert(Metric::None),
                    "kat" => 4.0 * UnitCatalyticActivity::Katal(Metric::None),
                    "lyr" => continue, // skip this for now
                    "pc" => continue,  // skip this for now
                    "eV" => continue,  // skip this for now

                    "b" => {
                        if Metric::None < Metric::None {
                            continue;
                        } else if Metric::None == Metric::Hecto {
                            continue;
                        } else if Metric::None == Metric::Deca {
                            continue;
                        }
                        4.0 * UnitInformation::Byte(Metric::None)
                    }
                    "B" => 4.0 * UnitSound::Bel(Metric::None),
                    "bits" | "bit" => {
                        continue; // These are special
                    }
                    "byte" | "bytes" => {
                        continue; // These are special
                    }
                    e => panic!("Malformed test [{}]!", e),
                };

                let t2: Value = match u2 {
                    "g" => 4.0 * UnitMass::Gram(Metric::Kilo),
                    "m" => 4.0 * UnitLength::Meter(Metric::Kilo),
                    "l" => 4.0 * UnitVolume::Liter(Metric::Kilo),
                    "s" => 4.0 * UnitTime::Second(Metric::Kilo),
                    "A" => 4.0 * UnitElectricCurrent::Ampere(Metric::Kilo),
                    "C" => 4.0 * UnitElectricCharge::Coulomb(Metric::Kilo),
                    "V" => 4.0 * UnitElectricPotential::Volt(Metric::Kilo),
                    "S" => 4.0 * UnitElectricConductance::Siemens(Metric::Kilo),
                    "F" => 4.0 * UnitElectricCapacitance::Farad(Metric::Kilo),
                    "Ω" => 4.0 * UnitElectricResistance::Ohm(Metric::Kilo),
                    "O" if !t1.is_resistance() => 4.0 * UnitElectricResistance::Ohm(Metric::Kilo),
                    "H" => 4.0 * UnitElectricInductance::Henry(Metric::Kilo),
                    "Wb" => 4.0 * UnitMagneticFlux::Weber(Metric::Kilo),
                    "T" => 4.0 * UnitMagneticFluxDensity::Tesla(Metric::Kilo),
                    "mol" => 4.0 * UnitSubstance::Mole(Metric::Kilo),
                    "cd" => 4.0 * UnitLuminousIntensity::Candela(Metric::Kilo),
                    "lm" => 4.0 * UnitLuminousFlux::Lumen(Metric::Kilo),
                    "lx" => 4.0 * UnitIlluminance::Lux(Metric::Kilo),
                    "bar" => continue, // skip this for now
                    "Pa" => 4.0 * UnitPressure::Pascal(Metric::Kilo),
                    "rad" => 4.0 * UnitAngle::Radian(Metric::Kilo),
                    "sr" => 4.0 * UnitSolidAngle::Steradian(Metric::Kilo),
                    "Hz" => 4.0 * UnitFrequency::Hertz(Metric::Kilo),
                    "N" => 4.0 * UnitForce::Newton(Metric::Kilo),
                    "J" => 4.0 * UnitEnergy::Joule(Metric::Kilo),
                    "cal" if !t1.is_energy() => 4.0 * UnitEnergy::GramCalorie(Metric::Kilo),
                    "W" => 4.0 * UnitPower::Watt(Metric::Kilo),
                    "Bq" => 4.0 * UnitRadioactivity::Becquerel(Metric::Kilo),
                    "Gy" => 4.0 * UnitAbsorbedDose::Gray(Metric::Kilo),
                    "Sv" if !t1.is_equivalent_dose() => {
                        4.0 * UnitRadioactivityExposure::Sievert(Metric::Kilo)
                    }
                    "kat" => 4.0 * UnitCatalyticActivity::Katal(Metric::Kilo),
                    "lyr" => continue, // skip this for now
                    "pc" => continue,  // skip this for now
                    "eV" => continue,  // skip this for now

                    "b" if !t1.is_information() => {
                        if Metric::None < Metric::None {
                            continue;
                        } else if Metric::None == Metric::Hecto {
                            continue;
                        } else if Metric::None == Metric::Deca {
                            continue;
                        }
                        4.0 * UnitInformation::Byte(Metric::None)
                    }
                    "B" => 4.0 * UnitSound::Bel(Metric::Kilo),
                    "bits" | "bit" => {
                        continue; // These are special
                    }
                    "byte" | "bytes" => {
                        continue; // These are special
                    }
                    _ => continue,
                };

                println!("t1 : {}\nt2 : {}\n", t1, t2);
                if u1 == u2 {
                    assert_eq!(t1 * t2, 16000.0);
                    assert_eq!(t1 / t2, 0.001);
                    assert_eq!(t1 + t2, 4004.0);
                    assert_eq!(t1 - t2, -3996.0);
                } else {
                    assert_eq!(t1 * t2, 16.0);
                    assert_eq!(t1 / t2, 1.0);
                }
            }
        }

        for u2 in TEST_METRIC_UNITS {
            for u1 in TEST_METRIC_UNITS {
                let mut t1: Value = match u1 {
                    "g" => 4.0 * UnitMass::Gram(Metric::None),
                    "m" => 4.0 * UnitLength::Meter(Metric::None),
                    "l" => 4.0 * UnitVolume::Liter(Metric::None),
                    "s" => 4.0 * UnitTime::Second(Metric::None),
                    "A" => 4.0 * UnitElectricCurrent::Ampere(Metric::None),
                    "C" => 4.0 * UnitElectricCharge::Coulomb(Metric::None),
                    "V" => 4.0 * UnitElectricPotential::Volt(Metric::None),
                    "S" => 4.0 * UnitElectricConductance::Siemens(Metric::None),
                    "F" => 4.0 * UnitElectricCapacitance::Farad(Metric::None),
                    "Ω" => 4.0 * UnitElectricResistance::Ohm(Metric::None),
                    "O" => continue,
                    "H" => 4.0 * UnitElectricInductance::Henry(Metric::None),
                    "Wb" => 4.0 * UnitMagneticFlux::Weber(Metric::None),
                    "T" => 4.0 * UnitMagneticFluxDensity::Tesla(Metric::None),
                    "mol" => 4.0 * UnitSubstance::Mole(Metric::None),
                    "cd" => 4.0 * UnitLuminousIntensity::Candela(Metric::None),
                    "lm" => 4.0 * UnitLuminousFlux::Lumen(Metric::None),
                    "lx" => 4.0 * UnitIlluminance::Lux(Metric::None),
                    "bar" => continue, // skip this for now
                    "Pa" => 4.0 * UnitPressure::Pascal(Metric::None),
                    "rad" => 4.0 * UnitAngle::Radian(Metric::None),
                    "sr" => 4.0 * UnitSolidAngle::Steradian(Metric::None),
                    "Hz" => 4.0 * UnitFrequency::Hertz(Metric::None),
                    "N" => 4.0 * UnitForce::Newton(Metric::None),
                    "J" => 4.0 * UnitEnergy::Joule(Metric::None),
                    "cal" => continue, // skip for now
                    "W" => 4.0 * UnitPower::Watt(Metric::None),
                    "Bq" => 4.0 * UnitRadioactivity::Becquerel(Metric::None),
                    "Gy" => 4.0 * UnitAbsorbedDose::Gray(Metric::None),
                    "Sv" => 4.0 * UnitRadioactivityExposure::Sievert(Metric::None),
                    "kat" => 4.0 * UnitCatalyticActivity::Katal(Metric::None),
                    "lyr" => continue, // skip this for now
                    "pc" => continue,  // skip this for now
                    "eV" => continue,  // skip this for now

                    "b" => {
                        if Metric::None < Metric::None {
                            continue;
                        } else if Metric::None == Metric::Hecto {
                            continue;
                        } else if Metric::None == Metric::Deca {
                            continue;
                        }
                        4.0 * UnitInformation::Byte(Metric::None)
                    }
                    "B" => 4.0 * UnitSound::Bel(Metric::None),
                    "bits" | "bit" => {
                        continue; // These are special
                    }
                    "byte" | "bytes" => {
                        continue; // These are special
                    }
                    e => panic!("Malformed test [{}]!", e),
                };

                let t2: Value = match u2 {
                    "g" => 4.0 * UnitMass::Gram(Metric::Kilo),
                    "m" => 4.0 * UnitLength::Meter(Metric::Kilo),
                    "l" => 4.0 * UnitVolume::Liter(Metric::Kilo),
                    "s" => 4.0 * UnitTime::Second(Metric::Kilo),
                    "A" => 4.0 * UnitElectricCurrent::Ampere(Metric::Kilo),
                    "C" => 4.0 * UnitElectricCharge::Coulomb(Metric::Kilo),
                    "V" => 4.0 * UnitElectricPotential::Volt(Metric::Kilo),
                    "S" => 4.0 * UnitElectricConductance::Siemens(Metric::Kilo),
                    "F" => 4.0 * UnitElectricCapacitance::Farad(Metric::Kilo),
                    "Ω" => 4.0 * UnitElectricResistance::Ohm(Metric::Kilo),
                    "O" if !t1.is_resistance() => 4.0 * UnitElectricResistance::Ohm(Metric::Kilo),
                    "H" => 4.0 * UnitElectricInductance::Henry(Metric::Kilo),
                    "Wb" => 4.0 * UnitMagneticFlux::Weber(Metric::Kilo),
                    "T" => 4.0 * UnitMagneticFluxDensity::Tesla(Metric::Kilo),
                    "mol" => 4.0 * UnitSubstance::Mole(Metric::Kilo),
                    "cd" => 4.0 * UnitLuminousIntensity::Candela(Metric::Kilo),
                    "lm" => 4.0 * UnitLuminousFlux::Lumen(Metric::Kilo),
                    "lx" => 4.0 * UnitIlluminance::Lux(Metric::Kilo),
                    "bar" => continue, // skip this for now
                    "Pa" => 4.0 * UnitPressure::Pascal(Metric::Kilo),
                    "rad" => 4.0 * UnitAngle::Radian(Metric::Kilo),
                    "sr" => 4.0 * UnitSolidAngle::Steradian(Metric::Kilo),
                    "Hz" => 4.0 * UnitFrequency::Hertz(Metric::Kilo),
                    "N" => 4.0 * UnitForce::Newton(Metric::Kilo),
                    "J" => 4.0 * UnitEnergy::Joule(Metric::Kilo),
                    "cal" if !t1.is_energy() => 4.0 * UnitEnergy::GramCalorie(Metric::Kilo),
                    "W" => 4.0 * UnitPower::Watt(Metric::Kilo),
                    "Bq" => 4.0 * UnitRadioactivity::Becquerel(Metric::Kilo),
                    "Gy" => 4.0 * UnitAbsorbedDose::Gray(Metric::Kilo),
                    "Sv" if !t1.is_equivalent_dose() => {
                        4.0 * UnitRadioactivityExposure::Sievert(Metric::Kilo)
                    }
                    "kat" => 4.0 * UnitCatalyticActivity::Katal(Metric::Kilo),
                    "lyr" => continue, // skip this for now
                    "pc" => continue,  // skip this for now
                    "eV" => continue,  // skip this for now

                    "b" if !t1.is_information() => {
                        if Metric::None < Metric::None {
                            continue;
                        } else if Metric::None == Metric::Hecto {
                            continue;
                        } else if Metric::None == Metric::Deca {
                            continue;
                        }
                        4.0 * UnitInformation::Byte(Metric::None)
                    }
                    "B" => 4.0 * UnitSound::Bel(Metric::Kilo),
                    "bits" | "bit" => {
                        continue; // These are special
                    }
                    "byte" | "bytes" => {
                        continue; // These are special
                    }
                    _ => continue,
                };

                println!("t1 : {}\nt2 : {}\n", t1, t2);
                if u1 == u2 {
                    t1 *= t2;
                    assert_eq!(t1, 16000.0);
                    t1 /= t2;
                    assert_eq!(t1, 4000.0);
                    t1 += t2;
                    assert_eq!(t1, 4004.0);
                    t1 -= t2;
                    assert_eq!(t1, 4000.0);
                    t1 /= t2;
                    assert_eq!(t1, 1000.0);
                } else {
                    t1 *= t2;
                    assert_eq!(t1, 16.0);
                    t1 /= t2;
                    assert_eq!(t1, 4.0);
                    t1 /= t2;
                    assert_eq!(t1, 1.0);
                }
            }
        }

        let mut t1: Value = 4.0 * UnitLength::Meter(Metric::None);
        let t2: Value = 2.0 * UnitLength::Meter(Metric::None);

        t1 += t2;
        assert_eq!(t1, 6.0 * UnitLength::Meter(Metric::None));
        t1 -= t2;
        assert_eq!(t1, 4.0 * UnitLength::Meter(Metric::None));
        t1 *= t2;
        assert_eq!(
            t1,
            8.0 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None)
        );
        t1 /= t2;
        assert_eq!(t1, 4.0 * UnitLength::Meter(Metric::None));

        assert_eq!(t2 + t1, 6.0 * UnitLength::Meter(Metric::None));
        assert_eq!(t2 - t1, -2.0 * UnitLength::Meter(Metric::None));
        assert_eq!(
            t2 * t1,
            8.0 * UnitLength::Meter(Metric::None) * UnitLength::Meter(Metric::None)
        );
        assert_eq!(t2 / t1, 0.5);
    }
}

#[cfg(test)]
mod value_display_tests {
    use v3::units::{Metric, UnitLength, UnitTime};
    use v3::values::Value;

    #[test]
    fn value_debug() {
        let t: Value = 1.5 * UnitLength::Mile;
        assert!(format!("{:?}", t).chars().count() > format!("{}", t).chars().count());
    }

    #[test]
    fn value_clone() {
        let t_old: Value = 1.5 * UnitLength::Mile;
        let t_new = t_old.clone();

        assert_eq!(t_new, t_old);
    }

    #[test]
    fn empty_value_display() {
        let t = Value::default();
        assert_eq!(t.to_string(), "0 ");
    }

    #[test]
    fn value_num_denom_display() {
        let t: Value = 1.5 * UnitLength::Foot / UnitTime::Second(Metric::None);
        assert_eq!(t.to_string(), "1.5 ft/s");
        assert!(t == "ft/s");
    }

    #[test]
    #[should_panic]
    fn value_false_eq() {
        let t: Value = 1.5 * UnitLength::Foot / UnitTime::Second(Metric::None);
        assert!(t == "slkjf");
    }

    #[test]
    fn value_exponents() {
        let t: Value = 1.5 * UnitLength::Foot * UnitLength::Foot
            / UnitTime::Second(Metric::None)
            / UnitTime::Second(Metric::None);
        assert_eq!(t.to_string(), "1.5 ft^2/s^2");
    }
}

#[cfg(test)]
mod value_edge_cases {
    use v3::units::{Metric, UnitLength};
    use v3::values::Value;

    #[test]
    fn double_divisor() {
        let t: Value = "1.5 1/m^-1".parse::<Value>().unwrap();
        assert_eq!(t, 1.5 * UnitLength::Meter(Metric::None));
    }
}
