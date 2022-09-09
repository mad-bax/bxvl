# V3
V3 is a library with the aim of assisting other programs that use a lot of math equations to integrate units and unit interactions into a single type. 

V3 parses strings given by a user and turns the parsed string into a 'Value' type. This type can be used in a similar way as to a regular ```<f64>``` type. It maintains the value given, as well as the measurement type given. 

When used in equations, it will automatically be updated wither within itself or the new construction will contain the updated unit types. 

## Table of Contents
- [V3](#v3)
  - [Table of Contents](#table-of-contents)
  - [Example](#example)
  - [Unit Support](#unit-support)
  - [Method Support](#method-support)
  - [Derrived Units](#derrived-units)
  - [Conversions](#conversions)
  - [Constants](#constants)

## Examples
To create a new value:
```rust
let t:Value = Value::new(1.2, "seconds").unwrap();
```

And to use it:
```rust
let s:Value = Value::new(5.6, "m/s").unwrap();
let d:Value = t * s;
```
d will be ```Result<Value>``` containing: `6.72 m`

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
```
Yotta => Y
Zetta => Z
Exa   => E
Peta  => P
Tera  => T
Giga  => G
Mega  => M
Kilo  => k
Hecto => h
Deca  => da
Deci  => c
Milli => m
Micro => u, μ
Nano  => n
Pico  => p
Femto => f
Atto  => a
Zepto => z
Yocto => y
```

Lengths
```
Meter             => m (kph; for kilometers per hour)
Inch              => in, inch, inches
Foot              => ft, feet
Yard              => yd, yds
Mile              => mile, miles (mph; for miles per hour)
Astronomical Unit => AU
Parsec            => pc
Light Year        => lightyear, lightyears, lyr
Ångström          => angstrom, angstroms, Å
```
Time
```
Second => s
Minute => min, minute, minutes 
Hour   => h, hour, hours
Day    => d, day, days
```
Mass
```
Gram  => g
Grain => gr, grain, grains
Ounce => oz, ounce, ounces
Pound => lb, lbs, pounds
```
Electric Current
```
Ampere => A
```
Electric Charge
```
Coulomb => C
```
Electric Potential
```
Volt => V
```
Electric Conductance
```
Siemens => S
```
Capacitance
```
Farad => F, farad, farads
```
Resistance
```
Ohm => Ω, O
```
Inductance
```
Henry => H
```
Magnetic Flux
```
Weber => Wb
```
Magnetic Flux Density
```
Tesla => T
```
Temperature
```
Celsius    => c, °c, °C
Fahrenheit => f, °f, °F
Kelvin     => K
```
Substance
```
Mole => mol
```
Luminous Intensity
```
Candela => cd
```
Luminous Flux
```
Lumen => lm
```
Illuminance
```
Lux => lx
```
Volume
```
Liter => l
```
Pressure
```
Pascal => Pa
Bar    => bar
Torr   => torr
mmHg   => mmHg
inHg   => inHg
Atm    => ATM, atm
Psi    => PSI, psi
```
Angle
```
Degree          => °, degree, degrees
Radian          => rad, radian, radians
milliradian     => mil, mils, MIL
Minute of Angle => moa, MOA
```
Frequency
```
Hertz => Hz
```
Force
```
Newton      => N
Pound Force => lbfr, lbsfr, poundforce, poundsforce
```
Energy
```
Joule         => J
Calorie       => cal (Cal; for kilocalorie)
Foot Pound    => ftlb, ftlbs, footpound, footpounds
Electron Volt => eV
```
Power
```
Watt => W
```
Radioactivity
```
Becquerel => Bq
Curie     => Ci
```
Absorbed Dosage of Radiation
```
Gray    => Gy
Röntgen => R
Rad     => rads, Rads
```
Radioactivity Exposure
```
Sievert => Sv
Rem     => rem, Rem
```
Catalytic Activity
```
Katal => kat
```
Sound
```
Bel => B
```
Information (Kilo through Yotta metric support. Base 2, metric scaling applied, i.e. Gib not Gb)
```
Bit  => bits
Byte => b, byte, bytes
```

Note that some unit strings like ```eV``` could be indended to be ```Exa-Volts``` or ```Electron Volts```. The library is case sensitive and will default to the 'least complex' unit that matches. So ```Electron Volts``` will be the parsed result. To get ```Exa-Volts```, the user must properly specify ```EV``` or simply ```V``` for volts and then convert to the ```Exa``` metric scaler. 

## Method Support
Values provide similar functionality to many functions that are available to other units such as i32, u64, f32, f64 etc. 
```rust
let m:Value = Value::new(f64::NAN, "feet").unwrap();
if m.is_nan() {
  panic!("Cannot have NAN value!");
}
```

## Derrived Units
Many of the SI units are derrived from other base units. When using the values to conduct arithmetic operations, values can be explicity asked to be 'complex' or 'reduced'. 

Making a complex value means combining different types into a new type.
```rust
let m:Value = Value::new(2.5, "kg").unwrap();
let acc:Value = Value::new(9.81, "m/s^2").unwrap();

let f1:Value = m*acc;
let f2:Value = (m*acc).complex().unwrap();
```
Variable `f1` will be `24.525 kg*m/s^2` whereas `f2` will be `24.525 N`

Reducing a value means setting a value to its derrived units.
```rust
let mut f:Value = Value::new(24.525, "N").unwrap();
f.reduce("kg*m/s^2").unwrap();
```

Variable `f` will be `24.525 kg*m/s^2`

This behavior is explicit and must be called by the user.

However functions like `.is_force()` will return `true` for both `kg*m/s^2` and `N`. 

## Conversions
All within their given measurement type will be able to be converted to eachother. Values with multiple types, in most cases, can be converted to their compatable types. 

Example converting feet into meters:
```rust
let mut m:Value = Value::new(3.2, "feet").unwrap();
m.convert("m");
```
You can use other Values for conversion:
```rust
let m:Value = Value::new(1.2, "yards").unwrap();
let n:Value = Value::new(1.0, "m").unwrap();

let k = (m >> n).unwrap();
```
There is also direct syntax for this feature:
```rust
let mut m:Value = Value::new(5.9, "km/h").unwrap();
m >>= "m";
```
If you require better runtime efficiency at the cost of specifying each unit type:
```rust
use v3::units::unit_types::{UnitLength, UnitTime};
let mut s:Value = Value::new(5.3, "mph").unwrap();
s >>= UnitLength::Meter(Metric::Kilo);
s >>= UnitTime::Second(Metric::None);
```
Temperature cannot be converted to another unit if it has other units (like mass) within the value. 

Units cannot be converted between disparate types, although there are some exceptions. `ml` to `mm^3` is one such example of volume to a cubed length.

## Constants
There are also provided constants for easier usage. 
```
Absolute Zero         - K
```
```
Avogadro's Number     - mol^-1
```
```
Faraday Constant      - C/mol
```
```
Atomic Mass Constant  - kg
```
```
Molar Gas Constant    - J/(K*mol)
```
```
Coulombs Constant     - 1/mol
```
```
The Speed of light    - m/s
```
```
Boltzmann Constant    - J/K
```
```
The gravity of Earth  - m/s^2
```
```
Newtonian Gravitation - m^3/(kg*s^2)
```
```
Electron Charge       - C
```
```
Rydberg Constant      - 1/m
```
```
Plank's Constant      - J/Hz
```
```
Vacuum Permitivity    - F/m
```