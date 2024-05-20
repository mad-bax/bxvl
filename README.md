<p align="center">
  <img src="./images/v3_logo.png">
</p>

V3 is a scientific unit-type library that allows variables to dynamically keep track of different unit measurements. As these variables are defined and used, they may be converted to other units, metrically scaled, arithmetically combined with others, build new units, and divided into their base units.

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Examples](#examples)
- [Method Support](#method-support)
- [Derived Units](#derived-units)
  - [Unit Checking](#unit-checking)
- [Conversions](#conversions)
- [Constants](#constants)
- [Unit Support](#unit-support)
  - [Lengths](#lengths)
  - [Time](#time)
  - [Mass](#mass)
  - [Electric Current](#electric-current)
  - [Electric Charge](#electric-charge)
  - [Electric Potential](#electric-potential)
  - [Electric Conductance](#electric-conductance)
  - [Electric Capacitance](#electric-capacitance)
  - [Electric Resistance](#electric-resistance)
  - [Electric Inductance](#electric-inductance)
  - [Magnetic Flux](#magnetic-flux)
  - [Magnetic Flux Density](#magnetic-flux-density)
  - [Thermal Temperature](#thermal-temperature)
  - [Substance](#substance)
  - [Luminous Intensity](#luminous-intensity)
  - [Luminous Flux](#luminous-flux)
  - [Illuminance](#illuminance)
  - [Spatial Volume](#spatial-volume)
  - [Pressure](#pressure)
  - [Geometric Angle](#geometric-angle)
  - [Frequency](#frequency)
  - [Force](#force)
  - [Energy](#energy)
  - [Power](#power)
  - [Radioactivity](#radioactivity)
  - [*Absorbed* Dosage of Ionizing Radiation](#absorbed-dosage-of-ionizing-radiation)
  - [*Equivalent* Dosage of Ionizing Radiation](#equivalent-dosage-of-ionizing-radiation)
  - [Catalytic Activity](#catalytic-activity)
  - [Sound Intensity](#sound-intensity)
  - [Information](#information)
  - [Special Unit Keywords](#special-unit-keywords)
  - [Metric Prefix Identifiers](#metric-prefix-identifiers)
- [Future Work](#future-work)

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
  / UnitTime::Second(Metric::None)
  / UnitTime::Second(Metric::None)
  * UnitMass::Gram(Metric::Kilo)
  * UnitLength::Meter(Metric::None);
```

Using `Values`:

```rust
use v3::values::Value;
use v3::units::{Metric, UnitTime, UnitLength};

let time:Value = 3.4 * UnitTime::Second(Metric::None);
let dist:Value = 10.3 * UnitLength::Meter(Metric::None);

let speed:Value = dist/time;
assert!(speed >= 3.0293);
assert!(speed.is_velocity());
```

## Method Support

Values provide similar functionality to many functions that are available to other units such as `i32`, `f32`, `f64` etc.

```rust
use v3::values::Value;
use v3::units::{Metric, UnitLength};

let m:Value = Value::new(f64::NAN, "feet").unwrap();
if m.is_nan() {
  println!("Our value is not a number!");
}

let a:Value = 1.4 * UnitLength::Meter(Metric::None);
let r:Value = a.sin();
assert!(r.is_radians());
```

## Derived Units

Many of the SI units are derived from other base units. When using the values to conduct arithmetic operations, values can be explicitly asked to be 'complex' or 'reduced'.

Making a complex value means combining different types into a new type.

```rust
use v3::values::Value;

let m:Value = Value::new(2.5, "kg").unwrap();
let acc:Value = Value::new(9.81, "m/s^2").unwrap();

let f1:Value = m*acc;
let f2:Value = (m*acc).complex().unwrap();
assert!(f1.is_force() && f2.is_force());
assert!(f1.val == f2.val);
```

Variable `f1` will be `24.525 kg*m/s^2` whereas `f2` will be `24.525 N`

Reducing a value means setting a value to its derived units.

```rust
use v3::values::Value;

let mut f:Value = Value::new(24.525, "N").unwrap();

assert!(f.is_force());
f.reduce("kg*m/s^2").unwrap();
assert!(f.is_force());
```

Variable `f` will be `24.525 kg*m/s^2`

This behavior is explicit and must be called by the user.

### Unit Checking

V3 provides functions like `.is_force()` which will return `true` for both `kg*m/s^2` and `N`. Function support includes all of the base [unit types](#unit-support) as well as extra unit combinations, such as `.is_yank()` ([Force](#force)/[Time](#time)) and `.is_torque()` ([Force](#force)\*[Length](#lengths) *or* [Energy](#energy)/[Angle](#geometric-angle)) as two examples.

## Conversions

All `Value`s within their given measurement type will be able to be converted to each other. Values with multiple types, in most cases, can be converted to their compatible types.

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

The types can also be directly used: (The fastest conversion method)

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

Units cannot be converted between disparate types, although there are some exceptions.

| Exceptions |                      |                        |
| ---------- | -------------------- | ---------------------- |
| Period     | Time period (`1/s`)  | Frequency (`Hz`)       |
| Volume     | Cubic length (`m^3`) | Specific volume (`ml`) |

These exceptions are valid conversion so long as they are the *only* units within a `Value`. This is to avoid conversion scenarios where `Value`s produce (or are created with) neutralizing units, e.g. `mm^3/ml`, which is 'unitless'. Therefore, `m/s` cannot be converted to `m*kHz` and `m^3/N` cannot be converted to `l/N`.

## Constants

Some constants are provided for ease of use:

| Name                                       | `f64` numeric Value     | Units              |
| ------------------------------------------ | ----------------------- | ------------------ |
| Absolute Zero                              | `0.`                    | $K$                |
| Avogadro's Number                          | `6.022_140_76e23`       | $1 \over mol$      |
| Faraday Constant                           | `96_485.332_123_310_01` | $C \over mol$      |
| Atomic Mass Constant                       | `1.660_539_066_60e-27`  | $kg$               |
| Molar Gas Constant                         | `8.314_462_1`           | $J \over K*mol$    |
| Coulomb's Constant                         | `8.987_551`             | $1 \over mol$      |
| The Speed of Light                         | `299_792_458.0`         | $m \over s$        |
| Boltzmann Constant                         | `1.380_649e-23`         | $J \over K$        |
| Earth's Average Gravitational Acceleration | `9.806_65`              | $m \over s^2$      |
| Newtonian Constant of Gravitation          | `6.673_015e-11`         | $m^3 \over kg*s^2$ |
| Charge of an Electron                      | `1.602_176_634e-19`     | $C$                |
| Rydberg Constant                           | `10_973_731.568_539`    | $1 \over m$        |
| Plank's Constant                           | `6.626_070_15e-34`      | $J \over Hz$       |
| Vacuum Permittivity                        | `8.854_187_812_8e-12`   | $F \over m$        |

## Unit Support

The project supports all base SI units as listed by the National Institute of Standards and Technology (NIST) and many units listed by the General Conference on Weights and Measures (CGPM). *Some* American Imperial Units are also supported.

### Lengths

| Unit              | Metric Prefixing Support | Base Conversion Factor              | Unit string                |
| ----------------- | ------------------------ | ----------------------------------- | -------------------------- |
| Meter             | &check;                  | `1.0 m`                             | `m`                        |
| Inch              |                          | `0.025_4 m`                         | `in`<br/>`inch`[`es`]      |
| Foot              |                          | `0.304_8 m`                         | `ft`<br/>`feet`<br/>`foot` |
| Yard              |                          | `0.914_4 m`                         | `yd`[`s`]<br/>`yard`[`s`]  |
| Mile              |                          | `1_609.344 m`                       | `mile`[`s`]                |
| Astronomical Unit |                          | `149_569_870_700.0 m`               | `AU`                       |
| Parsec            | &check;                  | `(648_000.0/π)*149_569_870_700.0 m` | `pc`                       |
| Light Year        | &check;                  | `9_460_730_472_580_800.0 m`         | `lyr`                      |
| Ångström          |                          | `0.000_000_000_1 m`                 | `Å`<br/>`angstrom`[`s`]    |

### Time

| Unit   | Metric Prefixing Support | Base Conversion Factor | Unit string              |
| ------ | ------------------------ | ---------------------- | ------------------------ |
| Second | &check;                  | `1.0 s`                | `s`                      |
| Minute |                          | `60.0 s`               | `min`<br/>`minute`[`s`]  |
| Hour   |                          | `3_600.0 s`            | `h`[`r`]<br/>`hour`[`s`] |
| Day    |                          | `86_400.0 s`           | `d`<br/>`day`[`s`]       |

### Mass

|Unit |Metric Prefixing Support|Base Conversion Factor|Unit string           |
|-----|------------------------|----------------------|----------------------|
|Gram |&check;                 |`1.0 g`               |`g`                   |
|Grain|                        |`453.592_37/7_000.0 g`|`gr`<br/>`grain`[`s`] |
|Ounce|                        |`453.592_37/16.0 g`   |`oz`<br/>`ounce`[`s`] |
|Pound|                        |`453.592_37 g`        |`lb`[`s`]<br/>`pounds`|

### Electric Current

| Unit   | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ------ | ------------------------ | ---------------------- | ----------- |
| Ampere | &check;                  | `1.0 A`                | `A`         |

### Electric Charge

| Unit    | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ------- | ------------------------ | ---------------------- | ----------- |
| Coulomb | &check;                  | `1.0 C`                | `C`         |

### Electric Potential

| Unit | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ---- | ------------------------ | ---------------------- | ----------- |
| Volt | &check;                  | `1.0 V`                | `V`         |

### Electric Conductance

| Unit    | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ------- | ------------------------ | ---------------------- | ----------- |
| Siemens | &check;                  | `1.0 S`                | `S`         |

### Electric Capacitance

| Unit  | Metric Prefixing Support | Base Conversion Factor | Unit string          |
| ----- | ------------------------ | ---------------------- | -------------------- |
| Farad | &check;                  | `1.0 F`                | `F`<br/>`farad`[`s`] |

### Electric Resistance

| Unit | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ---- | ------------------------ | ---------------------- | ----------- |
| Ohm  | &check;                  | `1.0 Ω`                | `Ω`<br/>`O` |

### Electric Inductance

| Unit  | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ----- | ------------------------ | ---------------------- | ----------- |
| Henry | &check;                  | `1.0 H`                | `H`         |

### Magnetic Flux

| Unit  | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ----- | ------------------------ | ---------------------- | ----------- |
| Weber | &check;                  | `1.0 Wb`               | `Wb`        |

### Magnetic Flux Density

| Unit  | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ----- | ------------------------ | ---------------------- | ----------- |
| Tesla | &check;                  | `1.0 T`                | `T`         |

### Thermal Temperature

| Unit       | Metric Prefixing Support | Base Conversion Factor | Unit string           |
| ---------- | ------------------------ | ---------------------- | --------------------- |
| Celsius    | **[Future Support]**     | `c-273.15 K`           | `c`<br/>`°c`<br/>`°C` |
| Fahrenheit |                          | `(f-32.0)/1.8 c`       | `f`<br/>`°f`<br/>`°F` |
| Kelvin     | &check;                  | `1.0 K`                | `K`                   |

### Substance

| Unit | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ---- | ------------------------ | ---------------------- | ----------- |
| Mole | &check;                  | `1.0 mol`              | `mol`       |

### Luminous Intensity

| Unit    | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ------- | ------------------------ | ---------------------- | ----------- |
| Candela | &check;                  | `1.0 cd`               | `cd`        |

### Luminous Flux

| Unit  | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ----- | ------------------------ | ---------------------- | ----------- |
| Lumen | &check;                  | `1.0 lm`               | `lm`        |

### Illuminance

| Unit | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ---- | ------------------------ | ---------------------- | ----------- |
| Lux  | &check;                  | `1.0 lx`               | `lx`        |

### Spatial Volume

| Unit  | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ----- | ------------------------ | ---------------------- | ----------- |
| Liter | &check;                  | `1.0 l`                | `l`         |

### Pressure

| Unit                   | Metric Prefixing Support | Base Conversion Factor | Unit string     |
| ---------------------- | ------------------------ | ---------------------- | --------------- |
| Pascal                 | &check;                  | `1.0 Pa`               | `Pa`            |
| Bar                    | &check;                  | `100_000.0 Pa`         | `bar`           |
| Torr                   |                          | `101_325.0/760.0 Pa`   | `torr`          |
| mmHg                   |                          | `133.322_387_415 Pa`   | `mmHg`          |
| inHg                   |                          | `3_386.388_666_6 Pa`   | `inHg`          |
| Atmospheres            |                          | `101_325.0 Pa`         | `ATM`<br/>`atm` |
| Pounds per square inch |                          | `6894.757 Pa`          | `PSI`<br/>`psi` |

### Geometric Angle

| Unit            | Metric Prefixing Support | Base Conversion Factor | Unit string             |
| --------------- | ------------------------ | ---------------------- | ----------------------- |
| Degree          |                          | `π/180.0 rad`          | `°`<br/>`degree`[`s`]   |
| Radian          | &check;                  | `1.0 rad`              | `rad`<br/>`radian`[`s`] |
| Milliradian     | &check;                  | `1_000.0 rad`          | `mil`[`s`]<br/>`MIL`    |
| Minute of Angle |                          | `π/10_800.0 rad`       | `moa`<br/>`MOA`         |

### Frequency

| Unit  | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ----- | ------------------------ | ---------------------- | ----------- |
| Hertz | &check;                  | `1.0 Hz`               | `Hz`        |

### Force

| Unit        | Metric Prefixing Support | Base Conversion Factor  | Unit string                                           |
| ----------- | ------------------------ | ----------------------- | ----------------------------------------------------- |
| Newton      | &check;                  | `1.0 N`                 | `N`                                                   |
| Pound Force |                          | `4.448_221_615_260_5 N` | `lbfr`<br/>`lbsfr`<br/>`poundforce`<br/>`poundsforce` |

### Energy

| Unit          | Metric Prefixing Support | Base Conversion Factor | Unit string                      |
| ------------- | ------------------------ | ---------------------- | -------------------------------- |
| Joule         | &check;                  | `1.0 J`                | `J`                              |
| Calorie       | &check;                  | `4.184 J`              | `cal`                            |
| Foot pound    |                          | `1.355_818 J`          | `ftlb`[`s`]<br/>`footpound`[`s`] |
| Electron Volt | &check;                  | `1.6021_766_34e-19 J`  | `eV`                             |

### Power

| Unit | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ---- | ------------------------ | ---------------------- | ----------- |
| Watt | &check;                  | `1.0 W`                | `W`         |

### Radioactivity

| Unit      | Metric Prefixing Support | Base Conversion Factor | Unit string |
| --------- | ------------------------ | ---------------------- | ----------- |
| Becquerel | &check;                  | `1.0 Bq`               | `Bq`        |
| Curie     |                          | `37_000_000_000.0 Bq`  | `Ci`        |

### *Absorbed* Dosage of Ionizing Radiation

| Unit    | Metric Prefixing Support | Base Conversion Factor | Unit string       |
| ------- | ------------------------ | ---------------------- | ----------------- |
| Gray    | &check;                  | `1.0 Gy`               | `Gy`              |
| Röntgen |                          | `0.01 Gy`              | `R`               |
| Rad     |                          | `1.0/114.025 Gy`       | `rads`<br/>`Rads` |

### *Equivalent* Dosage of Ionizing Radiation

| Unit    | Metric Prefixing Support | Base Conversion Factor | Unit string     |
| ------- | ------------------------ | ---------------------- | --------------- |
| Sievert | &check;                  | `1.0 Sv`               | `Sv`            |
| Rem     |                          | `0.01 Sv`              | `rem`<br/>`Rem` |

### Catalytic Activity

| Unit  | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ----- | ------------------------ | ---------------------- | ----------- |
| Katal | &check;                  | `1.0 kat`              | `kat`       |

### Sound Intensity

| Unit | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ---- | ------------------------ | ---------------------- | ----------- |
| Bel  | &check;                  | `1.0 B`                | `B`         |

### Information

:warning: Metric scaling is in base **2**

*i.e.* `kb` &rarr; `1024 bytes`, *not* `1000 bytes`

| Unit | Metric Prefixing Support | Base Conversion Factor | Unit string         |
| ---- | ------------------------ | ---------------------- | ------------------- |
| Bit  | `Kilo` - `Quetta`        | `8.0 bits == 1.0 byte` | `bits`              |
| Byte | `Kilo` - `Quetta`        | `1.0 bytes`            | `b`<br/>`byte`[`s`] |

### Special Unit Keywords

| Unit                | Metric  | Unit string | Equivalent |
| ------------------- | ------- | ----------- | ---------- |
| Miles per hour      |         | `mph`       | `miles/hr` |
| Kilometers per hour | &check; | `kph`       | `km/hr`    |
| kilocalorie         | &check; | `Cal`       | `kcal`     |

### Metric Prefix Identifiers

| Metric name | Prefix string | Metric Scaling |
| ----------- | ------------- | -------------- |
| Quetta      | `Q`           | $1.0e30$       |
| Ronna       | `R`           | $1.0e27$       |
| Yotta       | `Y`           | $1.0e24$       |
| Zetta       | `Z`           | $1.0e21$       |
| Exa         | `E`           | $1.0e18$       |
| Peta        | `P`           | $1.0e15$       |
| Tera        | `T`           | $1.0e12$       |
| Giga        | `G`           | $1.0e9$        |
| Mega        | `M`           | $1.0e6$        |
| Kilo        | `k`           | $1.0e3$        |
| Hecto       | `h`           | $1.0e2$        |
| Deca        | `da`          | $1.0e1$        |
| Deci        | `d`           | $1.0e-1$       |
| Centi       | `c`           | $1.0e-2$       |
| Milli       | `m`           | $1.0e-3$       |
| Micro       | `μ`<br/>`u`   | $1.0e-6$       |
| Nano        | `n`           | $1.0e-9$       |
| Pico        | `p`           | $1.0e-12$      |
| Femto       | `f`           | $1.0e-15$      |
| Atto        | `a`           | $1.0e-18$      |
| Zepto       | `z`           | $1.0e-21$      |
| Yocto       | `y`           | $1.0e-24$      |
| Ronto       | `r`           | $1.0e-27$      |
| Quecto      | `q`           | $1.0e-30$      |

Note that some unit strings like `eV` could be indented to be `Exa-Volts` or `Electron Volts`. The library is case sensitive and will default to the 'least complex' unit that matches. So `Electron Volts` will be the parsed result. To get `Exa-Volts`, the user must properly specify `EV` or simply `V` for volts and then convert to the `Exa` metric scaler.

## Future Work

V3 can and is intended to be improved with some of these goals in mind:

- ~~Support for `<f32>`, `<i32>`, `<i64>`, and `<i128>` numeric types~~
  - ❌ Infeasible with type conversions and declared constants
- ~~Numerator and Denominator numeric variables to ensure floating point accuracy~~
  - ❌ Infeasible with type conversions and declared constants
- Significant digit considerations
- Equation definitions, which can expect a specific `Value` type
- More Imperial measurement support (US customary) 🤢
  - Lengths : `chains`, `furlongs`, `leagues`, `fathoms`, `cable`, `nautical mile`
  - Area : `acre`
  - Volume : `gallon`, `pint`, `quart`
  - Weight : `stone`, `ton`, `slug`
- ~~Metric support for `parsec` and `lightyears`~~
  - ✅ Support added
- ~~Metric support for `mmHg`~~
  - ❌ Does not make sense given its static metric prefix
- ~~Metric support for `electronVolt`~~
  - ✅ Support added
- ~~Metric support for `Kelvin`~~
  - ✅ Support added
- Metric support for `Celsius`
- ~~New support for: `Quetta`, `Quecto`, `Ronna`, and `Ronto`~~
- speed speed speed
