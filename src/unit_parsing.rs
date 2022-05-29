/**
 * File    :> unit_parsing.rs
 * Author  :> Bax
 * Version :> 0.0.1
 * Details :>
 */

use std::cmp::Ordering;
use crate::common::*;

/**
 * Function :> pub
 * Args :.
 *  - m : &Metric
 * Returns :> Result of String, error of V2Error
 * Details :> returns the string of a metric prefix
 */
pub fn get_metric_str(m:&Metric) -> Result<String, V2Error> {
    let temp = match m {
        Metric::Empty => "",
        Metric::Deca => "da",
        Metric::Atto => "a",
        Metric::Centi => "c",
        Metric::Deci => "d",
        Metric::Exa => "E",
        Metric::Femto => "f",
        Metric::Giga => "G",
        Metric::Hecto => "h",
        Metric::Kilo => "k",
        Metric::Mega => "M",
        Metric::Micro => "μ",
        Metric::Milli => "m",
        Metric::Nano => "n",
        Metric::Peta => "P",
        Metric::Pico => "p",
        Metric::Tera => "T",
        Metric::Yocto => "y",
        Metric::Yotta => "Y",
        Metric::Zepto => "z",
        Metric::Zetta => "Z",
    };
    Ok(temp.to_string())
}

/**
 * Function :> pub
 * Args :.
 *  - ty : &UnitTypes
 * Returns :> Result of String, error of V2Error
 * Details :> returns the string of a given unit
 */
pub fn get_unit_str(ty:&UnitTypes) -> Result<String, V2Error> {
    let temp = match ty {
        UnitTypes::Length(UnitLength::Meter) => "m", //
        UnitTypes::Length(UnitLength::Angstrom) => "Å",
        UnitTypes::Length(UnitLength::AstronomicalUnit) => "AU",
        UnitTypes::Length(UnitLength::Foot) => "ft",
        UnitTypes::Length(UnitLength::Inch) => "in",
        UnitTypes::Length(UnitLength::LightYear) => "lyr",
        UnitTypes::Length(UnitLength::Mile) => "miles",
        UnitTypes::Length(UnitLength::Parsec) => "pc",
        UnitTypes::Length(UnitLength::Yard) => "yds",

        UnitTypes::AbsorbedDose(UnitAbsorbedDose::Gray) => "Gy", //
        UnitTypes::AbsorbedDose(UnitAbsorbedDose::Rad) => "rads",
        UnitTypes::AbsorbedDose(UnitAbsorbedDose::Roentgen) => "R", // 

        UnitTypes::Angle(UnitAngle::Degree) => "°",
        UnitTypes::Angle(UnitAngle::Mil) => "mil",
        UnitTypes::Angle(UnitAngle::Moa) => "moa",
        UnitTypes::Angle(UnitAngle::Radian) => "rad",

        UnitTypes::Capacitance(UnitCapacitance::Farad) => "F",

        UnitTypes::CatalyticActivity(UnitCatalyticActivity::Katal) => "kat",

        UnitTypes::ElectricalConductance(UnitElectricConductance::Siemens) => "S",

        UnitTypes::ElectricCharge(UnitElectricCharge::Coulomb) => "C",

        UnitTypes::ElectricCurrent(UnitElectricCurrent::Ampere) => "A",

        UnitTypes::ElectricPotential(UnitElectricPotential::Volt) => "V",

        UnitTypes::Energy(UnitEnergy::Kilocalorie) => "Cal",
        UnitTypes::Energy(UnitEnergy::Calorie) => "cal",
        UnitTypes::Energy(UnitEnergy::ElectronVolt) => "eV",
        UnitTypes::Energy(UnitEnergy::FootPound) => "ftlb",
        UnitTypes::Energy(UnitEnergy::Joule) => "J",

        UnitTypes::Force(UnitForce::Newton) => "N",
        UnitTypes::Force(UnitForce::PoundForce) => "lbfr",

        UnitTypes::Frequency(UnitFrequency::Hertz) => "Hz",

        UnitTypes::Illuminance(UnitIlluminance::Lux) => "lx",

        UnitTypes::Inductance(UnitInductance::Henry) => "H",

        UnitTypes::Information(UnitInformation::Bit) => "bits",
        UnitTypes::Information(UnitInformation::Byte) => "b",

        UnitTypes::LuminousFlux(UnitLuminousFlux::Lumen) => "lm",

        UnitTypes::LuminousIntensity(UnitLuminousIntensity::Candela) => "cd",

        UnitTypes::MagneticFlux(UnitMagneticFlux::Weber) => "Wb",

        UnitTypes::MagneticFluxDensity(UnitMagneticFluxDensity::Tesla) => "T",

        UnitTypes::Mass(UnitMass::Grain) => "gr",
        UnitTypes::Mass(UnitMass::Gram) => "g",
        UnitTypes::Mass(UnitMass::Ounce) => "oz",
        UnitTypes::Mass(UnitMass::Pound) => "lb",

        UnitTypes::None => "",

        UnitTypes::Power(UnitPower::Watt) => "W",

        UnitTypes::Pressure(UnitPressure::Atm) => "atm",
        UnitTypes::Pressure(UnitPressure::Bar) => "bar",
        UnitTypes::Pressure(UnitPressure::Hgin) => "inHg",
        UnitTypes::Pressure(UnitPressure::Hgmm) => "mmHg",
        UnitTypes::Pressure(UnitPressure::Pascal) => "Pa",
        UnitTypes::Pressure(UnitPressure::Psi) => "psi",
        UnitTypes::Pressure(UnitPressure::Torr) => "torr",

        UnitTypes::Radioactivity(UnitRadioactivity::Becquerel) => "Bq",
        UnitTypes::Radioactivity(UnitRadioactivity::Curie) => "Ci",

        UnitTypes::RadioactivityExposure(UnitRadioactivityExposure::Rem) => "rem",
        UnitTypes::RadioactivityExposure(UnitRadioactivityExposure::Sievert) => "Sv",

        UnitTypes::Resistance(UnitResistance::Ohm) => "Ω",

        UnitTypes::Sound(UnitSound::Bel) => "B",

        UnitTypes::Substance(UnitSubstance::Mole) => "mol",

        UnitTypes::Temperature(UnitTemperature::Celsius) => "c",
        UnitTypes::Temperature(UnitTemperature::Fahrenheit) => "f",
        UnitTypes::Temperature(UnitTemperature::Kelvin) => "K",

        UnitTypes::Time(UnitTime::Day) => "days",
        UnitTypes::Time(UnitTime::Hour) => "hrs",
        UnitTypes::Time(UnitTime::Minute) => "min",
        UnitTypes::Time(UnitTime::Second) => "s",

        UnitTypes::Volume(UnitVolume::Liter) => "l",
    };
    Ok(temp.to_string())
}


/**
 * Function :> _get_single_letter
 * Args :.
 *  - label : &str
 * Returns :> Result of UnitTypes, error of V2Error
 * Details :> return unit based upon single character representation
 */
fn _get_single_letter(label:&str) -> Result<UnitTypes, V2Error>{
    match label {
        "m" => {
            Ok(UnitTypes::Length(UnitLength::Meter))
        } // meter
        "g" => {
            Ok(UnitTypes::Mass(UnitMass::Gram))
        } // gram
        "s" => {
            Ok(UnitTypes::Time(UnitTime::Second))
        } // second
        "A" => {
            Ok(UnitTypes::ElectricCurrent(UnitElectricCurrent::Ampere))
        } // ampere
        "J" => {
            Ok(UnitTypes::Energy(UnitEnergy::Joule))
        } // joule
        "W" => {
            Ok(UnitTypes::Power(UnitPower::Watt))
        } // watt
        "C" => {
            Ok(UnitTypes::ElectricCharge(UnitElectricCharge::Coulomb))
        } // coulomb
        "F" => {
            Ok(UnitTypes::Capacitance(UnitCapacitance::Farad))
        } // farad
        "Ω" | "O" => {
            Ok(UnitTypes::Resistance(UnitResistance::Ohm))
        } // ohm
        "S" => {
            Ok(UnitTypes::ElectricalConductance(UnitElectricConductance::Siemens))
        } // siemens
        "T" => {
            Ok(UnitTypes::MagneticFluxDensity(UnitMagneticFluxDensity::Tesla))
        } // tesla
        "c" => {
            Ok(UnitTypes::Temperature(UnitTemperature::Celsius))
        } // celsius
        "N" => {
            Ok(UnitTypes::Force(UnitForce::Newton))
        } // newton
        "K" => {
            Ok(UnitTypes::Temperature(UnitTemperature::Kelvin))
        } // kelvin
        "H" => {
            Ok(UnitTypes::Inductance(UnitInductance::Henry))
        } // henry
        "V" => {
            Ok(UnitTypes::ElectricPotential(UnitElectricPotential::Volt))
        } // volt
        "B" => {
            Ok(UnitTypes::Sound(UnitSound::Bel))
        } // bel
        "b" => {
            Ok(UnitTypes::Information(UnitInformation::Byte))
        }
        "°" => {
            Ok(UnitTypes::Angle(UnitAngle::Degree))
        } // degree
        "Å" => {
            Ok(UnitTypes::Length(UnitLength::Angstrom))
        } // angstrom
        "R" => {
            Ok(UnitTypes::AbsorbedDose(UnitAbsorbedDose::Roentgen))
        }
        "l" => {
            Ok(UnitTypes::Volume(UnitVolume::Liter))
        }
        "f" => {
            Ok(UnitTypes::Temperature(UnitTemperature::Fahrenheit))
        }
        _ => {
            Err(V2Error::UnsupportedUnit(label.to_string()))
        }
    }
}

/**
 * Function :> _get_double_letter
 * Args :.
 *  - label : &str
 *  - count : usize
 * Returns :> Result of UnitTypes, error of V2Error
 * Details :> return unit based upon double character representation
 */
fn _get_double_letter(label:&str, count:usize) -> Result<UnitTypes, V2Error> {
    match label {
        // first we check unique 2 letter units
        "Hz" => {
            Ok(UnitTypes::Frequency(UnitFrequency::Hertz))
        } // hertz
        "Pa" => {
            Ok(UnitTypes::Pressure(UnitPressure::Pascal))
        } // pascal
        "Wb" => {
            Ok(UnitTypes::MagneticFlux(UnitMagneticFlux::Weber))
        } // weber
        "lm" => {
            Ok(UnitTypes::LuminousFlux(UnitLuminousFlux::Lumen))
        } // lumen
        "lx" => {
            Ok(UnitTypes::Illuminance(UnitIlluminance::Lux))
        } // lux
        "Bq" => {
            Ok(UnitTypes::Radioactivity(UnitRadioactivity::Becquerel))
        } // becquerel
        "Sv" => {
            Ok(UnitTypes::RadioactivityExposure(UnitRadioactivityExposure::Sievert))
        } // sievert
        "cd" => {
            Ok(UnitTypes::LuminousIntensity(UnitLuminousIntensity::Candela))
        } // candela
        "au" | "AU" => {
            Ok(UnitTypes::Length(UnitLength::AstronomicalUnit))
        } // astronomical unit
        "eV" => {
            Ok(UnitTypes::Energy(UnitEnergy::ElectronVolt))
        } // electron volt
        "°C" => {
            Ok(UnitTypes::Temperature(UnitTemperature::Celsius))
        } // celsius
        "pc" => {
            Ok(UnitTypes::Length(UnitLength::Parsec))
        } // parsec
        "Ci" => {
            Ok(UnitTypes::Radioactivity(UnitRadioactivity::Curie))
        }
        "Gy" => {
            Ok(UnitTypes::AbsorbedDose(UnitAbsorbedDose::Gray))
        }
        // then we check single letter units
        _ => {
            if count >= 1 {
                let t:UnitTypes = _get_single_letter(&label[1..])?;
                Ok(t)
            } else {
                Err(V2Error::UnsupportedUnit(label.to_string()))
            }
        }
    }
}

/**
 * Function :> _get_triple_letter
 * Args :.
 *  - label : &str
 *  - count : usize
 * Returns :> Result of UnitTypes, error of V2Error
 * Details :> return unit based upon triple character representation
 */
fn _get_triple_letter(label:&str, count:usize) -> Result<UnitTypes, V2Error> {
    if let Some(stripped) = label.strip_prefix("da") {
        let t:UnitTypes = _get_single_letter(stripped)?;
        return Ok(t);
    }
    match label {
        "mol" => {
            Ok(UnitTypes::Substance(UnitSubstance::Mole))
        } // mole
        "kat" => {
            Ok(UnitTypes::CatalyticActivity(UnitCatalyticActivity::Katal))
        } // katal 
        "rad" => {
            Ok(UnitTypes::Angle(UnitAngle::Radian))
        } // radian
        "bar" => {
            Ok(UnitTypes::Pressure(UnitPressure::Bar))
        } // bar 
        "Cal" => {
            Ok(UnitTypes::Energy(UnitEnergy::Kilocalorie))
        }
        "cal" => {
            Ok(UnitTypes::Energy(UnitEnergy::Calorie))
        }
        _ => {
            if count >= 1 {
                let t:UnitTypes = _get_double_letter(&label[1..], count-1)?;
                Ok(t)
            } else {
                Err(V2Error::UnsupportedUnit(label.to_string()))
            }
        }
    }
}

/**
 * Function :> _get_quad_letter
 * Args :.
 *  - label : &str
 *  - count : usize
 * Returns :> Result of UnitTypes, error of V2Error
 * Details :> return unit based upon quadruple character representation
 */
fn _get_quad_letter(label:&str, count:usize) -> Result<UnitTypes, V2Error> {
    if let Some(stripped) = label.strip_prefix("da") {
        let t:UnitTypes = _get_double_letter(stripped, count-1)?;
        Ok(t)
    } else {
        match label {
            "torr" => {
                Ok(UnitTypes::Pressure(UnitPressure::Torr))
            }
            "bits" => {
                Ok(UnitTypes::Information(UnitInformation::Bit))
            }
            _ => {
                if count >= 1 {
                    let t:UnitTypes = _get_triple_letter(&label[1..], count-1)?;
                    Ok(t)
                } else {
                    Err(V2Error::UnsupportedUnit(label.to_string()))
                }
            }
        }
    }
}

/**
 * Function :> _get_penta_letter
 * Args :.
 *  - label : &str
 *  - count : usize
 * Returns :> Result of UnitTypes, error of V2Error
 * Details :> return unit based upon pentuple character representation
 */
fn _get_penta_letter(label:&str, count:usize) -> Result<UnitTypes, V2Error> {
    if let Some(stripped) = label.strip_prefix("da") {
        let t:UnitTypes = _get_triple_letter(stripped, count-1)?;
        Ok(t)
    } else {
        let t:UnitTypes = _get_quad_letter(&label[1..], count-1)?;
        Ok(t)
    }
}

/**
 * Function :> _create_unit
 * Args :.
 *  - label : &str
 *  - exp : f64
 * Returns :> Result of Vec<Unit>, error of V2Error
 * Details :> creates a new unit with a given string and exponent
 */
fn _create_unit(label:&str, exp:f64) -> Result<Vec<Unit>, V2Error> {
    // This will be the vector we return
    let mut ret:Vec<Unit> = Vec::new();

    // This indicates if we found a type
    // If we reach the end, and we haven't determined what the given
    // label is, we throw an error
    let mut found_label:bool = false;

    // First we check standard units that are actually 
    // combinations of other units (mph, kmh, etc...)
    if label.cmp("mph") == Ordering::Equal {
        let t1:Unit = Unit::new(
            UnitTypes::Length(UnitLength::Mile),
            Metric::Empty,
            1.0*exp
        );
        let t2:Unit = Unit::new(
            UnitTypes::Time(UnitTime::Hour),
            Metric::Empty,
            -1.0*exp
        );

        ret.push(t1);
        ret.push(t2);
        return Ok(ret);
    }

    if label.cmp("kph") == Ordering::Equal {
        let t1:Unit = Unit::new(
            UnitTypes::Length(UnitLength::Meter),
            Metric::Kilo,
            1.0*exp
        );
        let t2:Unit = Unit::new(
            UnitTypes::Time(UnitTime::Hour),
            Metric::Empty,
            -1.0*exp
        );
        ret.push(t1);
        ret.push(t2);
        return Ok(ret);
    }

    /* Parsing units goes as follows: 
     * We first check for imperial units and unique labels. These consist of 
     * more unconventional naming methods. These should be checked 
     * without any major string parsing techniques. 
     * 
     * If it cannot be parsed by the 'imperial' methods, it is then 
     * checked if it is a metric unit. This consists of looking at the 
     * last two characters. If those characters match any of the known 
     * '2 character' units of measurement, great parse them.
     * 
     * If not, we then check the last character to be any of the 
     * 'single' character measurements. If those match then we have our 
     * known measurement. 
     * 
     * If we cannot parse the string at all, we return an error 
     * indicating the given string is not any known unit of measurement.
     */
    match label {
        "mmHg" => {
            ret.push(Unit::new(UnitTypes::Pressure(UnitPressure::Hgmm), Metric::Empty, exp));
            found_label = true;
        }
        "inHg" => {
            ret.push(Unit::new(UnitTypes::Pressure(UnitPressure::Hgin), Metric::Empty, exp));
            found_label = true;
        }
        "byte" | "bytes" => {
            ret.push(Unit::new(UnitTypes::Information(UnitInformation::Byte), Metric::Empty, exp));
            found_label = true;
        }
        "bits" | "bit" => {
            ret.push(Unit::new(UnitTypes::Information(UnitInformation::Bit), Metric::Empty, exp));
            found_label = true;
        }
        "radian" | "radians" => {
            ret.push(Unit::new(UnitTypes::Angle(UnitAngle::Radian), Metric::Empty, exp));
            found_label = true;
        }
        "angstrom" | "angstroms" => {
            ret.push(Unit::new(UnitTypes::Length(UnitLength::Angstrom), Metric::Empty, exp));
            found_label = true;
        }
        "inch" | "in" => {
            ret.push(Unit::new(UnitTypes::Length(UnitLength::Inch), Metric::Empty, exp));
            found_label = true;
        }
        "foot" | "feet" | "ft" => {
            ret.push(Unit::new(UnitTypes::Length(UnitLength::Foot), Metric::Empty, exp));
            found_label = true;
        }
        "yard" | "yards" | "yd" | "yds" => {
            ret.push(Unit::new(UnitTypes::Length(UnitLength::Yard), Metric::Empty, exp));
            found_label = true;
        }
        "mile" | "miles" => {
            ret.push(Unit::new(UnitTypes::Length(UnitLength::Mile), Metric::Empty, exp));
            found_label = true;
        }
        "atm" | "ATM" => {
            ret.push(Unit::new(UnitTypes::Pressure(UnitPressure::Atm), Metric::Empty, exp));
            found_label = true;
        }
        "psi" | "PSI" => {
            ret.push(Unit::new(UnitTypes::Pressure(UnitPressure::Psi), Metric::Empty, exp));
            found_label = true;
        }
        "f" | "°f" | "°F" => {
            ret.push(Unit::new(UnitTypes::Temperature(UnitTemperature::Fahrenheit), Metric::Empty, exp));
            found_label = true;
        }
        "footpound" | "footpounds" | "ftlb" | "ftlbs" => {
            ret.push(Unit::new(UnitTypes::Energy(UnitEnergy::FootPound), Metric::Empty, exp));
            found_label = true;
        }
        "poundforce" | "lbfr" | "lbsfr" => {
            ret.push(Unit::new(UnitTypes::Force(UnitForce::PoundForce), Metric::Empty, exp));
            found_label = true;
        }
        "ounce" | "ounces" | "oz" => {
            ret.push(Unit::new(UnitTypes::Mass(UnitMass::Ounce), Metric::Empty, exp));
            found_label = true;
        }
        "grain" | "grains" | "gr" => {
            ret.push(Unit::new(UnitTypes::Mass(UnitMass::Grain), Metric::Empty, exp));
            found_label = true;
        }
        "pound" | "lbs" | "lb" => {
            ret.push(Unit::new(UnitTypes::Mass(UnitMass::Pound), Metric::Empty, exp));
            found_label = true;
        }
        "moa" | "MOA" => {
            ret.push(Unit::new(UnitTypes::Angle(UnitAngle::Moa), Metric::Empty, exp));
            found_label = true;
        }
        "rads" | "Rads" => {
            ret.push(Unit::new(UnitTypes::AbsorbedDose(UnitAbsorbedDose::Rad), Metric::Empty, exp));
            found_label = true;
        }
        "rem" | "Rem" => {
            ret.push(Unit::new(UnitTypes::RadioactivityExposure(UnitRadioactivityExposure::Rem), Metric::Empty, exp));
            found_label = true;
        }
        "mil" | "MIL" => {
            ret.push(Unit::new(UnitTypes::Angle(UnitAngle::Mil), Metric::Empty, exp));
            found_label = true;
        }
        "degrees" | "degree" => {
            ret.push(Unit::new(UnitTypes::Angle(UnitAngle::Degree), Metric::Empty, exp));
            found_label = true;
        }
        "lightyear" | "lightyears" | "lyr" => {
            ret.push(Unit::new(UnitTypes::Length(UnitLength::LightYear), Metric::Empty, exp));
            found_label = true;
        }
        "farad" | "farads" => {
            ret.push(Unit::new(UnitTypes::Capacitance(UnitCapacitance::Farad), Metric::Empty, exp));
            found_label = true;
        }
        "micron" | "microns" => {
            ret.push(Unit::new(UnitTypes::Length(UnitLength::Meter), Metric::Micro, exp));
            found_label = true;
        }
        "min" | "minute" | "minutes" => {
            ret.push(Unit::new(UnitTypes::Time(UnitTime::Minute), Metric::Empty, exp));
            found_label = true;
        }
        "h" | "hour" | "hours" => {
            println!("Here!");
            ret.push(Unit::new(UnitTypes::Time(UnitTime::Hour), Metric::Empty, exp));
            found_label = true;
        }
        "d" | "day" | "days" => {
            ret.push(Unit::new(UnitTypes::Time(UnitTime::Day), Metric::Empty, exp));
            found_label = true;
        }
        _ => {
            // Do nothing
        }
    }

    if found_label {
        return Ok(ret);
    }

    let l:usize = label.chars().count();
    let mut m:Metric = Metric::Empty;

    if l == 0 && !found_label {
        ret.push(Unit::new(UnitTypes::None, Metric::Empty, exp));
    } else if l == 1 {
        let t:UnitTypes = _get_single_letter(label)?;
        ret.push(Unit::new(t, Metric::Empty, exp));
        found_label = true;
    } else if l == 2 && !found_label {
        let t:UnitTypes = _get_double_letter(label, 1)?;
        if get_unit_str(&t)?.chars().count() < 2 {
            m = _get_metric(label, &t)?;
        }

        if t == UnitTypes::Information(UnitInformation::Byte) {
            match m {
                Metric::Empty => {}
                Metric::Yotta => {}
                Metric::Zetta => {}
                Metric::Exa => {}
                Metric::Peta => {}
                Metric::Tera => {}
                Metric::Giga => {}
                Metric::Mega => {}
                Metric::Kilo => {}
                _ => {
                    return Err(V2Error::ParsingError("Unsupported byte metric prefix".to_string()));
                }
            }
        }

        ret.push(Unit::new(t, m, exp));
        found_label = true;
    } else if l == 3 && !found_label {
        let t:UnitTypes = _get_triple_letter(label, 1)?;
        if get_unit_str(&t)?.chars().count() < 3 {
            m = _get_metric(label, &t)?;
        }
        ret.push(Unit::new(t, m, exp));
        found_label = true;
    } else if l == 4 && !found_label {
        let t:UnitTypes = _get_quad_letter(label, 1)?;
        if get_unit_str(&t)?.chars().count() < 4 {
            m = _get_metric(label, &t)?;
        }
        ret.push(Unit::new(t, m, exp));
        found_label = true;
    } else if l == 5 && !found_label {
        let t:UnitTypes = _get_penta_letter(label, 1)?;
        if get_unit_str(&t)?.chars().count() < 5 {
            m = _get_metric(label, &t)?;
        }
        if t == UnitTypes::Information(UnitInformation::Bit) {
            match m {
                Metric::Empty => {}
                Metric::Yotta => {}
                Metric::Zetta => {}
                Metric::Exa => {}
                Metric::Peta => {}
                Metric::Tera => {}
                Metric::Giga => {}
                Metric::Mega => {}
                Metric::Kilo => {}
                _ => {
                    return Err(V2Error::ParsingError("Unsupported bit metric prefix".to_string()));
                }
            }
        }
        ret.push(Unit::new(t, m, exp));
        found_label = true;
    }

    // By this point we should have found some label
    if !found_label {
        return Err(V2Error::ParsingError("Failed formating".to_string()));
    }
    Ok(ret)
}

/**
 * Function :> _get_metric
 * Args :.
 *  - full_label : &str
 *  - ut : &UnitTypes
 * Returns :> Result of Metric, error of V2Error
 * Details :> determines if a unit is a valid SI unit and can use metric prefixes
 */
fn _get_metric(full_label:&str, ut:&UnitTypes) -> Result<Metric, V2Error> {

    // We make sure that we only 'get metrics' for SI and compatible units 
    match ut {
        UnitTypes::Time(UnitTime::Second) => {}
        UnitTypes::Length(UnitLength::Meter) => {}
        UnitTypes::Mass(UnitMass::Gram) => {}
        UnitTypes::ElectricCurrent(UnitElectricCurrent::Ampere) => {}
        UnitTypes::Substance(UnitSubstance::Mole) => {}
        UnitTypes::LuminousIntensity(UnitLuminousIntensity::Candela) => {}
        UnitTypes::Frequency(UnitFrequency::Hertz) => {}
        UnitTypes::Force(UnitForce::Newton) => {}
        UnitTypes::Pressure(UnitPressure::Pascal) => {}
        UnitTypes::Pressure(UnitPressure::Bar) => {}
        UnitTypes::Energy(UnitEnergy::Joule) => {}
        UnitTypes::Power(UnitPower::Watt) => {}
        UnitTypes::ElectricCharge(UnitElectricCharge::Coulomb) => {}
        UnitTypes::ElectricPotential(UnitElectricPotential::Volt) => {}
        UnitTypes::Capacitance(UnitCapacitance::Farad) => {}
        UnitTypes::Resistance(UnitResistance::Ohm) => {}
        UnitTypes::ElectricalConductance(UnitElectricConductance::Siemens) => {}
        UnitTypes::MagneticFlux(UnitMagneticFlux::Weber) => {}
        UnitTypes::MagneticFluxDensity(UnitMagneticFluxDensity::Tesla) => {}
        UnitTypes::Inductance(UnitInductance::Henry) => {}
        UnitTypes::Illuminance(UnitIlluminance::Lux) => {}
        UnitTypes::LuminousFlux(UnitLuminousFlux::Lumen) => {}
        UnitTypes::Radioactivity(UnitRadioactivity::Becquerel) => {}
        UnitTypes::AbsorbedDose(UnitAbsorbedDose::Gray) => {}
        UnitTypes::RadioactivityExposure(UnitRadioactivityExposure::Sievert) => {}
        UnitTypes::CatalyticActivity(UnitCatalyticActivity::Katal) => {}
        UnitTypes::Volume(UnitVolume::Liter) => {}
        UnitTypes::Pressure(UnitPressure::Torr) => {}
        UnitTypes::Energy(UnitEnergy::Calorie) => {}
        UnitTypes::Sound(UnitSound::Bel) => {}
        UnitTypes::Information(UnitInformation::Bit) => {}
        UnitTypes::Information(UnitInformation::Byte) => {}
        _ => {return Err(V2Error::NonMetricUnit(*ut))}
    }

    if full_label.starts_with("da") {
        return Ok(Metric::Deca);
    } 

    let c = full_label.chars().next().unwrap();
    match c {
        'Y' => Ok(Metric::Yotta),
        'Z' => Ok(Metric::Zetta),
        'E' => Ok(Metric::Exa),
        'P' => Ok(Metric::Peta),
        'T' => Ok(Metric::Tera),
        'G' => Ok(Metric::Giga),
        'M' => Ok(Metric::Mega),
        'k' => Ok(Metric::Kilo),
        'h' => Ok(Metric::Hecto),
        'd' => Ok(Metric::Deci),
        'c' => Ok(Metric::Centi),
        'm' => Ok(Metric::Milli),
        'u' => Ok(Metric::Micro),
        'μ' => Ok(Metric::Micro),
        'n' => Ok(Metric::Nano),
        'p' => Ok(Metric::Pico),
        'f' => Ok(Metric::Femto),
        'a' => Ok(Metric::Atto),
        'z' => Ok(Metric::Zepto),
        'y' => Ok(Metric::Yocto),
        _ => Ok(Metric::Empty)
    }
}

/**
 * Function :> _get_tokens
 * Args :.
 *  - block : &str
 *  - do_denom : bool
 * Returns :> Result of (Vec<String>, error of Vec<String>)
 * Details :> gets the pieces of a string for parsing
 */
fn _get_tokens(block:&str, do_denom:bool) -> Result<(Vec<String>, Vec<String>), V2Error> {
    let mut numor:Vec<String> = Vec::new();
    let mut denom:Vec<String> = Vec::new();

    // first we find the outer most parentheses
    // if there are non we just continue
    let mut left_count:usize = 0;
    let mut start_index:usize = 0;
    let mut end_index:usize;
    let mut found_divisor:bool = do_denom;
    let mut constructor:String = String::new();
    for index in 0..block.chars().count() {
        let c:char = block.chars().nth(index).unwrap();
        match c {
            '(' => {
                if left_count == 0 {
                    start_index = index+1;
                }
                left_count+=1;
            }
            ')' => {
                left_count-=1;
                if left_count == 0 {
                    end_index = index;
                    let mut ret = _get_tokens(&block[start_index..end_index], found_divisor)?;
                    numor.append(&mut ret.0);
                    denom.append(&mut ret.1);
                }
            }
            '/' => {
                if !found_divisor {
                    found_divisor = true;
                } else {
                    return Err(V2Error::ParsingError("Too many divisors".to_string()));
                }
                if !found_divisor && !constructor.is_empty() {
                    denom.push(constructor.clone());
                } else if !constructor.is_empty() {
                    numor.push(constructor.clone());
                }
                constructor = String::new();
            }
            _ => {
                if left_count == 0 {
                    if c.is_whitespace() {
                        // Do nothing
                    } else if c == '*' {
                        if !do_denom && !found_divisor {
                            numor.push(constructor.clone());
                        } else {
                            denom.push(constructor.clone());
                        }
                        constructor = String::new();
                    } else {
                        constructor.push(c);
                    }
                }
            }
        };
    }

    if !constructor.is_empty() {
        if !do_denom && !found_divisor {
            numor.push(constructor.clone());
        } else {
            denom.push(constructor.clone());
        }
    }

    Ok((numor, denom))
}

/**
 * Function :> pub
 * Args :.
 *  - label : &str
 * Returns :> Result of Vec<Unit>, error of V2Error
 * Details :> parses a string to create a list of units 
 */
pub fn parse(label:&str) -> Result<Vec<Unit>, V2Error> {
    let mut ret:Vec<Unit> = vec![];

    if label.is_empty() {
        ret.push(Unit::new(
            UnitTypes::None,
            Metric::Empty,
            0.0
        ));
        return Ok(ret);
    }

    let tokens = _get_tokens(label, false)?;
    
    // do the numors first
    for t in tokens.0 {
        let mut expon:f64 = 1.0;
        let temp_split:Vec<&str> = t.split('^').collect();
        if temp_split.len() > 1 {
            expon = temp_split[1].parse::<f64>().unwrap();
        }
        ret.append(&mut _create_unit(temp_split[0], expon)?);
    }

    // now the denoms
    for t in tokens.1 {
        let mut expon:f64 = -1.0;
        let temp_split:Vec<&str> = t.split('^').collect();
        if temp_split.len() > 1 {
            expon *= temp_split[1].parse::<f64>().unwrap();
        }
        ret.append(&mut _create_unit(temp_split[0], expon)?);
    }

    Ok(ret)
}

#[cfg(test)]
mod unit_parsing_testing {

    #[test]
    /**
     * Function :> parsing_1
     * Details :>
     */
    fn parsing_1(){
        let temp = "kmol/ kg^2";
        let tok = super::_get_tokens(&temp, false).unwrap();
        assert_eq!(tok.0[0], "kmol");
        assert_eq!(tok.1[0], "kg^2");
    }

    #[test]
    /**
     * Function :> parsing_2
     * Details :>
     */
    fn parsing_2(){
        let temp = "(kmol*kg^2)/(s(J^2))";
        let tok = super::_get_tokens(&temp, false).unwrap();
        assert_eq!(tok.0[0], "kmol");
        assert_eq!(tok.0[1], "kg^2");
        assert_eq!(tok.1[0], "J^2");
        assert_eq!(tok.1[1], "s");
    }

    #[test]
    /**
     * Function :> parsing_3
     * Details :>
     */
    fn parsing_3(){
        let temp = "1/C";
        let tok = super::_get_tokens(&temp, false).unwrap();
        assert_eq!(tok.1[0], "C");
        assert_eq!(tok.0[0], "1");
    }
}