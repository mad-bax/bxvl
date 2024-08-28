<p align="center">
  <img src="./images/bxvl_logo.ico">
</p>

**bxvl** is a rust library that allows variables to dynamically keep track of different unit measurements. As these variables are defined and used, they may be converted to other units, metrically scaled, arithmetically combined with others, build new units, and divided into their base units.

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
  - [Geometric Solid Angle](#geometric-solid-angle)
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
use bxvl::{Value, value};
use bxvl::units::{Metric, UnitTime::Second, UnitMass::Gram, UnitLength::Meter};

// Slowest
let v1:Value = match "22.3 kg*m/s^2".parse::<Value>() {
  Ok(v) => v,
  Err(e) => panic!("{}", e)
};

// Slow
let v2:Value = value!(22.3, "kg*m/s^2");

// Average
let v3:Value = match Value::new(22.3, "kg*m/s^2") {
  Ok(v) => v,
  Err(e) => panic!("{}", e)
};

// Fastest
let v4:Value = 22.3
  / Second(Metric::None)
  / Second(Metric::None)
  * Gram(Metric::Kilo)
  * Meter(Metric::None);

assert!((v1 == v2) == (v3 == v4));
assert!((v1 == v3) == (v2 == v4));
assert!((v1 == v4) == (v2 == v3));
```

Creating `Value`s using other `Values`:

```rust
use bxvl::Value;
use bxvl::units::{Metric, UnitTime, UnitLength};

let time:Value = 4.0 * UnitTime::Second(Metric::None);
let dist:Value = 16.8 * UnitLength::Meter(Metric::None);

let speed:Value = dist/time;
assert!(speed.is_velocity());
assert_eq!(speed.to_string(), "4.2 m/s");
```

## Method Support

Values provide similar functionality to many functions that are available to other units such as `i32`, `f32`, `f64` etc.

```rust
use bxvl::Value;
use bxvl::units::{Metric, UnitLength};

let m:Value = Value::new(f64::NAN, "feet").unwrap();
if m.is_nan() {
  println!("Our value is not a number!");
}

let a:Value = 1.4 * UnitLength::Meter(Metric::None);
let r:Value = a.sin();
assert!(r.is_radians());
assert!(r.val >= 0.985449);
assert!(r.val < 0.985450);
```

## Derived Units

Many of the SI units are derived from other base units. When using the values to conduct arithmetic operations, values can be explicitly asked to be 'complex' or 'reduced'.

Making a complex value means combining different types into a new type.

```rust
use bxvl::Value;

let m:Value = Value::new(2.5, "kg").unwrap();
let acc:Value = Value::new(10.0, "m/s^2").unwrap();

let f1:Value = m*acc;
let f2:Value = (m*acc).complex();
assert!(f1.is_force() && f2.is_force());
assert!(f1.val == f2.val);
assert_eq!(f1.to_string(), "25 m*kg/s^2");
assert_eq!(f2.to_string(), "25 N");
```

Reducing a value means setting a value to its derived units.

```rust
use bxvl::Value;

let mut f:Value = Value::new(25.0, "N").unwrap();

assert!(f.is_force());
f.reduce("kg*m/s^2").unwrap();
assert!(f.is_force());
assert_eq!(f.to_string(), "25 m*kg/s^2");
```

This behavior is explicit and must be called by the user.

### Unit Checking

**bxvl** provides functions like `.is_force()` which will return `true` for both `kg*m/s^2` and `N`. Function support includes all of the base [unit types](#unit-support) as well as extra unit combinations (See below).

| Function                     | Measurement Types                                                                                                                                                                                                                                                                      |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `is_length()`                | [Length](#lengths)                                                                                                                                                                                                                                                                     |
| `is_area()`                  | [Length](#lengths)^2                                                                                                                                                                                                                                                                   |
| `is_volume()`                | [Volume](#spatial-volume)<br/>[Length](#lengths)^3                                                                                                                                                                                                                                     |
| `is_temperature()`           | [Temperature](#thermal-temperature)                                                                                                                                                                                                                                                    |
| `is_mass()`                  | [Mass](#mass)                                                                                                                                                                                                                                                                          |
| `is_density()`               | [Mass](#mass)/[Volume](#spatial-volume)<br/>[Mass](#mass)/[Length](#lengths)^3                                                                                                                                                                                                         |
| `is_time()`                  | [Time](#time)                                                                                                                                                                                                                                                                          |
| `is_substance()`             | [Substance](#substance)                                                                                                                                                                                                                                                                |
| `is_angle()`                 | [Angle](#geometric-angle)                                                                                                                                                                                                                                                              |
| `is_solid_angle()`           | [Solid Angle](#geometric-solid-angle)                                                                                                                                                                                                                                                  |
| `is_information()`           | [Information](#information)                                                                                                                                                                                                                                                            |
| `is_velocity()`              | [Length](#lengths)/[Time](#time)                                                                                                                                                                                                                                                       |
| `is_acceleration()`          | [Length](#lengths)/[Time](#time)^2                                                                                                                                                                                                                                                     |
| `is_force()`                 | [Force](#force)<br/>[Mass](#mass)\**acceleration*                                                                                                                                                                                                                                      |
| `is_momentum()`              | [Mass](#mass)\**velocity*                                                                                                                                                                                                                                                              |
| `is_frequency()`             | [Frequency](#frequency)<br/>1/[Time](#time)                                                                                                                                                                                                                                            |
| `is_pressure()`              | [Pressure](#pressure)<br/>[Force](#force)/*area*<br/>[Mass](#mass)/([Length](#lengths)\*[Time](#time)^2)                                                                                                                                                                               |
| `is_energy()`                | [Energy](#energy)<br/>[Length](#lengths)\*[Force](#force)<br/>[Electric Potential](#electric-potential)\*[Electric Charge](#electric-charge)<br/>[Power](#power)\*[Time](#time)<br/>[Mass](#mass)\**area*/[Time](#time)^2                                                              |
| `is_power()`                 | [Power](#power)<br/>[Energy](#energy)/[Time](#time)<br/>[Electrical Potential](#electric-potential)\*[Electric Current](#electric-current)<br/>[Mass](#mass)\**area*/[Time](#time)^3                                                                                                   |
| `is_electric_charge()`       | [Electric Charge](#electric-charge)<br/>[Electric Current](#electric-current)\*[Time](#time)<br/>[Electric Capacitance](#electric-capacitance)\*[Electric Potential](#electric-potential)                                                                                              |
| `is_electric_current()`      | [Electric Current](#electric-current)                                                                                                                                                                                                                                                  |
| `is_electric_potential()`    | [Electric Potential](#electric-potential)<br/>[Power](#power)/[Electric Current](#electric-current)<br/>[Energy](#energy)/[Electric Charge](#electric-charge)                                                                                                                          |
| `is_capacitance()`           | [Electric Capacitance](#electric-capacitance)<br/>[Electric Charge](#electric-charge)/[Electric Potential](#electric-potential)<br/>[Energy](#energy)/[Electric Charge](#electric-charge)                                                                                              |
| `is_resistance()`            | [Electric Resistance](#electric-resistance)<br/>1/[Electric Conductance](#electric-conductance)<br/>[Electric Potential](#electric-potential)/[Electric Current](#electric-current)                                                                                                    |
| `is_conductance()`           | [Electric Conductance](#electric-conductance)<br/>1/[Electric Resistance](#electric-resistance)<br/>[Electric Current](#electric-current)/[Electric Potential](#electric-potential)                                                                                                    |
| `is_magnetic_flux()`         | [Magnetic Flux](#magnetic-flux)<br/>[Energy](#energy)/[Electric Current](#electric-current)<br/>[Magnetic Flux Density](#magnetic-flux-density)\**area*<br/>[Electric Potential](#electric-potential)\*[Time](#time)                                                                   |
| `is_magnetic_flux_density()` | [Magnetic Flux Density](#magnetic-flux-density)<br/>[Electric Potential](#electric-potential)\*[Time](#time)/*area*<br/>[Magnetic Flux](#magnetic-flux)/*area*<br/>[Force](#force)/([Electric Current](#electric-current)\*[Length](#lengths))                                         |
| `is_inductance()`            | [Electric Inductance](#electric-inductance)<br/>[Electric Potential](#electric-potential)\*[Time](#time)/[Electric Current](#electric-current)<br/>[Electric Resistance](#electric-resistance)*[Time](#time)<br/>[Magnetic Flux](#magnetic-flux)/[Electric Current](#electric-current) |
| `is_luminous_flux()`         | [Luminous Flux](#luminous-flux)                                                                                                                                                                                                                                                        |
| `is_illuminance()`           | [Illuminance](#illuminance)<br/>[Luminous Flux](#luminous-flux)/*area*                                                                                                                                                                                                                 |
| `is_luminous_intensity()`    | [Luminous Intensity](#luminous-intensity)                                                                                                                                                                                                                                              |
| `is_radioactivity()`         | [Radioactivity](#radioactivity)                                                                                                                                                                                                                                                        |
| `is_absorbed_dose()`         | [Absorbed Dose](#absorbed-dosage-of-ionizing-radiation)                                                                                                                                                                                                                                |
| `is_equivalent_dose()`       | [Equivalent Dose](#equivalent-dosage-of-ionizing-radiation)                                                                                                                                                                                                                            |
| `is_catalytic_activity()`    | [Catalytic Activity](#catalytic-activity)<br/>[Substance](#substance)/[Time](#time)                                                                                                                                                                                                    |
| `is_sound()`                 | [Sound](#sound-intensity)                                                                                                                                                                                                                                                              |
| `is_jerk()`                  | [Length](#lengths)/[Time](#time)^3                                                                                                                                                                                                                                                     |
| `is_snap()`                  | [Length](#lengths)/[Time](#time)^4                                                                                                                                                                                                                                                     |
| `is_angular_velocity()`      | [Angle](#geometric-angle)/[Time](#time)                                                                                                                                                                                                                                                |
| `is_angular_acceleration()`  | [Angle](#geometric-angle)/[Time](#time)                                                                                                                                                                                                                                                |
| `is_frequency_drift()`       | [Frequency](#frequency)/[Time](#time)                                                                                                                                                                                                                                                  |
| `is_flow()`                  | [Volume](#spatial-volume)/[Time](#time)<br/>[Length](#lengths)^3/[Time](#time)                                                                                                                                                                                                         |
| `is_angular_momentum()`      | [Force](#force)\*[Length](#lengths)\*[Time](#time)                                                                                                                                                                                                                                     |
| `is_torque()`                | [Force](#force)*[Length](#lengths)<br/>[Energy](#energy)/[Angle](#geometric-angle)                                                                                                                                                                                                     |
| `is_energy_density()`        | [Energy](#energy)/[Volume](#spatial-volume)<br/>[Energy](#energy)/[Length](#lengths)^3                                                                                                                                                                                                 |

## Conversions

All `Value`s within their given measurement type will be able to be converted to each other. Values with multiple types, in most cases, can be converted to their compatible types.

Example converting feet into meters:

```rust
use bxvl::Value;

let mut m:Value = Value::new(3.2, "feet").unwrap();

m.convert("m").unwrap();
```

There is also direct syntax for this feature:

```rust
use bxvl::Value;

let mut m:Value = Value::new(5.9, "km/hr").unwrap();

m >>= "m/s";
```

You can use other Values for conversion:

```rust
use bxvl::Value;

let m:Value = Value::new(1.2, "yards").unwrap();
let n:Value = Value::new(1.0, "m").unwrap();

let k:Value = (m >> n).unwrap();
```

The types can also be directly used: (The fastest conversion method)

```rust
use bxvl::Value;
use bxvl::units::{Metric, UnitLength, UnitTime};

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

These exceptions are valid conversion so long as they are the *only* units within a `Value`. This is to avoid conversion scenarios where `Value`s produce (or are created with) neutralizing units, e.g. `mm^3/ml`, which is 'unitless'. Therefore, `m/s` cannot be converted to `m*kHz` and `m^3/N` cannot be converted to `ml/N`.

## Constants

Some constants are provided for ease of use:

| Name                                       | `f64` numeric Value     | Units              |
| ------------------------------------------ | ----------------------- | ------------------ |
| Absolute Zero                              | `0.`                    | $K$                |
| Avogadro's Number                          | `6.022_140_76e23`       | $mol^{-1}$      |
| Faraday Constant                           | `96_485.332_123_310_01` | $C \over mol$      |
| Atomic Mass Constant                       | `1.660_539_066_60e-27`  | $kg$               |
| Molar Gas Constant                         | `8.314_462_1`           | $J \over K*mol$    |
| Coulomb's Constant                         | `8.987_551`             | $mol^{-1}$      |
| The Speed of Light                         | `299_792_458.0`         | $m \over s$        |
| Boltzmann Constant                         | `1.380_649e-23`         | $J \over K$        |
| Earth's Average Gravitational Acceleration | `9.806_65`              | $m \over s^2$      |
| Newtonian Constant of Gravitation          | `6.673_015e-11`         | $m^3 \over kg*s^2$ |
| Charge of an Electron                      | `1.602_176_634e-19`     | $C$                |
| Rydberg Constant                           | `10_973_731.568_539`    | $m^{-1}$           |
| Plank's Constant                           | `6.626_070_15e-34`      | $J \over Hz$       |
| Vacuum Permittivity                        | `8.854_187_812_8e-12`   | $F \over m$        |

```rust
use bxvl::{Value, consts, units::{UnitMass, Metric}};

let acc:Value = consts::EARTH_GRAVITY;
let m:Value = 100.0 * UnitMass::Gram(Metric::Kilo);

let f = (m * acc).complex();

assert_eq!(f.to_string(), "980.665 N");
```

## Unit Support

The project supports all base SI units as listed by the National Institute of Standards and Technology (NIST) and many units listed by the General Conference on Weights and Measures (CGPM). *Some* American Imperial Units are also supported.

### Lengths

| Unit              | Metric Prefixing Support | Base Conversion Factor              | Unit string            |
| ----------------- | ------------------------ | ----------------------------------- | ---------------------- |
| Meter             | &check;                  | `1.0 m`                             | `m`                    |
| Inch              |                          | `0.025_4 m`                         | `in`, `inch`[`es`]     |
| Foot              |                          | `0.304_8 m`                         | `ft`, `feet` `foot`    |
| Yard              |                          | `0.914_4 m`                         | `yd`[`s`], `yard`[`s`] |
| Mile              |                          | `1_609.344 m`                       | `mile`[`s`]            |
| Astronomical Unit |                          | `149_569_870_700.0 m`               | `AU`                   |
| Parsec            | &check;                  | `(648_000.0/π)*149_569_870_700.0 m` | `pc`                   |
| Light Year        | &check;                  | `9_460_730_472_580_800.0 m`         | `lyr`                  |
| Ångström          |                          | `0.000_000_000_1 m`                 | `Å`, `angstrom`[`s`]   |

### Time

| Unit   | Metric Prefixing Support | Base Conversion Factor | Unit string           |
| ------ | ------------------------ | ---------------------- | --------------------- |
| Second | &check;                  | `1.0 s`                | `s`                   |
| Minute |                          | `60.0 s`               | `min`, `minute`[`s`]  |
| Hour   |                          | `3_600.0 s`            | `h`[`r`], `hour`[`s`] |
| Day    |                          | `86_400.0 s`           | `d`, `day`[`s`]       |

### Mass

| Unit  | Metric Prefixing Support | Base Conversion Factor | Unit string         |
| ----- | ------------------------ | ---------------------- | ------------------- |
| Gram  | &check;                  | `1.0 g`                | `g`                 |
| Grain |                          | `453.592_37/7_000.0 g` | `gr`, `grain`[`s`]  |
| Ounce |                          | `453.592_37/16.0 g`    | `oz`, `ounce`[`s`]  |
| Pound |                          | `453.592_37 g`         | `lb`[`s`], `pounds` |

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

| Unit  | Metric Prefixing Support | Base Conversion Factor | Unit string       |
| ----- | ------------------------ | ---------------------- | ----------------- |
| Farad | &check;                  | `1.0 F`                | `F`, `farad`[`s`] |

### Electric Resistance

| Unit | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ---- | ------------------------ | ---------------------- | ----------- |
| Ohm  | &check;                  | `1.0 Ω`                | `Ω` `O` |

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

| Unit       | Metric Prefixing Support | Base Conversion Factor | Unit string                        |
| ---------- | ------------------------ | ---------------------- | ---------------------------------- |
| Celsius    | &check;                  | `c-273.15 K`           | `c`, `°`[`Metric Prefix`]`c`, `°C` |
| Fahrenheit |                          | `(f-32.0)/1.8 c`       | `f`, `°f`, `°F`                    |
| Kelvin     | &check;                  | `1.0 K`                | `K`                                |

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

| Unit                   | Metric Prefixing Support | Base Conversion Factor | Unit string  |
| ---------------------- | ------------------------ | ---------------------- | ------------ |
| Pascal                 | &check;                  | `1.0 Pa`               | `Pa`         |
| Bar                    | &check;                  | `100_000.0 Pa`         | `bar`        |
| Torr                   |                          | `101_325.0/760.0 Pa`   | `torr`       |
| mmHg                   |                          | `133.322_387_415 Pa`   | `mmHg`       |
| cmHg                   |                          | `1333.22_387_415 Pa`   | `cmHg`       |
| inHg                   |                          | `3_386.388_666_6 Pa`   | `inHg`       |
| Atmospheres            |                          | `101_325.0 Pa`         | `ATM`, `atm` |
| Pounds per square inch |                          | `6894.757 Pa`          | `PSI`, `psi` |

### Geometric Angle

| Unit            | Metric Prefixing Support | Base Conversion Factor | Unit string          |
| --------------- | ------------------------ | ---------------------- | -------------------- |
| Degree          |                          | `π/180.0 rad`          | `°`, `degree`[`s`]   |
| Radian          | &check;                  | `1.0 rad`              | `rad`, `radian`[`s`] |
| Milliradian     | &check;                  | `1_000.0 rad`          | `mil`[`s`], `MIL`    |
| Minute of Angle |                          | `π/10_800.0 rad`       | `moa`, `MOA`         |

### Geometric Solid Angle

| Unit      | Metric Prefixing Support | Base Conversion Factor | Unit string |
| --------- | ------------------------ | ---------------------- | ----------- |
| Steradian | &check;                  | `1.0 sr`               | `sr`        |

### Frequency

| Unit  | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ----- | ------------------------ | ---------------------- | ----------- |
| Hertz | &check;                  | `1.0 Hz`               | `Hz`        |

### Force

| Unit        | Metric Prefixing Support | Base Conversion Factor  | Unit string                                 |
| ----------- | ------------------------ | ----------------------- | ------------------------------------------- |
| Newton      | &check;                  | `1.0 N`                 | `N`                                         |
| Pound Force |                          | `4.448_221_615_260_5 N` | `lbfr`, `lbsfr`, `poundforce` `poundsforce` |

### Energy

| Unit          | Metric Prefixing Support | Base Conversion Factor | Unit string                   |
| ------------- | ------------------------ | ---------------------- | ----------------------------- |
| Joule         | &check;                  | `1.0 J`                | `J`                           |
| Calorie       | &check;                  | `4.184 J`              | `cal`                         |
| Foot pound    |                          | `1.355_818 J`          | `ftlb`[`s`], `footpound`[`s`] |
| Electron Volt | &check;                  | `1.6021_766_34e-19 J`  | `eV`                          |

### Power

| Unit       | Metric Prefixing Support | Base Conversion Factor | Unit string |
| ---------- | ------------------------ | ---------------------- | ----------- |
| Watt       | &check;                  | `1.0 W`                | `W`         |
| Horsepower |                          | `745.699872 W`         | `hp`        |

### Radioactivity

| Unit      | Metric Prefixing Support | Base Conversion Factor | Unit string |
| --------- | ------------------------ | ---------------------- | ----------- |
| Becquerel | &check;                  | `1.0 Bq`               | `Bq`        |
| Curie     |                          | `37_000_000_000.0 Bq`  | `Ci`        |

### *Absorbed* Dosage of Ionizing Radiation

| Unit    | Metric Prefixing Support | Base Conversion Factor | Unit string    |
| ------- | ------------------------ | ---------------------- | -------------- |
| Gray    | &check;                  | `1.0 Gy`               | `Gy`           |
| Röntgen |                          | `0.01 Gy`              | `R`            |
| Rad     |                          | `1.0/114.025 Gy`       | `rads`, `Rads` |

### *Equivalent* Dosage of Ionizing Radiation

| Unit    | Metric Prefixing Support | Base Conversion Factor | Unit string  |
| ------- | ------------------------ | ---------------------- | ------------ |
| Sievert | &check;                  | `1.0 Sv`               | `Sv`         |
| Rem     |                          | `0.01 Sv`              | `rem`, `Rem` |

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

| Unit | Metric Prefixing Support | Base Conversion Factor | Unit string      |
| ---- | ------------------------ | ---------------------- | ---------------- |
| Bit  | `Kilo` - `Quetta`        | `8.0 bits == 1.0 byte` | `bits`           |
| Byte | `Kilo` - `Quetta`        | `1.0 bytes`            | `b`, `byte`[`s`] |

### Special Unit Keywords

| Unit                | Unit string | Equivalent |
| ------------------- | ----------- | ---------- |
| Miles per hour      | `mph`       | `miles/hr` |
| Kilometers per hour | `kph`       | `km/hr`    |
| kilocalorie         | `Cal`       | `kcal`     |

### Metric Prefix Identifiers

| Metric name | Prefix string | Metric Scaling |
| ----------- | ------------- | -------------- |
| Quetta      | `Q`           | $1e30$         |
| Ronna       | `R`           | $1e27$         |
| Yotta       | `Y`           | $1e24$         |
| Zetta       | `Z`           | $1e21$         |
| Exa         | `E`           | $1e18$         |
| Peta        | `P`           | $1e15$         |
| Tera        | `T`           | $1e12$         |
| Giga        | `G`           | $1e9$          |
| Mega        | `M`           | $1e6$          |
| Kilo        | `k`           | $1e3$          |
| Hecto       | `h`           | $1e2$          |
| Deca        | `da`          | $1e1$          |
| **None**    |               | $1$            |
| Deci        | `d`           | $1e-1$         |
| Centi       | `c`           | $1e-2$         |
| Milli       | `m`           | $1e-3$         |
| Micro       | `μ` `u`       | $1e-6$         |
| Nano        | `n`           | $1e-9$         |
| Pico        | `p`           | $1e-12$        |
| Femto       | `f`           | $1e-15$        |
| Atto        | `a`           | $1e-18$        |
| Zepto       | `z`           | $1e-21$        |
| Yocto       | `y`           | $1e-24$        |
| Ronto       | `r`           | $1e-27$        |
| Quecto      | `q`           | $1e-30$        |

Note that some unit strings like `eV` could be indented to be `Exa-Volts` or `Electron Volts`. The library is case sensitive and will default to the 'least complex' unit that matches. So `Electron Volts` will be the parsed result. To get `Exa-Volts`, the user must properly specify `EV` or simply `V` for volts and then convert to the `Exa` metric scaler.

## Future Work

**bxvl** can and is intended to be improved with some of these goals in mind:

- `Value`s that have `sqrt` units.
- Logarithmic units.
- User defined `const Value`s
- Equation definitions, which can expect a specific `Value` type
- More Imperial measurement support (US customary) 🤢
  - Lengths : `chains`, `furlongs`, `leagues`, `fathoms`, `cable`, `nautical mile`
  - Area(?) : `acre`
  - Volume : `gallon`, `pint`, `quart`
  - Weight : `stone`, `ton`, `slug`
- speed speed speed
