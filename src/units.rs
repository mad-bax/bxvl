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

trait Convert<T1> {
    fn convert(&self, other:&T1) -> f64;
}

/* Metric scales */
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Metric {
    Yocto,
    Zepto,
    Atto,
    Femto,
    Pico,
    Nano,
    Micro,
    Milli,
    Centi,
    Deci,
    None,
    Deca,
    Hecto,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
    Exa,
    Zetta,
    Yotta
}

impl Metric {
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

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitLength {
    Meter(Metric),
    Inch,
    Foot,
    Yard,
    Mile,
    AstronomicalUnit,
    Parsec,
    LightYear,
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
    fn scale(&self) -> f64 {
        match self {
            Self::Meter(m) => m.scale(),
            _ => 1.0
        }
    }

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

    pub fn convert(&self, other:&UnitLength) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    pub fn convert_liter(&self, other:&UnitVolume) -> f64 {
        constants::METER3_TO_LITER / other.convert(&UnitVolume::Liter(Metric::None))
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Meter(m) => m.clone(),
            _ => Metric::None
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitTime {
    Second(Metric),
    Minute,
    Hour,
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
    fn scale(&self) -> f64 {
        match self {
            Self::Second(m) => m.scale(),
            _ => 1.0
        }
    }

    fn base(&self) -> f64 {
        match self {
            Self::Second(_) => 1.0,
            Self::Minute => 1.0 / 60.0,
            Self::Hour => 1.0 / 3600.0,
            Self::Day => 1.0 / 86400.0
        }
    }

    pub fn convert(&self, other:&UnitTime) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Second(m) => m.clone(),
            _ => Metric::None
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitMass {
    Gram(Metric),
    Grain,
    Ounce,
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
    fn scale(&self) -> f64 {
        match self {
            Self::Gram(m) => m.scale(),
            _ => 1.0
        }
    }

    fn base(&self) -> f64 {
        match self {
            Self::Gram(_) => 1.0,
            Self::Grain => constants::MASS_GR_TO_G,
            Self::Ounce => constants::MASS_OZ_TO_G,
            Self::Pound => constants::MASS_LB_TO_G
        }
    }

    pub fn convert(&self, other:&UnitMass) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Gram(m) => m.clone(),
            _ => Metric::None
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitElectricCurrent {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Ampere(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitElectricCurrent) -> f64 {
        self.scale() / other.scale()
    }

        pub fn get_metric(&self) -> Metric {
        match self {
            Self::Ampere(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitElectricCharge {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Coulomb(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitElectricCharge) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Coulomb(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitElectricPotential {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Volt(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitElectricPotential) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Volt(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitElectricConductance {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Siemens(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitElectricConductance) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Siemens(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitCapacitance {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Farad(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitCapacitance) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Farad(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitResistance {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Ohm(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitResistance) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Ohm(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitInductance {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Henry(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitInductance) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Henry(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitMagneticFlux {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Weber(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitMagneticFlux) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Weber(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitMagneticFluxDensity {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Tesla(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitMagneticFluxDensity) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Tesla(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitTemperature {
    Celsius,
    Fahrenheit,
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

    pub fn get_metric(&self) -> Metric {
        Metric::None
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitSubstance {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Mole(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitSubstance) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Mole(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitLuminousIntensity {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Candela(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitLuminousIntensity) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Candela(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitLuminousFlux {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Lumen(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitLuminousFlux) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Lumen(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitIlluminance {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Lux(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitIlluminance) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Lux(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitVolume {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Liter(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitVolume) -> f64 {
        self.scale() / other.scale()
    }

    pub fn convert_meter(&self, other:&UnitLength) -> f64 {
        self.scale() * (constants::METER3_TO_LITER * other.convert(&UnitLength::Meter(Metric::None)))
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Liter(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitPressure {
    Pascal(Metric),
    Bar(Metric),
    Torr,
    Hgmm,
    Hgin,
    Atm,
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
    fn scale(&self) -> f64 {
        match self {
            Self::Pascal(m) => m.scale(),
            Self::Bar(m) => m.scale(),
            _ => 1.0
        }
    }

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

    pub fn convert(&self, other:&UnitPressure) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Pascal(m) => m.clone(),
            Self::Bar(m) => m.clone(),
            _ => Metric::None
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitAngle {
    Degree,
    Radian(Metric),
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
    fn scale(&self) -> f64 {
        match self {
            Self::Radian(m) => m.scale(),
            _ => 1.0
        }
    }

    fn base(&self) -> f64 {
        match self {
            Self::Radian(_) => 1.0,
            Self::Degree => constants::ANGLE_DEG_TO_RAD,
            Self::Moa => constants::ANGLE_MOA_TO_RAD,
        }
    }

    pub fn convert(&self, other:&UnitAngle) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Radian(m) => m.clone(),
            _ => Metric::None
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitSolidAngle {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Steradian(m) => m.scale(),
            _ => 1.0
        }
    }

    fn base(&self) -> f64 {
        match self {
            Self::Steradian(_) => 1.0
        }
    }

    pub fn convert(&self, other:&UnitSolidAngle) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Steradian(m) => m.clone(),
            _ => Metric::None
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitFrequency {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Hertz(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitFrequency) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Hertz(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitForce {
    Newton(Metric),
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
    fn scale(&self) -> f64 {
        match self {
            Self::Newton(m) => m.scale(),
            _ => 1.0
        }
    }

    fn base(&self) -> f64 { 
        match self {
            Self::Newton(_) => 1.0,
            Self::PoundForce => constants::FC_LBF_TO_N
        }
    }

    pub fn convert(&self, other:&UnitForce) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Newton(m) => m.clone(),
            _ => Metric::None
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitEnergy {
    Joule(Metric),
    GramCalorie(Metric),
    FootPound,
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
    fn scale(&self) -> f64 {
        match self {
            Self::Joule(m) => m.scale(),
            Self::GramCalorie(m) => m.scale(),
            _ => 1.0
        }
    }

    fn base(&self) -> f64 {
        match self {
            Self::Joule(_) => 1.0,
            Self::GramCalorie(_) => constants::EN_CAL_TO_J,
            Self::FootPound => constants::EN_FTLB_TO_J,
            Self::ElectronVolt => constants::EN_EV_TO_J
        }
    }

    pub fn convert(&self, other:&UnitEnergy) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Joule(m) => m.clone(),
            Self::GramCalorie(m) => m.clone(),
            _ => Metric::None
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitPower {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Watt(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitPower) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Watt(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitRadioactivity {
    Becquerel(Metric),
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
    fn scale(&self) -> f64 {
        match self {
            Self::Becquerel(m) => m.scale(),
            _ => 1.0
        }
    }

    fn base(&self) -> f64 {
        match self {
            Self::Becquerel(_) => 1.0,
            Self::Curie => constants::RADIO_C_TO_BQ
        }
    }

    pub fn convert(&self, other:&UnitRadioactivity) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Becquerel(m) => m.clone(),
            _ => Metric::None
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitAbsorbedDose {
    Gray(Metric),
    Roentgen,
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
    fn scale(&self) -> f64 {
        match self {
            Self::Gray(m) => m.scale(),
            _ => 1.0
        }
    }

    fn base(&self) -> f64 {
        match self {
            Self::Gray(_) => 1.0,
            Self::Roentgen => constants::AB_ROE_TO_GY,
            Self::Rad => constants::AB_RAD_TO_GY
        }
    }

    pub fn convert(&self, other:&UnitAbsorbedDose) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Gray(m) => m.clone(),
            _ => Metric::None
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitRadioactivityExposure {
    Sievert(Metric),
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
    fn scale(&self) -> f64 {
        match self {
            Self::Sievert(m) => m.scale(),
            _ => 1.0
        }
    }

    fn base(&self) -> f64 {
        match self {
            Self::Sievert(_) => 1.0,
            Self::Rem => constants::RADEX_REM_TO_SV
        }
    }

    pub fn convert(&self, other:&UnitRadioactivityExposure) -> f64 {
        (self.scale() / self.scale()) * (self.base() / other.base())
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Sievert(m) => m.clone(),
            _ => Metric::None
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitCatalyticActivity {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Katal(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitCatalyticActivity) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Katal(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitSound {
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
    fn scale(&self) -> f64 {
        match self {
            Self::Bel(m) => m.scale(),
        }
    }

    pub fn convert(&self, other:&UnitSound) -> f64 {
        self.scale() / other.scale()
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Bel(m) => m.clone()
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum UnitInformation {
    Bit(Metric),
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

    fn base(&self) -> f64 {
        match self {
            Self::Byte(_) => 1.0,
            Self::Bit(_) => 0.125
        }
    }

    pub fn convert(&self, other:&UnitInformation) -> f64 {
        (self.scale() / other.scale()) * (self.base() / other.base())
    }

    pub fn get_metric(&self) -> Metric {
        match self {
            Self::Bit(m) => m.clone(),
            Self::Byte(m) => m.clone()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Metric::*;
    use super::UnitLength::*;
    use super::UnitTime;
    use super::UnitTime::*;
    use super::UnitMass::*;
    use super::UnitInformation::*;

    #[test]
    fn metric_comparison_1(){
        let x = Meter(None);
        let y = Meter(Kilo);
        let z = Meter(Milli);

        assert_ne!(x, z);
        assert_eq!(y > x, true);
        assert_eq!(x > z, true);
        assert_eq!(y > x && x > z, true);
    }

    #[test]
    fn metric_comparison_2(){
        let y:UnitTime = Second(None);
        let z:UnitTime = Second(Milli);

        assert_ne!(y, z);
        assert_eq!(y > z, true);
    }

    #[test]
    fn metric_comparison_3(){
        let x = Meter(None);
        let y = Meter(Kilo);

        assert_eq!(y.scale() > x.scale(), true);
        assert_eq!(y.scale() - x.scale(), 1000.0-1.0);
    }

    #[test]
    fn conv_test1(){
        let x = Meter(Kilo);
        let y = Mile;

        assert_eq!(y.convert(&x), 1.609344);
    }

    #[test]
    fn conv_test2(){
        let x = Grain;
        let y = Pound;
        let z = Ounce;
        let w = Gram(Milli);
        assert_eq!(y.convert(&x), 7000.0);
        assert_eq!(x.convert(&y), 1.0/7000.0);
        assert_eq!(z.convert(&y), 1.0/16.0);
        assert_eq!(y.convert(&z), 16.0);
        assert_eq!(z.convert(&w), 28349.523125);
        assert_eq!(w.convert(&z), 1.0/28349.523125);
    }

    #[test]
    fn conv_test3(){
        let x = Byte(Giga);
        let y = Byte(Mega);

        assert_eq!(x.convert(&y), 1024.0);
        assert_eq!(y.convert(&x), 1.0/1024.0);
    }

    #[test]
    fn str_test1(){
        let x = Meter(Kilo);
        assert_eq!(x.to_string(), "km");
    }
}