use crate::{
    consts::{
        ABSORBED_DOSE_INDEX, ABSORBED_DOSE_MAP, ANGLE_INDEX, ANGLE_MAP, CAPACITANCE_INDEX,
        CAPACITANCE_MAP, CATALYTIC_ACTIVITY_INDEX, CATALYTIC_ACTIVITY_MAP, ELECTRIC_CHARGE_INDEX,
        ELECTRIC_CHARGE_MAP, ELECTRIC_CONDUCTANCE_INDEX, ELECTRIC_CONDUCTANCE_MAP,
        ELECTRIC_CURRENT_INDEX, ELECTRIC_CURRENT_MAP, ELECTRIC_POTENTIAL_INDEX,
        ELECTRIC_POTENTIAL_MAP, ENERGY_INDEX, ENERGY_MAP, FORCE_INDEX, FORCE_MAP, FREQUENCY_INDEX,
        FREQUENCY_MAP, ILLUMINANCE_INDEX, ILLUMINANCE_MAP, INDUCTANCE_INDEX, INDUCTANCE_MAP,
        INFORMATION_INDEX, INFORMATION_MAP, LENGTH_INDEX, LENGTH_MAP, LUMINOUS_FLUX_INDEX,
        LUMINOUS_FLUX_MAP, LUMINOUS_INTENSITY_INDEX, LUMINOUS_INTENSITY_MAP,
        MAGNETIC_FLUX_DENSITY_INDEX, MAGNETIC_FLUX_DENSITY_MAP, MAGNETIC_FLUX_INDEX,
        MAGNETIC_FLUX_MAP, MASS_INDEX, MASS_MAP, POWER_INDEX, POWER_MAP, PRESSURE_INDEX,
        PRESSURE_MAP, RADIOACTIVITY_EXPOSURE_INDEX, RADIOACTIVITY_EXPOSURE_MAP,
        RADIOACTIVITY_INDEX, RADIOACTIVITY_MAP, RESISTANCE_INDEX, RESISTANCE_MAP,
        SOLID_ANGLE_INDEX, SOLID_ANGLE_MAP, SOUND_INDEX, SOUND_MAP, SUBSTANCE_INDEX, SUBSTANCE_MAP,
        TEMPERATURE_INDEX, TEMPERATURE_MAP, TIME_INDEX, TIME_MAP, VOLUME_INDEX, VOLUME_MAP,
    },
    errors::V3Error,
    units::{
        Metric, UnitAbsorbedDose, UnitAngle, UnitCatalyticActivity, UnitElectricCapacitance,
        UnitElectricCharge, UnitElectricConductance, UnitElectricCurrent, UnitElectricInductance,
        UnitElectricPotential, UnitElectricResistance, UnitEnergy, UnitForce, UnitFrequency,
        UnitIlluminance, UnitInformation, UnitLength, UnitLuminousFlux, UnitLuminousIntensity,
        UnitMagneticFlux, UnitMagneticFluxDensity, UnitMass, UnitPower, UnitPressure,
        UnitRadioactivity, UnitRadioactivityExposure, UnitSolidAngle, UnitSound, UnitSubstance,
        UnitTemperature, UnitTime, UnitVolume,
    },
    Value,
};

impl Value {
    /// Creates a new unit type when constructing a [`Value`]
    pub(crate) fn _create_unit(&mut self, units: &str) -> Result<(), V3Error> {
        let tokens: (Vec<String>, Vec<String>) = Value::_get_tokens(units, false)?;

        // do the numors first
        for t in tokens.0 {
            let mut expon: i32 = 1;
            let temp_split: Vec<&str> = t.split('^').collect();
            if temp_split.len() > 1 {
                expon = match temp_split[1].parse::<i32>() {
                    Ok(t) => t,
                    Err(_) => {
                        return Err(V3Error::ParsingError(
                            "[_create_unit_1] Cannot parse int".into(),
                        ))
                    }
                };
            }
            self._parse_units(String::from(temp_split[0]), expon)?;
        }

        // now the denoms
        for t in tokens.1 {
            let mut expon: i32 = -1;
            let temp_split: Vec<&str> = t.split('^').collect();
            if temp_split.len() > 1 {
                expon *= match temp_split[1].parse::<i32>() {
                    Ok(t) => t,
                    Err(_) => {
                        return Err(V3Error::ParsingError(
                            "[_create_unit_2] Cannot parse int".into(),
                        ))
                    }
                };
            }
            self._parse_units(String::from(temp_split[0]), expon)?;
        }

        Ok(())
    }

    /// Tokenizes a given string for a new [`Value`] for easier parsing
    pub(crate) fn _get_tokens(
        block: &str,
        do_denom: bool,
    ) -> Result<(Vec<String>, Vec<String>), V3Error> {
        let mut numor: Vec<String> = Vec::new();
        let mut denom: Vec<String> = Vec::new();

        // first we find the outer most parentheses
        // if there are non we just continue
        let mut left_count: usize = 0;
        let mut start_index: usize = 0;
        let mut end_index: usize;
        let mut found_divisor: bool = do_denom;
        let mut constructor: String = String::new();
        for index in 0..block.chars().count() {
            let c: char = match block.chars().nth(index) {
                Some(t) => t,
                None => return Err(V3Error::ParsingError("[_get_tokens] Index error".into())),
            };
            match c {
                '(' => {
                    if left_count == 0 {
                        start_index = index + 1;
                    }
                    left_count += 1;
                }
                ')' => {
                    if left_count == 0 {
                        return Err(V3Error::ParsingError(
                            "[_get_tokens] Parenthesis Ordering".into(),
                        ));
                    }
                    left_count -= 1;
                    if left_count == 0 {
                        end_index = index;
                        let mut ret: (Vec<String>, Vec<String>) =
                            Self::_get_tokens(&block[start_index..end_index], found_divisor)?;
                        numor.append(&mut ret.0);
                        denom.append(&mut ret.1);
                    }
                }
                '/' => {
                    if !found_divisor && left_count == 0 {
                        found_divisor = true;
                    } else if left_count > 0 {
                        // Do nothing, this div is in parentheses
                    } else {
                        panic!("Too many divisors")
                    }
                    if !constructor.is_empty() {
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

    /// Searches and assigns a unit type to a [`Value`] during string parsing and construction
    fn _get_single_letter(&mut self, unit: char, exp: i32, m: Metric) -> Result<(), V3Error> {
        match unit {
            '1' => {
                // This handles the case of 1/m as a given string to parse
            }
            'm' => {
                self.v_length = Some(UnitLength::Meter(m));
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
            }
            'g' => {
                self.v_mass = Some(UnitMass::Gram(m));
                self.exp[MASS_INDEX] = exp;
                self.unit_map |= MASS_MAP;
            }
            's' => {
                self.v_time = Some(UnitTime::Second(m));
                self.exp[TIME_INDEX] = exp;
                self.unit_map |= TIME_MAP;
            }
            'A' => {
                self.v_electric_current = Some(UnitElectricCurrent::Ampere(m));
                self.exp[ELECTRIC_CURRENT_INDEX] = exp;
                self.unit_map |= ELECTRIC_CURRENT_MAP;
            }
            'J' => {
                self.v_energy = Some(UnitEnergy::Joule(m));
                self.exp[ENERGY_INDEX] = exp;
                self.unit_map |= ENERGY_MAP;
            }
            'W' => {
                self.v_power = Some(UnitPower::Watt(m));
                self.exp[POWER_INDEX] = exp;
                self.unit_map |= POWER_MAP;
            }
            'C' => {
                self.v_electric_charge = Some(UnitElectricCharge::Coulomb(m));
                self.exp[ELECTRIC_CHARGE_INDEX] = exp;
                self.unit_map |= ELECTRIC_CHARGE_MAP;
            }
            'F' => {
                self.v_capacitance = Some(UnitElectricCapacitance::Farad(m));
                self.exp[CAPACITANCE_INDEX] = exp;
                self.unit_map |= CAPACITANCE_MAP;
            }
            'Ω' | 'O' => {
                self.v_resistance = Some(UnitElectricResistance::Ohm(m));
                self.exp[RESISTANCE_INDEX] = exp;
                self.unit_map |= RESISTANCE_MAP;
            }
            'S' => {
                self.v_electric_conductance = Some(UnitElectricConductance::Siemens(m));
                self.exp[ELECTRIC_CONDUCTANCE_INDEX] = exp;
                self.unit_map |= ELECTRIC_CONDUCTANCE_MAP;
            }
            'T' => {
                self.v_magnetic_flux_density = Some(UnitMagneticFluxDensity::Tesla(m));
                self.exp[MAGNETIC_FLUX_DENSITY_INDEX] = exp;
                self.unit_map |= MAGNETIC_FLUX_DENSITY_MAP;
            }
            'N' => {
                self.v_force = Some(UnitForce::Newton(m));
                self.exp[FORCE_INDEX] = exp;
                self.unit_map |= FORCE_MAP;
            }
            'K' => {
                self.v_temperature = Some(UnitTemperature::Kelvin(m));
                self.exp[TEMPERATURE_INDEX] = exp;
                self.unit_map |= TEMPERATURE_MAP;
            }
            'c' => {
                self.v_temperature = Some(UnitTemperature::Celsius(m));
                self.exp[TEMPERATURE_INDEX] = exp;
                self.unit_map |= TEMPERATURE_MAP;
            }
            'H' => {
                self.v_inductance = Some(UnitElectricInductance::Henry(m));
                self.exp[INDUCTANCE_INDEX] = exp;
                self.unit_map |= INDUCTANCE_MAP;
            }
            'V' => {
                self.v_electric_potential = Some(UnitElectricPotential::Volt(m));
                self.exp[ELECTRIC_POTENTIAL_INDEX] = exp;
                self.unit_map |= ELECTRIC_POTENTIAL_MAP;
            }
            'B' => {
                self.v_sound = Some(UnitSound::Bel(m));
                self.exp[SOUND_INDEX] = exp;
                self.unit_map |= SOUND_MAP;
            }
            'b' => {
                self.v_information = Some(UnitInformation::Byte(m));
                self.exp[INFORMATION_INDEX] = exp;
                self.unit_map |= INFORMATION_MAP;
            }
            'Å' => {
                self.v_length = Some(UnitLength::Angstrom);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
            }
            'R' => {
                self.v_ab_dose = Some(UnitAbsorbedDose::Roentgen);
                self.exp[ABSORBED_DOSE_INDEX] = exp;
                self.unit_map |= ABSORBED_DOSE_MAP;
            }
            'l' => {
                self.v_volume = Some(UnitVolume::Liter(m));
                self.exp[VOLUME_INDEX] = exp;
                self.unit_map |= VOLUME_MAP;
            }
            _ => {
                return Err(V3Error::UnsupportedUnit(format!(
                    "[_get_single_letter] Unsupported unit: {}",
                    unit
                )));
            }
        }
        Ok(())
    }

    /// Searches and assigns a unit type to a [`Value`] during string parsing and construction
    fn _get_double_letter(&mut self, unit: &String, exp: i32, m: Metric) -> Result<(), V3Error> {
        match unit.as_str() {
            "Hz" => {
                self.v_frequency = Some(UnitFrequency::Hertz(m));
                self.exp[FREQUENCY_INDEX] = exp;
                self.unit_map |= FREQUENCY_MAP;
            }
            "Pa" => {
                self.v_pressure = Some(UnitPressure::Pascal(m));
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
            }
            "Wb" => {
                self.v_magnetic_flux = Some(UnitMagneticFlux::Weber(m));
                self.exp[MAGNETIC_FLUX_INDEX] = exp;
                self.unit_map |= MAGNETIC_FLUX_MAP;
            }
            "lm" => {
                self.v_luminous_flux = Some(UnitLuminousFlux::Lumen(m));
                self.exp[LUMINOUS_FLUX_INDEX] = exp;
                self.unit_map |= LUMINOUS_FLUX_MAP;
            }
            "lx" => {
                self.v_illuminance = Some(UnitIlluminance::Lux(m));
                self.exp[ILLUMINANCE_INDEX] = exp;
                self.unit_map |= ILLUMINANCE_MAP;
            }
            "Bq" => {
                self.v_radioactivity = Some(UnitRadioactivity::Becquerel(m));
                self.exp[RADIOACTIVITY_INDEX] = exp;
                self.unit_map |= RADIOACTIVITY_MAP;
            }
            "Sv" => {
                self.v_radioactivity_exposure = Some(UnitRadioactivityExposure::Sievert(m));
                self.exp[RADIOACTIVITY_EXPOSURE_INDEX] = exp;
                self.unit_map |= RADIOACTIVITY_EXPOSURE_MAP;
            }
            "cd" => {
                self.v_luminous_flux_intensity = Some(UnitLuminousIntensity::Candela(m));
                self.exp[LUMINOUS_INTENSITY_INDEX] = exp;
                self.unit_map |= LUMINOUS_INTENSITY_MAP;
            }
            "au" | "AU" => {
                self.v_length = Some(UnitLength::AstronomicalUnit);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
            }
            "pc" => {
                self.v_length = Some(UnitLength::Parsec(m));
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
            }
            "Ci" => {
                self.v_radioactivity = Some(UnitRadioactivity::Curie);
                self.exp[RADIOACTIVITY_INDEX] = exp;
                self.unit_map |= RADIOACTIVITY_MAP;
            }
            "Gy" => {
                self.v_ab_dose = Some(UnitAbsorbedDose::Gray(m));
                self.exp[ABSORBED_DOSE_INDEX] = exp;
                self.unit_map |= ABSORBED_DOSE_MAP;
            }
            "sr" => {
                self.v_solid_angle = Some(UnitSolidAngle::Steradian(m));
                self.exp[SOLID_ANGLE_INDEX] = exp;
                self.unit_map |= SOLID_ANGLE_MAP;
            }
            "eV" => {
                self.v_energy = Some(UnitEnergy::ElectronVolt(m));
                self.exp[ENERGY_INDEX] = exp;
                self.unit_map |= ENERGY_MAP;
                return Ok(());
            }
            "hp" => {
                self.v_power = Some(UnitPower::Horsepower);
                self.exp[POWER_INDEX] = exp;
                self.unit_map |= POWER_MAP;
                return Ok(());
            }
            _ => {
                if m != Metric::None {
                    return Err(V3Error::UnsupportedUnit(format!(
                        "[_get_double_letter] Unsupported unit: {}",
                        unit
                    )));
                }
                match self._get_metric(match &unit.chars().next() {
                    Some(t) => t,
                    None => {
                        // Unreachable all things being equal
                        return Err(V3Error::ParsingError(
                            "[_get_double_letter] Cannot get next metric char".into(),
                        ));
                    }
                }) {
                    Ok(new_m) => {
                        self._get_single_letter(unit.chars().nth(1).unwrap(), exp, new_m)?
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    /// Searches and assigns a unit type to a [`Value`] during string parsing and construction
    fn _get_triple_letter(&mut self, unit: &String, exp: i32, m: Metric) -> Result<(), V3Error> {
        if let Some(cel_deg) = unit.strip_prefix('°') {
            if m != Metric::None {
                return Err(V3Error::ParsingError(
                    "[_get_pentuple_letter] Already registered metric prefix".into(),
                ));
            }

            match self._get_metric(match &cel_deg.chars().next() {
                Some(t) => t,
                None => {
                    // Unreachable all things being equal
                    return Err(V3Error::ParsingError(
                        "[_get_triple_letter] Cannot get next metric char".into(),
                    ));
                }
            }) {
                Ok(new_m) => {
                    // Parsing strings is insane
                    return self._get_single_letter(cel_deg.chars().last().unwrap(), exp, new_m);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if let Some(da) = unit.strip_prefix("da") {
            if m != Metric::None {
                return Err(V3Error::ParsingError(
                    "[_get_pentuple_letter] Already registered metric prefix".into(),
                ));
            }
            return self._get_single_letter(da.chars().next().unwrap(), exp, Metric::Deca);
        }

        match unit.as_str() {
            "mol" => {
                self.v_substance = Some(UnitSubstance::Mole(m));
                self.exp[SUBSTANCE_INDEX] = exp;
                self.unit_map |= SUBSTANCE_MAP;
            }
            "kat" => {
                self.v_catalytic = Some(UnitCatalyticActivity::Katal(m));
                self.exp[CATALYTIC_ACTIVITY_INDEX] = exp;
                self.unit_map |= CATALYTIC_ACTIVITY_MAP;
            }
            "rad" => {
                self.v_angle = Some(UnitAngle::Radian(m));
                self.exp[ANGLE_INDEX] = exp;
                self.unit_map |= ANGLE_MAP;
            }
            "bar" => {
                self.v_pressure = Some(UnitPressure::Bar(m));
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
            }
            "cal" => {
                self.v_energy = Some(UnitEnergy::GramCalorie(m));
                self.exp[ENERGY_INDEX] = exp;
                self.unit_map |= ENERGY_MAP;
            }
            "Cal" => {
                self.v_energy = Some(UnitEnergy::GramCalorie(Metric::Kilo));
                self.exp[ENERGY_INDEX] = exp;
                self.unit_map |= ENERGY_MAP;
                return Ok(());
            }
            "lyr" => {
                self.v_length = Some(UnitLength::LightYear(m));
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            _ => {
                if m != Metric::None {
                    return Err(V3Error::UnsupportedUnit(format!(
                        "[_get_triple_letter] Unsupported unit: {}",
                        unit
                    )));
                }
                match self._get_metric(match &unit.chars().next() {
                    Some(t) => t,
                    None => {
                        // Unreachable all things being equal
                        return Err(V3Error::ParsingError(
                            "[_get_triple_letter] Cannot get next metric char".into(),
                        ));
                    }
                }) {
                    Ok(new_m) => {
                        // Parsing strings is insane
                        let t: Vec<char> = unit.chars().collect::<Vec<_>>();
                        self._get_double_letter(&t[1..].iter().collect::<String>(), exp, new_m)?
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    /// Searches and assigns a unit type to a [`Value`] during string parsing and construction
    fn _get_quadruple_letter(&mut self, unit: &String, exp: i32, m: Metric) -> Result<(), V3Error> {
        if let Some(cel_deg) = unit.strip_prefix("°da") {
            if m != Metric::None {
                return Err(V3Error::ParsingError(
                    "[_get_pentuple_letter] Already registered metric prefix".into(),
                ));
            }
            return self._get_single_letter(cel_deg.chars().last().unwrap(), exp, Metric::Deca);
        }

        if let Some(da) = unit.strip_prefix("da") {
            if m != Metric::None {
                return Err(V3Error::ParsingError(
                    "[_get_pentuple_letter] Already registered metric prefix".into(),
                ));
            }
            return self._get_double_letter(&da.to_string(), exp, Metric::Deca);
        }

        match unit.as_str() {
            "torr" => {
                self.v_pressure = Some(UnitPressure::Torr);
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
            }
            "bits" => {
                self.v_information = Some(UnitInformation::Bit(m));
                self.exp[INFORMATION_INDEX] = exp;
                self.unit_map |= INFORMATION_MAP;
            }
            _ => {
                if m != Metric::None {
                    return Err(V3Error::UnsupportedUnit(format!(
                        "[_get_quadruple_letter] Unsupported unit: {}",
                        unit
                    )));
                }
                match self._get_metric(match &unit.chars().next() {
                    Some(t) => t,
                    None => {
                        // Unreachable all things being equal
                        return Err(V3Error::ParsingError(
                            "[_get_quadruple_letter] Cannot get next metric char".into(),
                        ));
                    }
                }) {
                    Ok(new_m) => {
                        let t: Vec<char> = unit.chars().collect::<Vec<_>>();
                        self._get_triple_letter(&t[1..].iter().collect::<String>(), exp, new_m)?
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    /// Searches and assigns a unit type to a [`Value`] during string parsing and construction
    fn _get_pentuple_letter(&mut self, unit: &str, exp: i32, m: Metric) -> Result<(), V3Error> {
        if let Some(da) = unit.strip_prefix("da") {
            if m != Metric::None {
                return Err(V3Error::ParsingError(
                    "[_get_pentuple_letter] Already registered metric prefix".into(),
                ));
            }
            return self._get_triple_letter(&da.to_string(), exp, Metric::Deca);
        }

        if m != Metric::None {
            return Err(V3Error::UnsupportedUnit(format!(
                "[_get_pentuple_letter] Unsupported unit: {}",
                unit
            )));
        }
        match self._get_metric(match &unit.chars().next() {
            Some(t) => t,
            None => {
                // Unreachable all things being equal
                return Err(V3Error::ParsingError(
                    "[_get_pentuple_letter] Cannot get next metric char".into(),
                ));
            }
        }) {
            Ok(new_m) => {
                let t: Vec<char> = unit.chars().collect::<Vec<_>>();
                self._get_quadruple_letter(&t[1..].iter().collect::<String>(), exp, new_m)
            }
            Err(e) => Err(e),
        }
    }

    /// Searches and assigns a unit type to a [`Value`] during string parsing and construction
    fn _get_sextuple_letter(&mut self, unit: &str, exp: i32) -> Result<(), V3Error> {
        if let Some(da) = unit.strip_prefix("da") {
            return self._get_quadruple_letter(&da.to_string(), exp, Metric::Deca);
        }

        match self._get_metric(match &unit.chars().next() {
            Some(t) => t,
            None => {
                // Unreachable all things being equal
                return Err(V3Error::ParsingError(
                    "[_get_sextuple_letter] Cannot get next metric char".into(),
                ));
            }
        }) {
            Ok(new_m) => {
                let t: Vec<char> = unit.chars().collect::<Vec<_>>();
                self._get_pentuple_letter(&t[1..].iter().collect::<String>(), exp, new_m)
            }
            Err(e) => Err(e),
        }
    }

    /// Returns the `Metric` enum for a given prefix
    fn _get_metric(&mut self, unit: &char) -> Result<Metric, V3Error> {
        match unit {
            'Q' => Ok(Metric::Quetta),
            'R' => Ok(Metric::Ronna),
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
            'u' | 'μ' => Ok(Metric::Micro),
            'n' => Ok(Metric::Nano),
            'p' => Ok(Metric::Pico),
            'f' => Ok(Metric::Femto),
            'a' => Ok(Metric::Atto),
            'z' => Ok(Metric::Zepto),
            'y' => Ok(Metric::Yocto),
            'r' => Ok(Metric::Ronto),
            'q' => Ok(Metric::Quecto),
            _ => Err(V3Error::UnsupportedMetric(format!(
                "[_get_metric] Unsupported metric: {}",
                unit
            ))),
        }
    }

    /// Searches through the given string for a new [`Value`] to parse for units
    pub(crate) fn _parse_units(&mut self, unit: String, exp: i32) -> Result<(), V3Error> {
        let l: usize = unit.chars().count();
        if l == 0 {
            return Ok(());
        }

        // first match it against known unique strings
        match unit.as_str() {
            "mph" => {
                if exp != 1 && exp != -1 {
                    return Err(V3Error::ParsingError("[_parse_units] MPH exponent".into()));
                }
                self.v_length = Some(UnitLength::Mile);
                self.exp[LENGTH_INDEX] = exp;
                self.v_time = Some(UnitTime::Hour);
                self.exp[TIME_INDEX] = -exp;
                self.unit_map = LENGTH_MAP | TIME_MAP;
                return Ok(());
            }
            "kph" => {
                if exp != 1 && exp != -1 {
                    return Err(V3Error::ParsingError("[_parse_units] KPH exponent".into()));
                }
                self.v_length = Some(UnitLength::Meter(Metric::Kilo));
                self.exp[LENGTH_INDEX] = exp;
                self.v_time = Some(UnitTime::Hour);
                self.exp[TIME_INDEX] = -exp;
                self.unit_map |= LENGTH_MAP | TIME_MAP;
                return Ok(());
            }
            "mmHg" => {
                self.v_pressure = Some(UnitPressure::Hgmm);
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
                return Ok(());
            }
            "cmHg" => {
                self.v_pressure = Some(UnitPressure::Hgcm);
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
                return Ok(());
            }
            "inHg" => {
                self.v_pressure = Some(UnitPressure::Hgin);
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
                return Ok(());
            }
            "byte" | "bytes" => {
                self.v_information = Some(UnitInformation::Byte(Metric::None));
                self.exp[INFORMATION_INDEX] = exp;
                self.unit_map |= INFORMATION_MAP;
                return Ok(());
            }
            "bit" | "bits" => {
                self.v_information = Some(UnitInformation::Bit(Metric::None));
                self.exp[INFORMATION_INDEX] = exp;
                self.unit_map |= INFORMATION_MAP;
                return Ok(());
            }
            "radian" | "radians" => {
                self.v_angle = Some(UnitAngle::Radian(Metric::None));
                self.exp[ANGLE_INDEX] = exp;
                self.unit_map |= ANGLE_MAP;
                return Ok(());
            }
            "angstrom" | "angstroms" => {
                self.v_length = Some(UnitLength::Angstrom);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "inches" | "inch" | "in" => {
                self.v_length = Some(UnitLength::Inch);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "foot" | "feet" | "ft" => {
                self.v_length = Some(UnitLength::Foot);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "yard" | "yards" | "yd" | "yds" => {
                self.v_length = Some(UnitLength::Yard);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "mile" | "miles" => {
                self.v_length = Some(UnitLength::Mile);
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "atm" | "ATM" => {
                self.v_pressure = Some(UnitPressure::Atm);
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
                return Ok(());
            }
            "psi" | "PSI" => {
                self.v_pressure = Some(UnitPressure::Psi);
                self.exp[PRESSURE_INDEX] = exp;
                self.unit_map |= PRESSURE_MAP;
                return Ok(());
            }
            "f" | "°f" | "°F" => {
                self.v_temperature = Some(UnitTemperature::Fahrenheit);
                self.exp[TEMPERATURE_INDEX] = exp;
                self.unit_map |= TEMPERATURE_MAP;
                return Ok(());
            }
            "°c" | "°C" => {
                self.v_temperature = Some(UnitTemperature::Celsius(Metric::None));
                self.exp[TEMPERATURE_INDEX] = exp;
                self.unit_map |= TEMPERATURE_MAP;
                return Ok(());
            }
            "footpound" | "footpounds" | "ftlb" | "ftlbs" => {
                self.v_energy = Some(UnitEnergy::FootPound);
                self.exp[ENERGY_INDEX] = exp;
                self.unit_map |= ENERGY_MAP;
                return Ok(());
            }
            "poundforce" | "poundsforce" | "lbfr" | "lbsfr" => {
                self.v_force = Some(UnitForce::PoundForce);
                self.exp[FORCE_INDEX] = exp;
                self.unit_map |= FORCE_MAP;
                return Ok(());
            }
            "ounce" | "ounces" | "oz" => {
                self.v_mass = Some(UnitMass::Ounce);
                self.exp[MASS_INDEX] = exp;
                self.unit_map |= MASS_MAP;
                return Ok(());
            }
            "grain" | "grains" | "gr" => {
                self.v_mass = Some(UnitMass::Grain);
                self.exp[MASS_INDEX] = exp;
                self.unit_map |= MASS_MAP;
                return Ok(());
            }
            "pounds" | "lbs" | "lb" => {
                self.v_mass = Some(UnitMass::Pound);
                self.exp[MASS_INDEX] = exp;
                self.unit_map |= MASS_MAP;
                return Ok(());
            }
            "moa" | "MOA" => {
                self.v_angle = Some(UnitAngle::Moa);
                self.exp[ANGLE_INDEX] = exp;
                self.unit_map |= ANGLE_MAP;
                return Ok(());
            }
            "rads" | "Rads" => {
                self.v_ab_dose = Some(UnitAbsorbedDose::Rad);
                self.exp[ABSORBED_DOSE_INDEX] = exp;
                self.unit_map |= ABSORBED_DOSE_MAP;
                return Ok(());
            }
            "rem" | "Rem" => {
                self.v_radioactivity_exposure = Some(UnitRadioactivityExposure::Rem);
                self.exp[RADIOACTIVITY_EXPOSURE_INDEX] = exp;
                self.unit_map |= RADIOACTIVITY_EXPOSURE_MAP;
                return Ok(());
            }
            "mil" | "MIL" | "mils" => {
                self.v_angle = Some(UnitAngle::Radian(Metric::Milli));
                self.exp[ANGLE_INDEX] = exp;
                self.unit_map |= ANGLE_MAP;
                return Ok(());
            }
            "degrees" | "degree" | "°" => {
                self.v_angle = Some(UnitAngle::Degree);
                self.exp[ANGLE_INDEX] = exp;
                self.unit_map |= ANGLE_MAP;
                return Ok(());
            }
            "farad" | "farads" => {
                self.v_capacitance = Some(UnitElectricCapacitance::Farad(Metric::None));
                self.exp[CAPACITANCE_INDEX] = exp;
                self.unit_map |= CAPACITANCE_MAP;
                return Ok(());
            }
            "micron" | "microns" => {
                self.v_length = Some(UnitLength::Meter(Metric::Micro));
                self.exp[LENGTH_INDEX] = exp;
                self.unit_map |= LENGTH_MAP;
                return Ok(());
            }
            "min" | "minute" | "minutes" => {
                self.v_time = Some(UnitTime::Minute);
                self.exp[TIME_INDEX] = exp;
                self.unit_map |= TIME_MAP;
                return Ok(());
            }
            "h" | "hr" | "hour" | "hours" => {
                self.v_time = Some(UnitTime::Hour);
                self.exp[TIME_INDEX] = exp;
                self.unit_map |= TIME_MAP;
                return Ok(());
            }
            "d" | "day" | "days" => {
                self.v_time = Some(UnitTime::Day);
                self.exp[TIME_INDEX] = exp;
                self.unit_map |= TIME_MAP;
                return Ok(());
            }
            _ => {
                // do nothing
            }
        }

        if l == 1 {
            self._get_single_letter(unit.chars().next().unwrap(), exp, Metric::None)?;
        } else if l == 2 {
            self._get_double_letter(&unit, exp, Metric::None)?;
        } else if l == 3 {
            self._get_triple_letter(&unit, exp, Metric::None)?;
        } else if l == 4 {
            self._get_quadruple_letter(&unit, exp, Metric::None)?;
        } else if l == 5 {
            self._get_pentuple_letter(&unit, exp, Metric::None)?;
        } else if l == 6 {
            self._get_sextuple_letter(&unit, exp)?;
        } else {
            return Err(V3Error::UnsupportedUnit(format!(
                "[_parse_units] Unit {} exceeds parsing bounds",
                unit
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod parse_testing {
    use crate::{
        consts::{
            ABSORBED_DOSE_INDEX, ABSORBED_DOSE_MAP, ANGLE_INDEX, ANGLE_MAP, CAPACITANCE_INDEX,
            CAPACITANCE_MAP, CATALYTIC_ACTIVITY_INDEX, CATALYTIC_ACTIVITY_MAP,
            ELECTRIC_CHARGE_INDEX, ELECTRIC_CHARGE_MAP, ELECTRIC_CONDUCTANCE_INDEX,
            ELECTRIC_CONDUCTANCE_MAP, ELECTRIC_CURRENT_INDEX, ELECTRIC_CURRENT_MAP,
            ELECTRIC_POTENTIAL_INDEX, ELECTRIC_POTENTIAL_MAP, ENERGY_INDEX, ENERGY_MAP,
            FORCE_INDEX, FORCE_MAP, FREQUENCY_INDEX, FREQUENCY_MAP, ILLUMINANCE_INDEX,
            ILLUMINANCE_MAP, INDUCTANCE_INDEX, INDUCTANCE_MAP, INFORMATION_INDEX, INFORMATION_MAP,
            LENGTH_INDEX, LENGTH_MAP, LUMINOUS_FLUX_INDEX, LUMINOUS_FLUX_MAP,
            LUMINOUS_INTENSITY_INDEX, LUMINOUS_INTENSITY_MAP, MAGNETIC_FLUX_DENSITY_INDEX,
            MAGNETIC_FLUX_DENSITY_MAP, MAGNETIC_FLUX_INDEX, MAGNETIC_FLUX_MAP, MASS_INDEX,
            MASS_MAP, POWER_INDEX, POWER_MAP, PRESSURE_INDEX, PRESSURE_MAP,
            RADIOACTIVITY_EXPOSURE_INDEX, RADIOACTIVITY_EXPOSURE_MAP, RADIOACTIVITY_INDEX,
            RADIOACTIVITY_MAP, RESISTANCE_INDEX, RESISTANCE_MAP, SOLID_ANGLE_INDEX,
            SOLID_ANGLE_MAP, SOUND_INDEX, SOUND_MAP, SUBSTANCE_INDEX, SUBSTANCE_MAP,
            TEMPERATURE_INDEX, TEMPERATURE_MAP, TIME_INDEX, TIME_MAP, VOLUME_INDEX, VOLUME_MAP,
        },
        units::{
            Metric, UnitAngle, UnitElectricCapacitance, UnitEnergy, UnitForce, UnitFrequency,
            UnitInformation, UnitLength, UnitMass, UnitPressure, UnitTemperature, UnitTime,
        },
        Value,
    };

    const TEST_METRIC: [(Metric, &str); 26] = [
        (Metric::Quetta, "Q"),
        (Metric::Ronna, "R"),
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
        (Metric::Ronto, "r"),
        (Metric::Quecto, "q"),
    ];

    #[test]
    #[should_panic]
    fn unique_names_mph_exp() {
        let _ = Value::new(1.5, "mph^2").unwrap();
    }

    #[test]
    #[should_panic]
    fn unique_names_kph_exp() {
        let _ = Value::new(1.5, "kph^2").unwrap();
    }

    #[test]
    #[should_panic]
    fn multiple_divs() {
        let _ = Value::new(1.5, "ml/g/s").unwrap();
    }

    #[test]
    fn empty_units() {
        let v = Value::new(1.5, "").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, 0);
    }

    #[test]
    fn extra_metric() {
        let _ = Value::new(1.5, "dadam").is_err();
        let _ = Value::new(1.5, "kMm").is_err();
    }

    #[test]
    fn bad_exp() {
        let _ = Value::new(1.5, "m^r").is_err();
        let _ = Value::new(1.5, "m^-r").is_err();
        let _ = Value::new(1.5, "1/m^r").is_err();
    }

    #[test]
    fn div_special_case() {
        assert!(Value::new(1.5, "/m").is_ok());
    }

    #[test]
    fn mul_special_case() {
        assert!(Value::new(1.5, "m*s").is_ok());
        assert!(Value::new(1.5, "m*(s/l)").is_ok());
        assert!(Value::new(1.5, "(m*s)/l").is_ok());
    }

    #[test]
    fn char_lengths() {
        assert!(Value::new(1.5, " \t\n\t          ").is_ok());
        assert!(Value::new(1.5, "Q").is_err());
        assert!(Value::new(1.5, "Qo").is_err());
        assert!(Value::new(1.5, "Qoo").is_err());
        assert!(Value::new(1.5, "Qooo").is_err());
        assert!(Value::new(1.5, "Qoooo").is_err());
        assert!(Value::new(1.5, "Qooooo").is_err());
        assert!(Value::new(1.5, "Qoooooo").is_err());

        assert!(Value::new(1.5, "o").is_err());
        assert!(Value::new(1.5, "oo").is_err());
        assert!(Value::new(1.5, "ooo").is_err());
        assert!(Value::new(1.5, "oooo").is_err());
        assert!(Value::new(1.5, "ooooo").is_err());
        assert!(Value::new(1.5, "oooooo").is_err());
        assert!(Value::new(1.5, "ooooooo").is_err());
    }

    #[test]
    fn letters_4_len() {
        let units = [
            ("torr", PRESSURE_MAP, PRESSURE_INDEX),
            ("bits", INFORMATION_MAP, INFORMATION_INDEX),
        ];

        for u in units {
            for m in TEST_METRIC {
                let v = Value::new(1.5, &format!("{}{}", m.1, u.0)).unwrap();
                assert_eq!(v, 1.5);
                assert_eq!(v.unit_map, u.1);
                assert_eq!(v.exp[u.2], 1);
            }
        }
    }

    #[test]
    fn letters_3_len() {
        let units = [
            ("mol", SUBSTANCE_MAP, SUBSTANCE_INDEX),
            ("kat", CATALYTIC_ACTIVITY_MAP, CATALYTIC_ACTIVITY_INDEX),
            ("rad", ANGLE_MAP, ANGLE_INDEX),
            ("bar", PRESSURE_MAP, PRESSURE_INDEX),
            ("cal", ENERGY_MAP, ENERGY_INDEX),
            ("lyr", LENGTH_MAP, LENGTH_INDEX),
        ];

        for u in units {
            for m in TEST_METRIC {
                let v = Value::new(1.5, &format!("{}{}", m.1, u.0)).unwrap();
                assert_eq!(v, 1.5);
                assert_eq!(v.unit_map, u.1);
                assert_eq!(v.exp[u.2], 1);
            }
        }
    }

    #[test]
    fn letters_2_len() {
        let metric_units = [
            ("Hz", FREQUENCY_MAP, FREQUENCY_INDEX),
            ("Pa", PRESSURE_MAP, PRESSURE_INDEX),
            ("Wb", MAGNETIC_FLUX_MAP, MAGNETIC_FLUX_INDEX),
            ("lm", LUMINOUS_FLUX_MAP, LUMINOUS_FLUX_INDEX),
            ("lx", ILLUMINANCE_MAP, ILLUMINANCE_INDEX),
            ("Bq", RADIOACTIVITY_MAP, RADIOACTIVITY_INDEX),
            (
                "Sv",
                RADIOACTIVITY_EXPOSURE_MAP,
                RADIOACTIVITY_EXPOSURE_INDEX,
            ),
            ("cd", LUMINOUS_INTENSITY_MAP, LUMINOUS_INTENSITY_INDEX),
            ("pc", LENGTH_MAP, LENGTH_INDEX),
            ("Gy", ABSORBED_DOSE_MAP, ABSORBED_DOSE_INDEX),
            ("sr", SOLID_ANGLE_MAP, SOLID_ANGLE_INDEX),
            ("eV", ENERGY_MAP, ENERGY_INDEX),
        ];

        let non_metric_units = [
            ("au", LENGTH_MAP, LENGTH_INDEX),
            ("AU", LENGTH_MAP, LENGTH_INDEX),
            ("Ci", RADIOACTIVITY_MAP, RADIOACTIVITY_INDEX),
            ("hp", POWER_MAP, POWER_INDEX),
        ];

        for u in metric_units {
            for m in TEST_METRIC {
                let v = Value::new(1.5, &format!("{}{}", m.1, u.0)).unwrap();
                assert_eq!(v, 1.5);
                assert_eq!(v.unit_map, u.1);
                assert_eq!(v.exp[u.2], 1);
            }
        }

        for u in non_metric_units {
            let v = Value::new(1.5, u.0).unwrap();
            assert_eq!(v, 1.5);
            assert_eq!(v.unit_map, u.1);
            assert_eq!(v.exp[u.2], 1);
            for m in TEST_METRIC {
                let _ = Value::new(1.5, &format!("{}{}", m.1, u.0)).is_err();
            }
        }
    }

    #[test]
    fn letters_1_len() {
        let metric_units = [
            ("m", LENGTH_MAP, LENGTH_INDEX),
            ("g", MASS_MAP, MASS_INDEX),
            ("s", TIME_MAP, TIME_INDEX),
            ("A", ELECTRIC_CURRENT_MAP, ELECTRIC_CURRENT_INDEX),
            ("J", ENERGY_MAP, ENERGY_INDEX),
            ("W", POWER_MAP, POWER_INDEX),
            ("C", ELECTRIC_CHARGE_MAP, ELECTRIC_CHARGE_INDEX),
            ("F", CAPACITANCE_MAP, CAPACITANCE_INDEX),
            ("Ω", RESISTANCE_MAP, RESISTANCE_INDEX),
            ("S", ELECTRIC_CONDUCTANCE_MAP, ELECTRIC_CONDUCTANCE_INDEX),
            ("T", MAGNETIC_FLUX_DENSITY_MAP, MAGNETIC_FLUX_DENSITY_INDEX),
            ("N", FORCE_MAP, FORCE_INDEX),
            ("K", TEMPERATURE_MAP, TEMPERATURE_INDEX),
            ("H", INDUCTANCE_MAP, INDUCTANCE_INDEX),
            ("V", ELECTRIC_POTENTIAL_MAP, ELECTRIC_POTENTIAL_INDEX),
            ("B", SOUND_MAP, SOUND_INDEX),
            ("b", INFORMATION_MAP, INFORMATION_INDEX),
            ("l", VOLUME_MAP, VOLUME_INDEX),
        ];

        let non_metric_units = [
            ("Å", LENGTH_MAP, LENGTH_INDEX),
            ("R", ABSORBED_DOSE_MAP, ABSORBED_DOSE_INDEX),
        ];

        for u in metric_units {
            for m in TEST_METRIC {
                let v = Value::new(1.5, &format!("{}{}", m.1, u.0)).unwrap();
                assert_eq!(v, 1.5);
                assert_eq!(v.unit_map, u.1);
                assert_eq!(v.exp[u.2], 1);
            }
        }

        for u in non_metric_units {
            let v = Value::new(1.5, u.0).unwrap();
            assert_eq!(v, 1.5);
            assert_eq!(v.unit_map, u.1);
            assert_eq!(v.exp[u.2], 1);
            for m in TEST_METRIC {
                let _ = Value::new(1.5, &format!("{}{}", m.1, u.0)).is_err();
            }
        }
    }

    #[test]
    fn unique_names_mph() {
        // MPH
        let v = Value::new(1.5, "mph").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP | TIME_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Mile));
        assert_eq!(v.v_time, Some(UnitTime::Hour));
        assert_eq!(v.exp[LENGTH_INDEX], 1);
        assert_eq!(v.exp[TIME_INDEX], -1);

        let v = Value::new(1.5, "1/mph").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP | TIME_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Mile));
        assert_eq!(v.v_time, Some(UnitTime::Hour));
        assert_eq!(v.exp[LENGTH_INDEX], -1);
        assert_eq!(v.exp[TIME_INDEX], 1);
    }

    #[test]
    fn unique_names_kph() {
        // kph
        let v = Value::new(1.5, "kph").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP | TIME_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Meter(Metric::Kilo)));
        assert_eq!(v.v_time, Some(UnitTime::Hour));
        assert_eq!(v.exp[LENGTH_INDEX], 1);
        assert_eq!(v.exp[TIME_INDEX], -1);

        let v = Value::new(1.5, "1/kph").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP | TIME_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Meter(Metric::Kilo)));
        assert_eq!(v.v_time, Some(UnitTime::Hour));
        assert_eq!(v.exp[LENGTH_INDEX], -1);
        assert_eq!(v.exp[TIME_INDEX], 1);
    }

    #[test]
    fn unique_names_mmhg() {
        // mmHg
        let v = Value::new(1.5, "mmHg").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Hgmm));
        assert_eq!(v.exp[PRESSURE_INDEX], 1);

        let v = Value::new(1.5, "(1)/(mmHg)").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Hgmm));
        assert_eq!(v.exp[PRESSURE_INDEX], -1);
    }

    #[test]
    fn unique_names_cmhg() {
        // mmHg
        let v = Value::new(1.5, "cmHg").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Hgcm));
        assert_eq!(v.exp[PRESSURE_INDEX], 1);

        let v = Value::new(1.5, "(1)/(cmHg)").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Hgcm));
        assert_eq!(v.exp[PRESSURE_INDEX], -1);
    }

    #[test]
    fn unique_names_inhg() {
        // inHg
        let v = Value::new(1.5, "inHg").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Hgin));
        assert_eq!(v.exp[PRESSURE_INDEX], 1);

        let v = Value::new(1.5, "(1)/(inHg)").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Hgin));
        assert_eq!(v.exp[PRESSURE_INDEX], -1);
    }

    #[test]
    fn unique_names_bytes() {
        // bytes
        let v = Value::new(1.5, "bytes").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, INFORMATION_MAP);
        assert_eq!(v.v_information, Some(UnitInformation::Byte(Metric::None)));
        assert_eq!(v.exp[INFORMATION_INDEX], 1);

        let v = Value::new(1.5, "byte").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, INFORMATION_MAP);
        assert_eq!(v.v_information, Some(UnitInformation::Byte(Metric::None)));
        assert_eq!(v.exp[INFORMATION_INDEX], 1);

        let v = Value::new(1.5, "1/bytes").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, INFORMATION_MAP);
        assert_eq!(v.v_information, Some(UnitInformation::Byte(Metric::None)));
        assert_eq!(v.exp[INFORMATION_INDEX], -1);

        let v = Value::new(1.5, "1/byte").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, INFORMATION_MAP);
        assert_eq!(v.v_information, Some(UnitInformation::Byte(Metric::None)));
        assert_eq!(v.exp[INFORMATION_INDEX], -1);
    }

    #[test]
    fn unique_names_bits() {
        // bits
        let v = Value::new(1.5, "bits").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, INFORMATION_MAP);
        assert_eq!(v.v_information, Some(UnitInformation::Bit(Metric::None)));
        assert_eq!(v.exp[INFORMATION_INDEX], 1);

        let v = Value::new(1.5, "bit").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, INFORMATION_MAP);
        assert_eq!(v.v_information, Some(UnitInformation::Bit(Metric::None)));
        assert_eq!(v.exp[INFORMATION_INDEX], 1);

        let v = Value::new(1.5, "1/bits").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, INFORMATION_MAP);
        assert_eq!(v.v_information, Some(UnitInformation::Bit(Metric::None)));
        assert_eq!(v.exp[INFORMATION_INDEX], -1);

        let v = Value::new(1.5, "1/bit").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, INFORMATION_MAP);
        assert_eq!(v.v_information, Some(UnitInformation::Bit(Metric::None)));
        assert_eq!(v.exp[INFORMATION_INDEX], -1);
    }

    #[test]
    fn unique_names_radians() {
        // radians
        let v = Value::new(1.5, "radian").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Radian(Metric::None)));
        assert_eq!(v.exp[ANGLE_INDEX], 1);

        let v = Value::new(1.5, "radians").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Radian(Metric::None)));
        assert_eq!(v.exp[ANGLE_INDEX], 1);

        let v = Value::new(1.5, "1/radian").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Radian(Metric::None)));
        assert_eq!(v.exp[ANGLE_INDEX], -1);

        let v = Value::new(1.5, "1/radians").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Radian(Metric::None)));
        assert_eq!(v.exp[ANGLE_INDEX], -1);
    }

    #[test]
    fn unique_names_angstrom() {
        // angstroms
        let v = Value::new(1.5, "angstrom").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Angstrom));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "angstroms").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Angstrom));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "1/angstrom").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Angstrom));
        assert_eq!(v.exp[LENGTH_INDEX], -1);

        let v = Value::new(1.5, "1/angstroms").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Angstrom));
        assert_eq!(v.exp[LENGTH_INDEX], -1);
    }

    #[test]
    fn unique_names_inch() {
        // inches
        let v = Value::new(1.5, "inch").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Inch));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "inches").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Inch));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "in").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Inch));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "1/inch").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Inch));
        assert_eq!(v.exp[LENGTH_INDEX], -1);

        let v = Value::new(1.5, "1/inches").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Inch));
        assert_eq!(v.exp[LENGTH_INDEX], -1);

        let v = Value::new(1.5, "1/in").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Inch));
        assert_eq!(v.exp[LENGTH_INDEX], -1);
    }

    #[test]
    fn unique_names_foot() {
        // feet
        let v = Value::new(1.5, "feet").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Foot));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "foot").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Foot));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "ft").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Foot));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "1/feet").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Foot));
        assert_eq!(v.exp[LENGTH_INDEX], -1);

        let v = Value::new(1.5, "1/foot").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Foot));
        assert_eq!(v.exp[LENGTH_INDEX], -1);

        let v = Value::new(1.5, "1/ft").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Foot));
        assert_eq!(v.exp[LENGTH_INDEX], -1);
    }

    #[test]
    fn unique_names_yards() {
        // yards
        let v = Value::new(1.5, "yard").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Yard));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "yards").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Yard));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "yds").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Yard));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "yd").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Yard));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "1/yard").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Yard));
        assert_eq!(v.exp[LENGTH_INDEX], -1);

        let v = Value::new(1.5, "1/yards").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Yard));
        assert_eq!(v.exp[LENGTH_INDEX], -1);

        let v = Value::new(1.5, "1/yds").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Yard));
        assert_eq!(v.exp[LENGTH_INDEX], -1);

        let v = Value::new(1.5, "1/yd").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Yard));
        assert_eq!(v.exp[LENGTH_INDEX], -1);
    }

    #[test]
    fn unique_names_mile() {
        // Miles
        let v = Value::new(1.5, "mile").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Mile));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "miles").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Mile));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "1/mile").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Mile));
        assert_eq!(v.exp[LENGTH_INDEX], -1);

        let v = Value::new(1.5, "1/miles").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Mile));
        assert_eq!(v.exp[LENGTH_INDEX], -1);
    }

    #[test]
    fn unique_names_atm() {
        // atmospheres
        let v = Value::new(1.5, "atm").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Atm));
        assert_eq!(v.exp[PRESSURE_INDEX], 1);

        let v = Value::new(1.5, "ATM").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Atm));
        assert_eq!(v.exp[PRESSURE_INDEX], 1);

        let v = Value::new(1.5, "1/atm").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Atm));
        assert_eq!(v.exp[PRESSURE_INDEX], -1);

        let v = Value::new(1.5, "1/ATM").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Atm));
        assert_eq!(v.exp[PRESSURE_INDEX], -1);
    }

    #[test]
    fn unique_names_psi() {
        // pounds per square inch
        let v = Value::new(1.5, "psi").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Psi));
        assert_eq!(v.exp[PRESSURE_INDEX], 1);

        let v = Value::new(1.5, "PSI").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Psi));
        assert_eq!(v.exp[PRESSURE_INDEX], 1);

        let v = Value::new(1.5, "1/psi").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Psi));
        assert_eq!(v.exp[PRESSURE_INDEX], -1);

        let v = Value::new(1.5, "1/PSI").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, PRESSURE_MAP);
        assert_eq!(v.v_pressure, Some(UnitPressure::Psi));
        assert_eq!(v.exp[PRESSURE_INDEX], -1);
    }

    #[test]
    fn unique_names_fahrenheit() {
        // f temp
        let v = Value::new(1.5, "°F").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(v.v_temperature, Some(UnitTemperature::Fahrenheit));
        assert_eq!(v.exp[TEMPERATURE_INDEX], 1);

        let v = Value::new(1.5, "°f").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(v.v_temperature, Some(UnitTemperature::Fahrenheit));
        assert_eq!(v.exp[TEMPERATURE_INDEX], 1);

        let v = Value::new(1.5, "f").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(v.v_temperature, Some(UnitTemperature::Fahrenheit));
        assert_eq!(v.exp[TEMPERATURE_INDEX], 1);

        let v = Value::new(1.5, "1/°F").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(v.v_temperature, Some(UnitTemperature::Fahrenheit));
        assert_eq!(v.exp[TEMPERATURE_INDEX], -1);

        let v = Value::new(1.5, "1/°f").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(v.v_temperature, Some(UnitTemperature::Fahrenheit));
        assert_eq!(v.exp[TEMPERATURE_INDEX], -1);

        let v = Value::new(1.5, "1/f").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(v.v_temperature, Some(UnitTemperature::Fahrenheit));
        assert_eq!(v.exp[TEMPERATURE_INDEX], -1);
    }

    #[test]
    fn unique_names_celsius() {
        // c temp
        let v = Value::new(1.5, "°C").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(
            v.v_temperature,
            Some(UnitTemperature::Celsius(Metric::None))
        );
        assert_eq!(v.exp[TEMPERATURE_INDEX], 1);

        let v = Value::new(1.5, "°c").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(
            v.v_temperature,
            Some(UnitTemperature::Celsius(Metric::None))
        );
        assert_eq!(v.exp[TEMPERATURE_INDEX], 1);

        let v = Value::new(1.5, "c").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(
            v.v_temperature,
            Some(UnitTemperature::Celsius(Metric::None))
        );
        assert_eq!(v.exp[TEMPERATURE_INDEX], 1);

        let v = Value::new(1.5, "1/°C").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(
            v.v_temperature,
            Some(UnitTemperature::Celsius(Metric::None))
        );
        assert_eq!(v.exp[TEMPERATURE_INDEX], -1);

        let v = Value::new(
            1.5,
            "      1                /                         °c    ",
        )
        .unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(
            v.v_temperature,
            Some(UnitTemperature::Celsius(Metric::None))
        );
        assert_eq!(v.exp[TEMPERATURE_INDEX], -1);

        let v = Value::new(1.5, "1        /c").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(
            v.v_temperature,
            Some(UnitTemperature::Celsius(Metric::None))
        );
        assert_eq!(v.exp[TEMPERATURE_INDEX], -1);

        let v = Value::new(1.5, "kc").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(
            v.v_temperature,
            Some(UnitTemperature::Celsius(Metric::Kilo))
        );
        assert_eq!(v.exp[TEMPERATURE_INDEX], 1);

        let v = Value::new(1.5, "°dac").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(
            v.v_temperature,
            Some(UnitTemperature::Celsius(Metric::Deca))
        );
        assert_eq!(v.exp[TEMPERATURE_INDEX], 1);

        let v = Value::new(1.5, "°kc").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TEMPERATURE_MAP);
        assert_eq!(
            v.v_temperature,
            Some(UnitTemperature::Celsius(Metric::Kilo))
        );
        assert_eq!(v.exp[TEMPERATURE_INDEX], 1);
    }

    #[test]
    fn unique_names_footpounds() {
        // foot pounds
        let v = Value::new(1.5, "footpound").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ENERGY_MAP);
        assert_eq!(v.v_energy, Some(UnitEnergy::FootPound));
        assert_eq!(v.exp[ENERGY_INDEX], 1);

        let v = Value::new(1.5, "footpounds").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ENERGY_MAP);
        assert_eq!(v.v_energy, Some(UnitEnergy::FootPound));
        assert_eq!(v.exp[ENERGY_INDEX], 1);

        let v = Value::new(1.5, "ftlb").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ENERGY_MAP);
        assert_eq!(v.v_energy, Some(UnitEnergy::FootPound));
        assert_eq!(v.exp[ENERGY_INDEX], 1);

        let v = Value::new(1.5, "ftlbs").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ENERGY_MAP);
        assert_eq!(v.v_energy, Some(UnitEnergy::FootPound));
        assert_eq!(v.exp[ENERGY_INDEX], 1);

        let v = Value::new(1.5, "1/footpound").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ENERGY_MAP);
        assert_eq!(v.v_energy, Some(UnitEnergy::FootPound));
        assert_eq!(v.exp[ENERGY_INDEX], -1);

        let v = Value::new(1.5, "1/ footpounds").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ENERGY_MAP);
        assert_eq!(v.v_energy, Some(UnitEnergy::FootPound));
        assert_eq!(v.exp[ENERGY_INDEX], -1);

        let v = Value::new(1.5, "1/ftlb").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ENERGY_MAP);
        assert_eq!(v.v_energy, Some(UnitEnergy::FootPound));
        assert_eq!(v.exp[ENERGY_INDEX], -1);

        let v = Value::new(1.5, "1 /    ftlbs").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ENERGY_MAP);
        assert_eq!(v.v_energy, Some(UnitEnergy::FootPound));
        assert_eq!(v.exp[ENERGY_INDEX], -1);
    }

    #[test]
    fn unique_names_poundsforce() {
        // pounds force
        let v = Value::new(1.5, "poundforce").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, FORCE_MAP);
        assert_eq!(v.v_force, Some(UnitForce::PoundForce));
        assert_eq!(v.exp[FORCE_INDEX], 1);

        let v = Value::new(1.5, "poundsforce").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, FORCE_MAP);
        assert_eq!(v.v_force, Some(UnitForce::PoundForce));
        assert_eq!(v.exp[FORCE_INDEX], 1);

        let v = Value::new(1.5, "lbfr").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, FORCE_MAP);
        assert_eq!(v.v_force, Some(UnitForce::PoundForce));
        assert_eq!(v.exp[FORCE_INDEX], 1);

        let v = Value::new(1.5, "lbsfr").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, FORCE_MAP);
        assert_eq!(v.v_force, Some(UnitForce::PoundForce));
        assert_eq!(v.exp[FORCE_INDEX], 1);

        let v = Value::new(1.5, "1/poundforce").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, FORCE_MAP);
        assert_eq!(v.v_force, Some(UnitForce::PoundForce));
        assert_eq!(v.exp[FORCE_INDEX], -1);

        let v = Value::new(1.5, "1/poundsforce").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, FORCE_MAP);
        assert_eq!(v.v_force, Some(UnitForce::PoundForce));
        assert_eq!(v.exp[FORCE_INDEX], -1);

        let v = Value::new(1.5, "1/lbfr").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, FORCE_MAP);
        assert_eq!(v.v_force, Some(UnitForce::PoundForce));
        assert_eq!(v.exp[FORCE_INDEX], -1);

        let v = Value::new(1.5, "1/lbsfr").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, FORCE_MAP);
        assert_eq!(v.v_force, Some(UnitForce::PoundForce));
        assert_eq!(v.exp[FORCE_INDEX], -1);
    }

    #[test]
    fn unique_names_ounce() {
        let v = Value::new(1.5, "ounce").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Ounce));
        assert_eq!(v.exp[MASS_INDEX], 1);

        let v = Value::new(1.5, "ounces").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Ounce));
        assert_eq!(v.exp[MASS_INDEX], 1);

        let v = Value::new(1.5, "oz").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Ounce));
        assert_eq!(v.exp[MASS_INDEX], 1);

        let v = Value::new(1.5, "1/ounce").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Ounce));
        assert_eq!(v.exp[MASS_INDEX], -1);

        let v = Value::new(1.5, "1/ounces").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Ounce));
        assert_eq!(v.exp[MASS_INDEX], -1);

        let v = Value::new(1.5, "1/oz").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Ounce));
        assert_eq!(v.exp[MASS_INDEX], -1);
    }

    #[test]
    fn unique_names_pound() {
        let v = Value::new(1.5, "pounds").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Pound));
        assert_eq!(v.exp[MASS_INDEX], 1);

        let v = Value::new(1.5, "lb").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Pound));
        assert_eq!(v.exp[MASS_INDEX], 1);

        let v = Value::new(1.5, "1/pounds").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Pound));
        assert_eq!(v.exp[MASS_INDEX], -1);

        let v = Value::new(1.5, "1/lb").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Pound));
        assert_eq!(v.exp[MASS_INDEX], -1);
    }

    #[test]
    fn unique_names_grain() {
        let v = Value::new(1.5, "grain").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Grain));
        assert_eq!(v.exp[MASS_INDEX], 1);

        let v = Value::new(1.5, "grains").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Grain));
        assert_eq!(v.exp[MASS_INDEX], 1);

        let v = Value::new(1.5, "gr").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Grain));
        assert_eq!(v.exp[MASS_INDEX], 1);

        let v = Value::new(1.5, "1/gr").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Grain));
        assert_eq!(v.exp[MASS_INDEX], -1);

        let v = Value::new(1.5, "1/grain").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Grain));
        assert_eq!(v.exp[MASS_INDEX], -1);

        let v = Value::new(1.5, "1/grains").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, MASS_MAP);
        assert_eq!(v.v_mass, Some(UnitMass::Grain));
        assert_eq!(v.exp[MASS_INDEX], -1);
    }

    #[test]
    fn unique_names_moa() {
        let v = Value::new(1.5, "moa").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Moa));
        assert_eq!(v.exp[ANGLE_INDEX], 1);

        let v = Value::new(1.5, "MOA").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Moa));
        assert_eq!(v.exp[ANGLE_INDEX], 1);

        let v = Value::new(1.5, "1/moa").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Moa));
        assert_eq!(v.exp[ANGLE_INDEX], -1);

        let v = Value::new(1.5, "1/MOA").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Moa));
        assert_eq!(v.exp[ANGLE_INDEX], -1);
    }

    #[test]
    fn unique_names_mils() {
        let v = Value::new(1.5, "mil").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Radian(Metric::Milli)));
        assert_eq!(v.exp[ANGLE_INDEX], 1);

        let v = Value::new(1.5, "MIL").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Radian(Metric::Milli)));
        assert_eq!(v.exp[ANGLE_INDEX], 1);

        let v = Value::new(1.5, "mils").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Radian(Metric::Milli)));
        assert_eq!(v.exp[ANGLE_INDEX], 1);

        let v = Value::new(1.5, "1/mil").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Radian(Metric::Milli)));
        assert_eq!(v.exp[ANGLE_INDEX], -1);

        let v = Value::new(1.5, "1/MIL").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Radian(Metric::Milli)));
        assert_eq!(v.exp[ANGLE_INDEX], -1);

        let v = Value::new(1.5, "1/mils").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Radian(Metric::Milli)));
        assert_eq!(v.exp[ANGLE_INDEX], -1);
    }

    #[test]
    fn unique_names_degrees() {
        let v = Value::new(1.5, "degree").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Degree));
        assert_eq!(v.exp[ANGLE_INDEX], 1);

        let v = Value::new(1.5, "degrees").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Degree));
        assert_eq!(v.exp[ANGLE_INDEX], 1);

        let v = Value::new(1.5, "°").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Degree));
        assert_eq!(v.exp[ANGLE_INDEX], 1);

        let v = Value::new(1.5, "1/degree").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Degree));
        assert_eq!(v.exp[ANGLE_INDEX], -1);

        let v = Value::new(1.5, "1/degrees").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Degree));
        assert_eq!(v.exp[ANGLE_INDEX], -1);

        let v = Value::new(1.5, "1/°").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ANGLE_MAP);
        assert_eq!(v.v_angle, Some(UnitAngle::Degree));
        assert_eq!(v.exp[ANGLE_INDEX], -1);
    }

    #[test]
    fn unique_names_farad() {
        let v = Value::new(1.5, "farad").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, CAPACITANCE_MAP);
        assert_eq!(
            v.v_capacitance,
            Some(UnitElectricCapacitance::Farad(Metric::None))
        );
        assert_eq!(v.exp[CAPACITANCE_INDEX], 1);

        let v = Value::new(1.5, "farads").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, CAPACITANCE_MAP);
        assert_eq!(
            v.v_capacitance,
            Some(UnitElectricCapacitance::Farad(Metric::None))
        );
        assert_eq!(v.exp[CAPACITANCE_INDEX], 1);

        let v = Value::new(1.5, "1/farad").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, CAPACITANCE_MAP);
        assert_eq!(
            v.v_capacitance,
            Some(UnitElectricCapacitance::Farad(Metric::None))
        );
        assert_eq!(v.exp[CAPACITANCE_INDEX], -1);

        let v = Value::new(1.5, "1/farads").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, CAPACITANCE_MAP);
        assert_eq!(
            v.v_capacitance,
            Some(UnitElectricCapacitance::Farad(Metric::None))
        );
        assert_eq!(v.exp[CAPACITANCE_INDEX], -1);
    }

    #[test]
    fn unique_names_micron() {
        let v = Value::new(1.5, "micron").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Meter(Metric::Micro)));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "microns").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Meter(Metric::Micro)));
        assert_eq!(v.exp[LENGTH_INDEX], 1);

        let v = Value::new(1.5, "1/micron").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Meter(Metric::Micro)));
        assert_eq!(v.exp[LENGTH_INDEX], -1);

        let v = Value::new(1.5, "1/microns").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, LENGTH_MAP);
        assert_eq!(v.v_length, Some(UnitLength::Meter(Metric::Micro)));
        assert_eq!(v.exp[LENGTH_INDEX], -1);
    }

    #[test]
    fn unique_names_hour() {
        let v = Value::new(1.5, "hour").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Hour));
        assert_eq!(v.exp[TIME_INDEX], 1);

        let v = Value::new(1.5, "hours").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Hour));
        assert_eq!(v.exp[TIME_INDEX], 1);

        let v = Value::new(1.5, "h").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Hour));
        assert_eq!(v.exp[TIME_INDEX], 1);

        let v = Value::new(1.5, "hr").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Hour));
        assert_eq!(v.exp[TIME_INDEX], 1);

        let v = Value::new(1.5, "1/hour").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Hour));
        assert_eq!(v.exp[TIME_INDEX], -1);

        let v = Value::new(1.5, "1/hours").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Hour));
        assert_eq!(v.exp[TIME_INDEX], -1);

        let v = Value::new(1.5, "1/h").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Hour));
        assert_eq!(v.exp[TIME_INDEX], -1);

        let v = Value::new(1.5, "1/hr").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Hour));
        assert_eq!(v.exp[TIME_INDEX], -1);
    }

    #[test]
    fn unique_names_day() {
        let v = Value::new(1.5, "d").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Day));
        assert_eq!(v.exp[TIME_INDEX], 1);

        let v = Value::new(1.5, "day").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Day));
        assert_eq!(v.exp[TIME_INDEX], 1);

        let v = Value::new(1.5, "days").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Day));
        assert_eq!(v.exp[TIME_INDEX], 1);

        let v = Value::new(1.5, "1/d").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Day));
        assert_eq!(v.exp[TIME_INDEX], -1);

        let v = Value::new(1.5, "1/day").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Day));
        assert_eq!(v.exp[TIME_INDEX], -1);

        let v = Value::new(1.5, "1/days").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, TIME_MAP);
        assert_eq!(v.v_time, Some(UnitTime::Day));
        assert_eq!(v.exp[TIME_INDEX], -1);
    }

    #[test]
    fn unique_names_kilocalorie() {
        let v = Value::new(1.5, "Cal").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ENERGY_MAP);
        assert_eq!(v.v_energy, Some(UnitEnergy::GramCalorie(Metric::Kilo)));
        assert_eq!(v.exp[ENERGY_INDEX], 1);

        let v = Value::new(1.5, "1/Cal").unwrap();
        assert_eq!(v, 1.5);
        assert_eq!(v.unit_map, ENERGY_MAP);
        assert_eq!(v.v_energy, Some(UnitEnergy::GramCalorie(Metric::Kilo)));
        assert_eq!(v.exp[ENERGY_INDEX], -1);
    }

    #[test]
    fn weird_units() {
        let t = "1.5 (1/GHz^-1)*s".parse::<Value>().unwrap();

        let mut k = Value {
            val: 1.5,
            unit_map: FREQUENCY_MAP | TIME_MAP,
            exp: [0; 31],
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
            v_frequency: Some(UnitFrequency::Hertz(Metric::Giga)),
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
            v_time: Some(UnitTime::Second(Metric::None)),
            v_volume: None,
            v_magnetic_flux: None,
            v_magnetic_flux_density: None,
            v_solid_angle: None,
        };

        k.exp[TIME_INDEX] = 1;
        k.exp[FREQUENCY_INDEX] = 1;

        assert_eq!(t, k);

        let t = "1.5 1/(GHz^-1*s)".parse::<Value>().unwrap();

        let mut k = Value {
            val: 1.5,
            unit_map: FREQUENCY_MAP | TIME_MAP,
            exp: [0; 31],
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
            v_frequency: Some(UnitFrequency::Hertz(Metric::Giga)),
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
            v_time: Some(UnitTime::Second(Metric::None)),
            v_volume: None,
            v_magnetic_flux: None,
            v_magnetic_flux_density: None,
            v_solid_angle: None,
        };

        k.exp[TIME_INDEX] = -1;
        k.exp[FREQUENCY_INDEX] = 1;

        assert_eq!(t, k);

        let t = "1.5 1/GHz^-1*s".parse::<Value>().unwrap();

        let mut k = Value {
            val: 1.5,
            unit_map: FREQUENCY_MAP | TIME_MAP,
            exp: [0; 31],
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
            v_frequency: Some(UnitFrequency::Hertz(Metric::Giga)),
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
            v_time: Some(UnitTime::Second(Metric::None)),
            v_volume: None,
            v_magnetic_flux: None,
            v_magnetic_flux_density: None,
            v_solid_angle: None,
        };

        k.exp[TIME_INDEX] = -1;
        k.exp[FREQUENCY_INDEX] = 1;

        assert_eq!(t, k);
    }

    #[test]
    #[should_panic]
    fn parentheses_testing() {
        let _ = "2.0 ))((kg".parse::<Value>().unwrap();
    }
}
