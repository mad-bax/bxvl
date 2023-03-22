/**
 * File    :> mod.rs
 * Author  :> Bax
 * Version :> 0.0.1
 * Details :>
 */

//pub mod unit_worker;
//mod unit_parsing;

use std::fmt::Display;

use crate::constants;

/// Trait that can be used and called by all of the unit types
trait Convert<T1> {
    /// The function template for a unit type
    fn convert(&self, other:&T1) -> f64;
}

/// The Metric scale names
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone, Default)]
pub enum Metric {
    /// Yocto
    Yocto,
    /// Zepto
    Zepto,
    /// Atto
    Atto,
    /// Femto
    Femto,
    /// Pico
    Pico,
    /// Nano
    Nano,
    /// Micro
    Micro,
    /// Milli
    Milli,
    /// Centi
    Centi,
    /// Deci
    Deci,
    /// None (default)
    #[default] None,
    /// Deca
    Deca,
    /// Hecto
    Hecto,
    /// Kilo
    Kilo,
    /// Mega
    Mega,
    /// Giga
    Giga,
    /// Tera
    Tera,
    /// Peta
    Peta,
    /// Exa
    Exa,
    /// Zetta
    Zetta,
    /// Yotta
    Yotta
}

impl Metric {
    /// Returns the numeric scaling of a given metric prefix
    pub fn scale(&self) -> f64 {
        match self {
            Metric::Yotta => 1000000000000000000000000.0,
            Metric::Zetta => 1000000000000000000000.0,
            Metric::Exa   => 1000000000000000000.0,
            Metric::Peta  => 1000000000000000.0,
            Metric::Tera  => 1000000000000.0,
            Metric::Giga  => 1000000000.0,
            Metric::Mega  => 1000000.0,
            Metric::Kilo  => 1000.0,
            Metric::Hecto => 100.0,
            Metric::Deca  => 10.0,
            Metric::None  => 1.0,
            Metric::Deci  => 0.1,
            Metric::Centi => 0.01,
            Metric::Milli => 0.001,
            Metric::Micro => 0.000001,
            Metric::Nano  => 0.000000001,
            Metric::Pico  => 0.000000000001,
            Metric::Femto => 0.000000000000001,
            Metric::Atto  => 0.000000000000000001,
            Metric::Zepto => 0.000000000000000000001,
            Metric::Yocto => 0.000000000000000000000001
        }
    }

    /// Returns the string representation of the metric prefix
    pub fn as_str(&self) -> &str {
        match self {
            Metric::Yotta => "Y",
            Metric::Zetta => "Z",
            Metric::Exa   => "E",
            Metric::Peta  => "P",
            Metric::Tera  => "T",
            Metric::Giga  => "G",
            Metric::Mega  => "M",
            Metric::Kilo  => "k",
            Metric::Hecto => "h",
            Metric::Deca  => "da",
            Metric::None  => "",
            Metric::Deci  => "d",
            Metric::Centi => "c",
            Metric::Milli => "m",
            Metric::Micro => "μ",
            Metric::Nano  => "n",
            Metric::Pico  => "p",
            Metric::Femto => "f",
            Metric::Atto  => "a",
            Metric::Zepto => "z",
            Metric::Yocto => "y"
        }
    }
}

/// The unit types for length
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitLength {
    /// SI unit
    Meter(Metric),
    /// Imperial
    Inch,
    /// Imperial
    Foot,
    /// Imperial
    Yard,
    /// Imperial
    Mile,
    /// Astronomical
    AstronomicalUnit,
    /// Astronomical
    Parsec,
    /// SI integrated 
    LightYear,
    /// Legacy
    Angstrom
}

impl Display for UnitLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Meter(m) => {
                ret.push_str(m.as_str());
                ret.push('m');
            }
            Self::Inch => ret.push_str("in"),
            Self::Foot => ret.push_str("ft"),
            Self::Yard => ret.push_str("yds"),
            Self::Mile => ret.push_str("miles"),
            Self::AstronomicalUnit => ret.push_str("AU"),
            Self::Parsec => ret.push_str("pc"),
            Self::LightYear => ret.push_str("lyr"),
            Self::Angstrom => ret.push('Å')
        }
        write!(f, "{}", ret)
    }
}

impl UnitLength {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Meter(m) => m.scale(),
            _ => 1.0
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Meter(_) => 1.0,
            Self::Inch => constants::LENGTH_IN_TO_METER,
            Self::Foot => constants::LENGTH_FT_TO_METER,
            Self::Yard => constants::LENGTH_YD_TO_METER,
            Self::Mile => constants::LENGTH_MILE_TO_METER,
            Self::AstronomicalUnit => constants::LENGTH_AU_TO_METER,
            Self::Parsec => constants::LENGTH_PC_TO_METER,
            Self::LightYear => constants::LENGTH_LYR_TO_METER,
            Self::Angstrom => constants::LENGTH_A_TO_METER
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitLength) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert_liter(&self, other:&UnitVolume) -> f64 {
        constants::METER3_TO_LITER / other.convert(&UnitVolume::Liter(Metric::None))
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Meter(m) => *m,
            _ => Metric::None
        }
    }
}

/// The unit types for time
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitTime {
    /// SI unit
    Second(Metric),
    /// Non - SI
    Minute,
    /// Non - SI
    Hour,
    /// Non - SI
    Day
}

impl Display for UnitTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Second(m) => {
                ret.push_str(m.as_str());
                ret.push('s');
            }
            Self::Minute => ret.push_str("min"),
            Self::Hour => ret.push_str("hr"),
            Self::Day => ret.push_str("day")
        }
        write!(f, "{}", ret)
    }
}

impl UnitTime {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Second(m) => m.scale(),
            _ => 1.0
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Second(_) => 1.0,
            Self::Minute => 1.0 / 60.0,
            Self::Hour => 1.0 / 3600.0,
            Self::Day => 1.0 / 86400.0
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitTime) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert_freq(&self, other:&UnitFrequency) -> f64 {
        (self.scale() / other.scale()) * self.base()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Second(m) => *m,
            _ => Metric::None
        }
    }
}

/// The unit types for mass
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitMass {
    /// SI unit
    Gram(Metric),
    /// Imperial
    Grain,
    /// Imperial
    Ounce,
    /// Imperial
    Pound,
}

impl Display for UnitMass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Gram(m) => {
                ret.push_str(m.as_str());
                ret.push('g');
            }
            Self::Grain => ret.push_str("gr"),
            Self::Ounce => ret.push_str("oz"),
            Self::Pound => ret.push_str("lb")
        }
        write!(f, "{}", ret)
    }
}

impl UnitMass {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Gram(m) => m.scale(),
            _ => 1.0
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Gram(_) => 1.0,
            Self::Grain => constants::MASS_GR_TO_G,
            Self::Ounce => constants::MASS_OZ_TO_G,
            Self::Pound => constants::MASS_LB_TO_G
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitMass) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Gram(m) => *m,
            _ => Metric::None
        }
    }
}

/// The unit types for electric current
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitElectricCurrent {
    /// SI unit
    Ampere(Metric)
}

impl Display for UnitElectricCurrent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Ampere(m) => ret.push_str(m.as_str())
        }
        ret.push('A');
        write!(f, "{}", ret)
    }
}

impl UnitElectricCurrent {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Ampere(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitElectricCurrent) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Ampere(m) => *m
        }
    }
}

/// The unit types for electric charge
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitElectricCharge {
    /// SI unit
    Coulomb(Metric)
}

impl Display for UnitElectricCharge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Coulomb(m) => ret.push_str(m.as_str())
        }
        ret.push('C');
        write!(f, "{}", ret)
    }
}

impl UnitElectricCharge {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Coulomb(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitElectricCharge) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Coulomb(m) => *m
        }
    }
}

/// The unit types for electric potential
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitElectricPotential {
    /// SI unit
    Volt(Metric)
}

impl Display for UnitElectricPotential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Volt(m) => ret.push_str(m.as_str())
        }
        ret.push('V');
        write!(f, "{}", ret)
    }
}

impl UnitElectricPotential {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Volt(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitElectricPotential) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Volt(m) => *m
        }
    }
}

/// The unit types for electric conductance
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitElectricConductance {
    /// SI unit
    Siemens(Metric)
}

impl Display for UnitElectricConductance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Siemens(m) => ret.push_str(m.as_str())
        }
        ret.push('S');
        write!(f, "{}", ret)
    }
}

impl UnitElectricConductance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Siemens(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitElectricConductance) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Siemens(m) => *m
        }
    }
}

/// The unit types for electric capacitance
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitCapacitance {
    /// SI unit
    Farad(Metric)
}

impl Display for UnitCapacitance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Farad(m) => ret.push_str(m.as_str())
        }
        ret.push('F');
        write!(f, "{}", ret)
    }
}

impl UnitCapacitance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Farad(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitCapacitance) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Farad(m) => *m
        }
    }
}

/// The unit types for electric resistance
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitResistance {
    /// SI unit
    Ohm(Metric)
}

impl Display for UnitResistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Ohm(m) => ret.push_str(m.as_str())
        }
        ret.push('Ω');
        write!(f, "{}", ret)
    }
}

impl UnitResistance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Ohm(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitResistance) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Ohm(m) => *m
        }
    }
}

/// The unit types for electric inductance
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitInductance {
    /// SI unit
    Henry(Metric)
}

impl Display for UnitInductance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Henry(m) => ret.push_str(m.as_str())
        }
        ret.push('H');
        write!(f, "{}", ret)
    }
}

impl UnitInductance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Henry(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitInductance) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Henry(m) => *m
        }
    }
}

/// The unit types for magnetic flux
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitMagneticFlux {
    /// SI unit
    Weber(Metric)
}

impl Display for UnitMagneticFlux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Weber(m) => ret.push_str(m.as_str())
        }
        ret.push_str("Wb");
        write!(f, "{}", ret)
    }
}

impl UnitMagneticFlux {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Weber(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitMagneticFlux) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Weber(m) => *m
        }
    }
}

/// The unit types for magnetic flux density
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitMagneticFluxDensity {
    /// SI unit
    Tesla(Metric)
}

impl Display for UnitMagneticFluxDensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Tesla(m) => ret.push_str(m.as_str())
        }
        ret.push('T');
        write!(f, "{}", ret)
    }
}

impl UnitMagneticFluxDensity {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Tesla(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitMagneticFluxDensity) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Tesla(m) => *m
        }
    }
}

/// The unit types for temperature
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitTemperature {
    /// SI Unit
    Celsius,
    /// Imperial
    Fahrenheit,
    /// SI unit 
    Kelvin
}

impl Display for UnitTemperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Celsius => ret.push_str("°c"),
            Self::Kelvin => ret.push('K'),
            Self::Fahrenheit => ret.push_str("°f")
        }
        write!(f, "{}", ret)
    }
}

impl UnitTemperature {

    /// Returns a `f64` to assign to a `Value`
    pub fn convert(&self, other:&UnitTemperature, val:f64) -> f64 {
        if self == other {
            return val;
        }

        match self {
            Self::Celsius => {
                match other {
                    Self::Celsius => val,
                    Self::Fahrenheit => (val*1.8)+32.0,
                    Self::Kelvin => f64::max(val + 273.15, 0.0),
                }
            }
            Self::Fahrenheit => {
                match other {
                    Self::Celsius => (val-32.0)/1.8,
                    Self::Fahrenheit => val,
                    Self::Kelvin => f64::max(((val-32.0)/1.8)+273.15, 0.0)
                }
            }
            Self::Kelvin => {
                match other {
                    Self::Celsius => val-273.15,
                    Self::Fahrenheit => ((val-273.15)*1.8)+32.0,
                    Self::Kelvin => val
                }
            }
        }
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        Metric::None
    }
}

/// The unit types for substance
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitSubstance {
    /// SI unit
    Mole(Metric)
}

impl Display for UnitSubstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Mole(m) => ret.push_str(m.as_str())
        }
        ret.push_str("mol");
        write!(f, "{}", ret)
    }
}

impl UnitSubstance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Mole(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitSubstance) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Mole(m) => *m
        }
    }
}

/// The unit types for luminous intensity
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitLuminousIntensity {
    /// SI unit
    Candela(Metric)
}

impl Display for UnitLuminousIntensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Candela(m) => ret.push_str(m.as_str())
        }
        ret.push_str("cd");
        write!(f, "{}", ret)
    }
}

impl UnitLuminousIntensity {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Candela(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitLuminousIntensity) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Candela(m) => *m
        }
    }
}

/// The unit types for luminous flux
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitLuminousFlux {
    /// SI unit
    Lumen(Metric)
}

impl Display for UnitLuminousFlux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Lumen(m) => ret.push_str(m.as_str())
        }
        ret.push_str("lm");
        write!(f, "{}", ret)
    }
}

impl UnitLuminousFlux {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Lumen(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitLuminousFlux) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Lumen(m) => *m
        }
    }
}

/// The unit types for illuminance
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitIlluminance {
    /// SI unit
    Lux(Metric)
}

impl Display for UnitIlluminance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Lux(m) => ret.push_str(m.as_str())
        }
        ret.push_str("lx");
        write!(f, "{}", ret)
    }
}

impl UnitIlluminance {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Lux(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitIlluminance) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Lux(m) => *m
        }
    }
}

/// The unit types for volume
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitVolume {
    /// SI unit
    Liter(Metric)
}

impl Display for UnitVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Liter(m) => ret.push_str(m.as_str())
        }
        ret.push('l');
        write!(f, "{}", ret)
    }
}

impl UnitVolume {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Liter(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitVolume) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert_meter(&self, other:&UnitLength) -> f64 {
        self.scale() * (constants::METER3_TO_LITER * other.convert(&UnitLength::Meter(Metric::None)))
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Liter(m) => *m
        }
    }
}

/// The unit types for pressure
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitPressure {
    /// SI unit
    Pascal(Metric),
    /// SI integrated, Common Standard
    Bar(Metric),
    /// Common Standard
    Torr,
    /// SI integrated 
    Hgmm,
    /// Imperial
    Hgin,
    /// Common Standard
    Atm,
    /// Imperial
    Psi
}

impl Display for UnitPressure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Pascal(m) => {
                ret.push_str(m.as_str());
                ret.push_str("Pa");
            }
            Self::Bar(m) => {
                ret.push_str(m.as_str());
                ret.push_str("bar");
            }
            Self::Torr => ret.push_str("torr"),
            Self::Hgmm => ret.push_str("mmHg"),
            Self::Hgin => ret.push_str("inHg"),
            Self::Atm => ret.push_str("atm"),
            Self::Psi => ret.push_str("psi")
        }
        write!(f, "{}", ret)
    }
}

impl UnitPressure {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Pascal(m) => m.scale(),
            Self::Bar(m) => m.scale(),
            _ => 1.0
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Pascal(_) => 1.0,
            Self::Bar(_) => constants::PR_BAR_TO_P,
            Self::Torr => constants::PR_TORR_TO_P,
            Self::Hgmm => constants::PR_MM_TO_P,
            Self::Hgin => constants::PR_IN_TO_P,
            Self::Atm => constants::PR_ATM_TO_P,
            Self::Psi => constants::PR_PSI_TO_P
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitPressure) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Pascal(m) => *m,
            Self::Bar(m) => *m,
            _ => Metric::None
        }
    }
}

/// The unit types for angles
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitAngle {
    /// Common Standard
    Degree,
    /// SI unit
    Radian(Metric),
    /// Common Standard
    Moa
}

impl Display for UnitAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Radian(Metric::Milli) => ret.push_str("mil"),
            Self::Radian(m) => {
                ret.push_str(m.as_str());
                ret.push_str("rad")
            }
            Self::Degree => ret.push('°'),
            Self::Moa => ret.push_str("moa")
        }
        write!(f, "{}", ret)
    }
}

impl UnitAngle {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Radian(m) => m.scale(),
            _ => 1.0
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Radian(_) => 1.0,
            Self::Degree => constants::ANGLE_DEG_TO_RAD,
            Self::Moa => constants::ANGLE_MOA_TO_RAD,
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitAngle) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Radian(m) => *m,
            _ => Metric::None
        }
    }
}

/// The unit types of solid angles
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitSolidAngle {
    /// SI unit
    Steradian(Metric)
}

impl Display for UnitSolidAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Steradian(m) => {
                ret.push_str(m.as_str());
                ret.push_str("sr")
            }
        }
        write!(f, "{}", ret)
    }
}

impl UnitSolidAngle {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Steradian(m) => m.scale()
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Steradian(_) => 1.0
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitSolidAngle) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Steradian(m) => *m
        }
    }
}

/// The unit types of frequency
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitFrequency {
    /// SI unit
    Hertz(Metric)
}

impl Display for UnitFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Hertz(m) => ret.push_str(m.as_str())
        }
        ret.push_str("Hz");
        write!(f, "{}", ret)
    }
}

impl UnitFrequency {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Hertz(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitFrequency) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert_time(&self, other:&UnitTime) -> f64 {
        (self.scale() / other.scale()) * (other.convert(&UnitTime::Second(Metric::None)))
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Hertz(m) => *m
        }
    }
}

/// The unit types of force
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitForce {
    /// SI unit
    Newton(Metric),
    /// Imperial
    PoundForce
}

impl Display for UnitForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Newton(m) => {
                ret.push_str(m.as_str());
                ret.push('N');
            }
            Self::PoundForce => ret.push_str("lbfr")
        }
        write!(f, "{}", ret)
    }
}

impl UnitForce {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Newton(m) => m.scale(),
            _ => 1.0
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 { 
        match self {
            Self::Newton(_) => 1.0,
            Self::PoundForce => constants::FC_LBF_TO_N
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitForce) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Newton(m) => *m,
            _ => Metric::None
        }
    }
}

/// The unit types of energy
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitEnergy {
    /// SI unit
    Joule(Metric),
    /// SI integrated
    GramCalorie(Metric),
    /// Imperial
    FootPound,
    /// SI integrated
    ElectronVolt
}

impl Display for UnitEnergy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Joule(m) => {
                ret.push_str(m.as_str());
                ret.push('J');
            }
            Self::GramCalorie(Metric::Kilo) => ret.push_str("Cal"),
            Self::GramCalorie(m) => {
                ret.push_str(m.as_str());
                ret.push_str("cal");
            }
            Self::FootPound => ret.push_str("ftlb"),
            Self::ElectronVolt => ret.push_str("eV")
        }
        write!(f, "{}", ret)
    }
}

impl UnitEnergy {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Joule(m) => m.scale(),
            Self::GramCalorie(m) => m.scale(),
            _ => 1.0
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Joule(_) => 1.0,
            Self::GramCalorie(_) => constants::EN_CAL_TO_J,
            Self::FootPound => constants::EN_FTLB_TO_J,
            Self::ElectronVolt => constants::EN_EV_TO_J
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitEnergy) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Joule(m) => *m,
            Self::GramCalorie(m) => *m,
            _ => Metric::None
        }
    }
}

/// The unit types of power
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitPower {
    /// SI unit
    Watt(Metric)
}

impl Display for UnitPower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Watt(m) => ret.push_str(m.as_str())
        }
        ret.push('W');
        write!(f, "{}", ret)
    }
}

impl UnitPower {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Watt(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitPower) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Watt(m) => *m
        }
    }
}

/// The unit types of radioactivity
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitRadioactivity {
    /// SI unit
    Becquerel(Metric),
    /// Legacy
    Curie
}

impl Display for UnitRadioactivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Becquerel(m) => {
                ret.push_str(m.as_str());
                ret.push_str("Bq");
            }
            Self::Curie => ret.push_str("Ci")
        }
        write!(f, "{}", ret)
    }
}

impl UnitRadioactivity {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Becquerel(m) => m.scale(),
            _ => 1.0
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Becquerel(_) => 1.0,
            Self::Curie => constants::RADIO_C_TO_BQ
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitRadioactivity) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Becquerel(m) => *m,
            _ => Metric::None
        }
    }
}

/// The unit types of absorbed dose of ionizing radiation
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitAbsorbedDose {
    /// SI unit
    Gray(Metric),
    /// Legacy
    Roentgen,
    /// Legacy
    Rad
}

impl Display for UnitAbsorbedDose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Gray(m) => {
                ret.push_str(m.as_str());
                ret.push_str("Gy");
            }
            Self::Roentgen => ret.push('R'),
            Self::Rad => ret.push_str("rads")
        }
        write!(f, "{}", ret)
    }
}

impl UnitAbsorbedDose {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Gray(m) => m.scale(),
            _ => 1.0
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Gray(_) => 1.0,
            Self::Roentgen => constants::AB_ROE_TO_GY,
            Self::Rad => constants::AB_RAD_TO_GY
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitAbsorbedDose) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Gray(m) => *m,
            _ => Metric::None
        }
    }
}

/// The unit types of equaivalent dose of ionizing radiation
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitRadioactivityExposure {
    /// SI unit
    Sievert(Metric),
    /// Legacy
    Rem
}

impl Display for UnitRadioactivityExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Sievert(m) => {
                ret.push_str(m.as_str());
                ret.push_str("Sv");
            }
            Self::Rem => ret.push_str("rem")
        }
        write!(f, "{}", ret)
    }
}

impl UnitRadioactivityExposure {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Sievert(m) => m.scale(),
            _ => 1.0
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Sievert(_) => 1.0,
            Self::Rem => constants::RADEX_REM_TO_SV
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitRadioactivityExposure) -> f64 {
        (self.scale() / self.scale()) * (self.base() / other.base())
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Sievert(m) => *m,
            _ => Metric::None
        }
    }
}

/// The unit types for catalytic activity
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitCatalyticActivity {
    /// SI unit
    Katal(Metric)
}

impl Display for UnitCatalyticActivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Katal(m) => ret.push_str(m.as_str())
        }
        ret.push_str("kat");
        write!(f, "{}", ret)
    }
}

impl UnitCatalyticActivity {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Katal(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitCatalyticActivity) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Katal(m) => *m
        }
    }
}

/// The unit types of a measurement of sound
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitSound {
    /// SI unit
    Bel(Metric)
}

impl Display for UnitSound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Bel(m) => ret.push_str(m.as_str())
        }
        ret.push('B');
        write!(f, "{}", ret)
    }
}

impl UnitSound {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            Self::Bel(m) => m.scale(),
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitSound) -> f64 {
        self.scale() / other.scale()
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Bel(m) => *m
        }
    }
}

/// The unit types for a measurement of information
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum UnitInformation {
    /// Not SI but uses metric prefixing
    Bit(Metric),
    /// Not SI but uses metric prefixing
    Byte(Metric)
}

impl Display for UnitInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret:String = String::new();
        match self {
            Self::Bit(m) => {
                ret.push_str(m.as_str());
                ret.push_str("bits");
            }
            Self::Byte(m) => {
                ret.push_str(m.as_str());
                ret.push('b');
            }
        }
        write!(f, "{}", ret)
    }
}

impl UnitInformation {
    /// Returns the metric scaler of an SI unit
    fn scale(&self) -> f64 {
        match self {
            UnitInformation::Bit(m) | UnitInformation::Byte(m) => {
            match m {
                Metric::Yotta => 1208925819614629174706176.0,
                Metric::Zetta => 1180591620717411303424.0,
                Metric::Exa   => 1152921504606846976.0,
                Metric::Peta  => 1125899906842624.0,
                Metric::Tera  => 1099511627776.0,
                Metric::Giga  => 1073741824.0,
                Metric::Mega  => 1048576.0,
                Metric::Kilo  => 1024.0,
                Metric::None  => 1.0,
                _ => 1.0
            }}
        }
    }

    /// Returns the base unit conversion in relation to the standard SI unit
    fn base(&self) -> f64 {
        match self {
            Self::Byte(_) => 1.0,
            Self::Bit(_) => 0.125
        }
    }

    /// Returns the `f64` multiplier to convert a `Value`
    pub fn convert(&self, other:&UnitInformation) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    /// Returns the `Metric` prefix for the unit
    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Bit(m) => *m,
            Self::Byte(m) => *m
        }
    }
}

#[cfg(test)]
mod units_unit_test {
    use crate::units::Metric;
    use crate::units::UnitCatalyticActivity;
    use crate::units::UnitInformation;
    use crate::units::UnitTime;

    use super::UnitAbsorbedDose;
    use super::UnitAngle;
    use super::UnitEnergy;
    use super::UnitForce;
    use super::UnitLength;
    use super::UnitMass;
    use super::UnitPressure;
    use super::UnitRadioactivity;
    use super::UnitRadioactivityExposure;

    /// # Metric Comparison
    /// 
    /// All of the metric prefixes are in the right order
    #[test]
    fn metric_comparison() {
        assert_eq!(true, Metric::Yocto < Metric::Zepto);
        assert_eq!(true, Metric::Zepto < Metric::Atto);
        assert_eq!(true, Metric::Atto < Metric::Femto);
        assert_eq!(true, Metric::Femto < Metric::Pico);
        assert_eq!(true, Metric::Pico < Metric::Nano);
        assert_eq!(true, Metric::Nano < Metric::Micro);
        assert_eq!(true, Metric::Micro < Metric::Milli);
        assert_eq!(true, Metric::Milli < Metric::Centi);
        assert_eq!(true, Metric::Centi < Metric::Deci);
        assert_eq!(true, Metric::Deci < Metric::None);
        assert_eq!(true, Metric::None < Metric::Deca);
        assert_eq!(true, Metric::Deca < Metric::Hecto);
        assert_eq!(true, Metric::Hecto < Metric::Kilo);
        assert_eq!(true, Metric::Kilo < Metric::Mega);
        assert_eq!(true, Metric::Mega < Metric::Giga);
        assert_eq!(true, Metric::Giga < Metric::Tera);
        assert_eq!(true, Metric::Tera < Metric::Peta);
        assert_eq!(true, Metric::Peta < Metric::Exa);
        assert_eq!(true, Metric::Exa < Metric::Zetta);
        assert_eq!(true, Metric::Zetta < Metric::Yotta);
    }

    /// # Metric Comparison Scale
    /// 
    /// All of the metric scale values are in the right order
    #[test]
    fn metric_comparison_scale() {
        assert_eq!(true, Metric::Yocto.scale() < Metric::Zepto.scale());
        assert_eq!(true, Metric::Zepto.scale() < Metric::Atto.scale());
        assert_eq!(true, Metric::Atto.scale() < Metric::Femto.scale());
        assert_eq!(true, Metric::Femto.scale() < Metric::Pico.scale());
        assert_eq!(true, Metric::Pico.scale() < Metric::Nano.scale());
        assert_eq!(true, Metric::Nano.scale() < Metric::Micro.scale());
        assert_eq!(true, Metric::Micro.scale() < Metric::Milli.scale());
        assert_eq!(true, Metric::Milli.scale() < Metric::Centi.scale());
        assert_eq!(true, Metric::Centi.scale() < Metric::Deci.scale());
        assert_eq!(true, Metric::Deci.scale() < Metric::None.scale());
        assert_eq!(true, Metric::None.scale() < Metric::Deca.scale());
        assert_eq!(true, Metric::Deca.scale() < Metric::Hecto.scale());
        assert_eq!(true, Metric::Hecto.scale() < Metric::Kilo.scale());
        assert_eq!(true, Metric::Kilo.scale() < Metric::Mega.scale());
        assert_eq!(true, Metric::Mega.scale() < Metric::Giga.scale());
        assert_eq!(true, Metric::Giga.scale() < Metric::Tera.scale());
        assert_eq!(true, Metric::Tera.scale() < Metric::Peta.scale());
        assert_eq!(true, Metric::Peta.scale() < Metric::Exa.scale());
        assert_eq!(true, Metric::Exa.scale() < Metric::Zetta.scale());
        assert_eq!(true, Metric::Zetta.scale() < Metric::Yotta.scale());
    }

    #[test]
    fn unit_get_metric() {
        let t = UnitPressure::Bar(Metric::Exa);
        assert_eq!(t.get_metric(), Metric::Exa);
    }

    /// Unit Information Comparison Base
    /// 
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_information_base_comparison() {
        // Bits
        assert!(UnitInformation::Bit(Metric::None).base() == 0.125);
        // Bytes are the base 'SI unit'
        assert!(UnitInformation::Byte(Metric::None).base() == 1.0);
    }

    /// Unit Length Comparison Base
    /// 
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_length_base_comparison() {
        // Meters are the base SI unit
        assert!(UnitLength::Meter(Metric::None).base() == 1.0);
        // Feet
        assert!(UnitLength::Foot.base() == 0.3048);
        // Inches
        assert!(UnitLength::Inch.base() == 0.0254);
        // Yards
        assert!(UnitLength::Yard.base() == 0.9144);
        // Mile
        assert!(UnitLength::Mile.base() == 1609.344);
        // Astronomical Unit
        assert!(UnitLength::AstronomicalUnit.base() == 149_597_870_700.0);
        // Lightyear
        assert!(UnitLength::LightYear.base() == 9_460_730_472_580_800.0);
        // Ångström
        assert!(UnitLength::Angstrom.base() == 0.000_000_000_1);
        // Parsec
        assert!(UnitLength::Parsec.base() >= 3.085_677_581_491e16);
    }

    /// Unit Mass Comparison Base
    /// 
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_mass_base_comparison() {
        // Grams are the base SI unit
        assert!(UnitMass::Gram(Metric::None).base() == 1.0);
        // Pounds
        assert!(UnitMass::Pound.base() == 453.592_37);
        // Grains
        assert!(UnitMass::Grain.base() >= 0.06479890);
        // Ounces
        assert!(UnitMass::Ounce.base() >= 28.349_523_124);
    }

    /// Unit Angle Comparison Base
    /// 
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_angle_base_comparison() {
        // Radians are the base SI unit
        assert!(UnitAngle::Radian(Metric::None).base() == 1.0);
        // Degrees
        assert!(UnitAngle::Degree.base() >= 0.017_453_292_50);
        // Minute of Angle
        assert!(UnitAngle::Moa.base() >= 0.000_290_888_208_664);
    }

    /// Unit Energy Comparison Base
    /// 
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_energy_base_comparison() {
        // Joules are the base SI unit
        assert!(UnitEnergy::Joule(Metric::None).base() == 1.0);
        // Calories
        assert!(UnitEnergy::GramCalorie(Metric::None).base() == 4.184);
        // Footpounds
        assert!(UnitEnergy::FootPound.base() == 1.355818);
        // Electron Volts
        assert!(UnitEnergy::ElectronVolt.base() >= 1.602_176_633e-19);
    }

    /// Unit Force Comparison Base
    /// 
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_force_base_comparison() {
        // Newtons are the base SI unit
        assert!(UnitForce::Newton(Metric::None).base() == 1.0);
        // Poundforce
        assert!(UnitForce::PoundForce.base() == 4.448_221_615_260_5);
    }

    /// Unit Pressure Comparison Base
    /// 
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_pressure_base_comparison() {
        // Pascals are the base SI unit
        assert!(UnitPressure::Pascal(Metric::None).base() == 1.0);
        // Atmospheres
        assert!(UnitPressure::Atm.base() == 101325.0);
        // Bar
        assert!(UnitPressure::Bar(Metric::None).base() == 100000.0);
        // inHg
        assert!(UnitPressure::Hgin.base() >= 3386.3886665);
        // mmHg
        assert!(UnitPressure::Hgmm.base() >= 133.322387414);
        // PSI
        assert!(UnitPressure::Psi.base() == 6894.757);
        // Torr
        assert!(UnitPressure::Torr.base() >= 133.322368420);
    }

    /// Unit Radioactivity Comparison Base
    /// 
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_radioactivity_base_comparison() {
        // Becquerels are the base SI unit
        assert!(UnitRadioactivity::Becquerel(Metric::None).base() == 1.0);
        // Curies
        assert!(UnitRadioactivity::Curie.base() == 37_000_000_000.0);
    }

    /// Unit Absorbed Dose of Ionizing Radiation Comparison Base
    /// 
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_absorbed_base_comparison() {
        // Grays are the base SI unit
        assert!(UnitAbsorbedDose::Gray(Metric::None).base() == 1.0);
        // Rads
        assert!(UnitAbsorbedDose::Rad.base() == 0.01);
        // Roentgens
        assert!(UnitAbsorbedDose::Roentgen.base() >= 0.00877000656);
    }

    /// Unit Equivalent Dose of Ionizing Radiation Comparison Base
    /// 
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_equivalent_base_comparison() {
        // Seiverts are the base SI unit
        assert!(UnitRadioactivityExposure::Sievert(Metric::None).base() == 1.0);
        // Rems
        assert!(UnitRadioactivityExposure::Rem.base() == 0.01);
    }

    /// Unit Time Comparison Base
    /// 
    /// All units must return the 'base' value relative to the standard SI unit
    #[test]
    fn unit_time_base_comparison() {
        assert_eq!(UnitTime::Second(Metric::None).base(), 1.0);
        assert_eq!(UnitTime::Minute.base(), 1.0/60.0);
        assert_eq!(UnitTime::Hour.base(), 1.0/3600.0);
        assert_eq!(UnitTime::Day.base(), 1.0/86400.0);
    }
}