use serde::{Deserialize, Serialize};

/// The Metric scale names
#[derive(Debug, PartialOrd, Eq, PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
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
    #[default]
    None,
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
    Yotta,
}

impl Metric {
    /// Returns the numeric scaling of a given metric prefix
    pub fn scale(&self) -> f64 {
        match self {
            Metric::Yotta => 1000000000000000000000000.0.into(),
            Metric::Zetta => 1000000000000000000000.0.into(),
            Metric::Exa => 1000000000000000000.0.into(),
            Metric::Peta => 1000000000000000.0.into(),
            Metric::Tera => 1000000000000.0.into(),
            Metric::Giga => 1000000000.0.into(),
            Metric::Mega => 1000000.0.into(),
            Metric::Kilo => 1000.0.into(),
            Metric::Hecto => 100.0.into(),
            Metric::Deca => 10.0.into(),
            Metric::None => 1.0.into(),
            Metric::Deci => 0.1.into(),
            Metric::Centi => 0.01.into(),
            Metric::Milli => 0.001.into(),
            Metric::Micro => 0.000001.into(),
            Metric::Nano => 0.000000001.into(),
            Metric::Pico => 0.000000000001.into(),
            Metric::Femto => 0.000000000000001.into(),
            Metric::Atto => 0.000000000000000001.into(),
            Metric::Zepto => 0.000000000000000000001.into(),
            Metric::Yocto => 0.000000000000000000000001.into(),
        }
    }

    /// Returns the string representation of the metric prefix
    pub fn as_str(&self) -> &str {
        match self {
            Metric::Yotta => "Y",
            Metric::Zetta => "Z",
            Metric::Exa => "E",
            Metric::Peta => "P",
            Metric::Tera => "T",
            Metric::Giga => "G",
            Metric::Mega => "M",
            Metric::Kilo => "k",
            Metric::Hecto => "h",
            Metric::Deca => "da",
            Metric::None => "",
            Metric::Deci => "d",
            Metric::Centi => "c",
            Metric::Milli => "m",
            Metric::Micro => "μ",
            Metric::Nano => "n",
            Metric::Pico => "p",
            Metric::Femto => "f",
            Metric::Atto => "a",
            Metric::Zepto => "z",
            Metric::Yocto => "y",
        }
    }
}