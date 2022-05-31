# V2
V2 is a library with the aim of assisting other programs that use a lot of math equations to integrate units and unit interactions into a single type. 

V2 parses strings given by a user and turns the parsed string into a 'Value' type. This type can be used in a similar way as to a regular ```<f64>``` type. It maintains the value given, as well as the measurement type given. 

When used in equations, it will automatically be updated wither within itself or the new construction will contain the updated unit types. 

## Table of Contents
- [V2](#v2)
  - [Table of Contents](#table-of-contents)
  - [Example](#example)
  - [Unit Support](#unit-support)
  - [Conversions](#conversions)
  - [Constants](#constants)

## Example
To create a new value:
```rust
let t:Value = Value::new(1.2, "seconds").unwrap();
```

And to use it:
```rust
let s:Value = Value::new(5.6, "m/s").unwrap();
let d:Value = t * s;
```
d will contain the value: `6.72 m`

You can also conduct other operations:
```rust
let mut m:Value = Value::new(60.0, "liters").unwrap();
m += 40.0;
```
```rust
uet mut r:Value = Value::new(2.0, "radians").unwrap();
let n:f64 = r.sin();
```

## Unit Support

The project supports all official SI units as listed by the National Institute of Standards and Technology (NIST) and many units listed by the General Conference on Weights and Measures (CGPM). *Some* American Imperial Units are also included. 

Metric units, unless otherwise specified, will support full Yocto-Yotta metric ranges. 

Lengths
```
Meter
Inch
Foot
Yard
Mile
Astronomical Unit
Parsec
Light Year
Ångström
```
Time
```
Second
Minute
Hour
Day
```
Mass
```
Gram
Grain
Ounce
Pound
```
Electric Current
```
Ampere
```
Electric Charge
```
Coulomb
```
Electric Potential
```
Volt
```
Electric Conductance
```
Siemens
```
Capacitance
```
Farad
```
Resistance
```
Ohm
```
Inductance
```
Henry
```
Magnetic Flux
```
Weber
```
Magnetic Flux Density
```
Tesla
```
Temperature
```
Celsius
Fahrenheit
Kelvin
```
Substance
```
Mole
```
Luminous Intensity
```
Candela
```
Luminous Flux
```
Lumen
```
Illuminance
```
Lux
```
Volume
```
Liter
```
Pressure
```
Pascal
Bar
Torr
Hgmm
Hgin
Atm
Psi
```
Angle
```
Degree
Radian
Minute of Angle
Milliradian
```
Frequency
```
Hertz
```
Force
```
Newton
Pound Force
```
Energy
```
Joule
Calorie
Kilocalorie
Foot Pound
Electron Volt
```
Power
```
Watt
```
Radioactivity
```
Becquerel
Curie
```
Absorbed Dosage of Radiation
```
Gray
Röntgen
Rad
```
Radioactivity Exposure
```
Sievert
Rem
```
Catalytic Activity
```
Katal
```
Sound
```
Bel
```
Information
```
Bit -> Partial metric support
Byte -> Partial metric support
```

## Conversions
All within their given measurement type will be able to be converted to eachother. Values with multiple types, in most cases, can be converted to their compatable types. 

Example converting feet into meters:
```rust
let mut m:Value = Value::new(3.2, "feet").unwrap();
m.convert("m");
```
There is also direct syntax for this feature:
```rust
let mut m:Value = Value::new(5.9, "km/h").unwrap();
m >>= "m";
```
If you require better runtime efficiency at the cost of specifying each unit type:
```rust
use v2::units::unit_types::{UnitLength, UnitTime};
let mut s:Value = Value::new(5.3, "mph").unwrap();
s >>= (Metric::Kilo, UnitLength::Meter);
s >>= UnitTime::Second;
```
Temperature cannot be converted to another unit if it has other units (like mass) within the value. 

Units cannot be converted between disparate types, although there are some exceptions. Liters and meters^3 is one such example. 

## Constants
There are also provided constants for easier usage. 
```
Absolute Zero - K
```
```
Avogadro's Number - mol^-1
```
```
Faraday Constant
```
```
Atomic Mass Constant
```
```
Molar Gas Constant - J/(K*mol)
```
```
Coulombs Constant
```
```
The Speed of light
```
```
Boltzmann Constant
```
```
The gravity of Earth - m/s^2
```
```
Electron Charge 
```
```
Rydberg Constant
```
```
Plank's Constant
```