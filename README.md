# V3
V3 is a scientific unit type library that allows variables to dynamically keep track of different unit measurements. As these variables are defined and used, they may be converted to other units, metricly scaled, arithmetically combined with others, build new units, and divided into their base units.

## Table of Contents
- [V3](#v3)
  - [Table of Contents](#table-of-contents)
  - [Unit Support](#unit-support)
  - [Examples](#examples)
  - [Method Support](#method-support)
  - [Derrived Units](#derrived-units)
  - [Conversions](#conversions)
  - [Constants](#constants)
  - [Future Work](#futurework)

## Unit Support

The project supports all base SI units as listed by the National Institute of Standards and Technology (NIST) and many units listed by the General Conference on Weights and Measures (CGPM). *Some* American Imperial Units are also supported.

`[m]` indicates that the unit supports `Metric` prefixes.

Lengths
```#[ignore]
Meter             [m] => m (kph; for kilometers per hour)
Inch                  => in, inch, inches
Foot                  => ft, feet
Yard                  => yd, yds
Mile                  => mile, miles (mph; for miles per hour)
Astronomical Unit     => AU
Parsec                => pc
Light Year            => lightyear, lightyears, lyr
Ångström              => angstrom, angstroms, Å
```
Time
```#[ignore]
Second            [m] => s
Minute                => min, minute, minutes 
Hour                  => h, hr, hour, hours
Day                   => d, day, days
```
Mass
```#[ignore]
Gram              [m] => g
Grain                 => gr, grain, grains
Ounce                 => oz, ounce, ounces
Pound                 => lb, lbs, pounds
```
Electric Current
```#[ignore]
Ampere            [m] => A
```
Electric Charge
```#[ignore]
Coulomb           [m] => C
```
Electric Potential
```#[ignore]
Volt              [m] => V
```
Electric Conductance
```#[ignore]
Siemens           [m] => S
```
Capacitance
```#[ignore]
Farad             [m] => F, farad, farads
```
Resistance
```#[ignore]
Ohm               [m] => Ω, O
```
Inductance
```#[ignore]
Henry             [m] => H
```
Magnetic Flux
```#[ignore]
Weber             [m] => Wb
```
Magnetic Flux Density
```#[ignore]
Tesla             [m] => T
```
Temperature
```#[ignore]
Celsius               => c, °c, °C
Fahrenheit            => f, °f, °F
Kelvin                => K
```
Substance
```#[ignore]
Mole              [m] => mol
```
Luminous Intensity
```#[ignore]
Candela           [m] => cd
```
Luminous Flux
```#[ignore]
Lumen             [m] => lm
```
Illuminance
```#[ignore]
Lux               [m] => lx
```
Volume
```#[ignore]
Liter             [m] => l
```
Pressure
```#[ignore]
Pascal            [m] => Pa
Bar               [m] => bar
Torr                  => torr
mmHg                  => mmHg
inHg                  => inHg
Atm                   => ATM, atm
Psi                   => PSI, psi
```
Angle
```#[ignore]
Degree                => °, degree, degrees
Radian            [m] => rad, radian, radians
milliradian       [m] => mil, mils, MIL
Minute of Angle       => moa, MOA
```
Frequency
```#[ignore]
Hertz             [m] => Hz
```
Force
```#[ignore]
Newton            [m] => N
Pound Force           => lbfr, lbsfr, poundforce, poundsforce
```
Energy
```#[ignore]
Joule             [m] => J
Calorie           [m] => cal (Cal; for kilocalorie)
Foot Pound            => ftlb, ftlbs, footpound, footpounds
Electron Volt         => eV
```
Power
```#[ignore]
Watt              [m] => W
```
Radioactivity
```#[ignore]
Becquerel         [m] => Bq
Curie                 => Ci
```
Absorbed Dosage of Ionizing Radiation
```#[ignore]
Gray              [m] => Gy
Röntgen               => R
Rad                   => rads, Rads
```
Equivalent Dosage of Ionizing Radiation
```#[ignore]
Sievert           [m] => Sv
Rem                   => rem, Rem
```
Catalytic Activity
```#[ignore]
Katal             [m] => kat
```
Sound
```#[ignore]
Bel               [m] => B
```
Information
```#[ignore]
Bit  [m base 2 : Kilo - Yotta] => bits
Byte [m base 2 : Kilo - Yotta] => b, byte, bytes
```

Metric prefixes
```#[ignore]
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

Note that some unit strings like `eV` could be indended to be `Exa-Volts` or `Electron Volts`. The library is case sensitive and will default to the 'least complex' unit that matches. So `Electron Volts` will be the parsed result. To get `Exa-Volts`, the user must properly specify `EV` or simply `V` for volts and then convert to the `Exa` metric scaler. 

## Examples
Creating `Value`s:
```rust
use v3::value;
use v3::values::Value;
use v3::units::{Metric, UnitTime, UnitMass, UnitLength};

// Slowest
let v1:Value = "22.3 kg*m/s^2".parse::<Value>().unwrap();

// Slow
let v2:Value = value!(22.3, "kg*m/s^2");

// Average
let v3:Value = Value::new(22.3, "kg*m/s^2").unwrap();

// Fastest
let v4:Value = 22.3
  ^ UnitTime::Second(Metric::None)
  ^ UnitTime::Second(Metric::None)
  | UnitMass::Gram(Metric::Kilo)
  | UnitLength::Meter(Metric::None);
```

Using `Values`:
```rust
use v3::values::Value;
use v3::units::{Metric, UnitTime, UnitLength};

let time:Value = 3.4 | UnitTime::Second(Metric::None);
let dist:Value = 10.3 | UnitLength::Meter(Metric::None);

let speed:Value = dist/time;
assert!(speed >= 3.0293);
```

## Method Support
Values provide similar functionality to many functions that are available to other units such as i32, f32, f64 etc. 
```rust
use v3::values::Value;
use v3::units::{Metric, UnitLength};

let m:Value = Value::new(f64::NAN, "feet").unwrap();
if m.is_nan() {
  println!("Our value is not a number!");
}

let a:Value = 1.4 | UnitLength::Meter(Metric::None);
let r:Value = a.sin(); // Value is in radians
```

## Derrived Units
Many of the SI units are derrived from other base units. When using the values to conduct arithmetic operations, values can be explicity asked to be 'complex' or 'reduced'. 

Making a complex value means combining different types into a new type.
```rust
use v3::values::Value;

let m:Value = Value::new(2.5, "kg").unwrap();
let acc:Value = Value::new(9.81, "m/s^2").unwrap();

let f1:Value = m*acc;
let f2:Value = (m*acc).complex().unwrap();
```
Variable `f1` will be `24.525 kg*m/s^2` whereas `f2` will be `24.525 N`

Reducing a value means setting a value to its derrived units.
```rust
use v3::values::Value;

let mut f:Value = Value::new(24.525, "N").unwrap();

f.reduce("kg*m/s^2").unwrap();
```
Variable `f` will be `24.525 kg*m/s^2`

This behavior is explicit and must be called by the user.

However functions like `.is_force()` will return `true` for both `kg*m/s^2` and `N`. 

## Conversions
All `Value`s within their given measurement type will be able to be converted to each other. Values with multiple types, in most cases, can be converted to their compatable types. 

Example converting feet into meters:
```rust
use v3::values::Value;

let mut m:Value = Value::new(3.2, "feet").unwrap();

m.convert("m").unwrap();
```

There is also direct syntax for this feature:
```rust
use v3::values::Value;

let mut m:Value = Value::new(5.9, "km/hr").unwrap();

m >>= "m/s";
```

You can use other Values for conversion:
```rust
use v3::values::Value;

let m:Value = Value::new(1.2, "yards").unwrap();
let n:Value = Value::new(1.0, "m").unwrap();

let k:Value = (m >> n).unwrap();
```

The types can also be directly used: (The fastest method)
```rust
use v3::values::Value;
use v3::units::{Metric, UnitLength, UnitTime};

let mut m:Value = Value::new(5.9, "kph").unwrap();

if m.is_velocity() {
  m >>= UnitLength::Meter(Metric::None);
  m >>= UnitTime::Second(Metric::None);
} else {
  panic!();
}
```
Temperature cannot be converted to another unit if it has other units (like mass) within the value. 

Units cannot be converted between disparate types, although there are some exceptions. `ml` to `mm^3` is one such example of volume to a cubed length.

## Constants
There are also provided constants for easier usage. 
```#[ignore]
Absolute Zero         - K
```
```#[ignore]
Avogadro's Number     - mol^-1
```
```#[ignore]
Faraday Constant      - C/mol
```
```#[ignore]
Atomic Mass Constant  - kg
```
```#[ignore]
Molar Gas Constant    - J/(K*mol)
```
```#[ignore]
Coulombs Constant     - 1/mol
```
```#[ignore]
The Speed of light    - m/s
```
```#[ignore]
Boltzmann Constant    - J/K
```
```#[ignore]
The gravity of Earth  - m/s^2
```
```#[ignore]
Newtonian Gravitation - m^3/(kg*s^2)
```
```#[ignore]
Electron Charge       - C
```
```#[ignore]
Rydberg Constant      - 1/m
```
```#[ignore]
Plank's Constant      - J/Hz
```
```#[ignore]
Vacuum Permitivity    - F/m
```

## Future Work
V3 can and is intended to be improved with some of these goals in mind:
- Support for `<f32>`, `<i32>`, `<i64>`, and `<i128>` numeric types
- Numerator and Denominator numeric variables to ensure floating point accuracy
- Significant digit considerations
- Equation definitions, which can expect a specific `Value` type
- More Imperial measurement support 🤢
- speed speed speed