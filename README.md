# V3

V3 is a scientific unit type library that allows variables to dynamically keep track of different unit measurements. As these variables are defined and used, they may be converted to other units, metrically scaled, arithmetically combined with others, build new units, and divided into their base units.

## Table of Contents

- [V3](#v3)
  - [Table of Contents](#table-of-contents)
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
    - [**Absorbed** Dosage of Ionizing Radiation](#absorbed-dosage-of-ionizing-radiation)
    - [**Equivalent** Dosage of Ionizing Radiation](#equivalent-dosage-of-ionizing-radiation)
    - [Catalytic Activity](#catalytic-activity)
    - [Sound Intensity](#sound-intensity)
    - [Information](#information)
    - [Special Unit Keywords](#special-unit-keywords)
    - [Metric Prefix Identifiers](#metric-prefix-identifiers)
  - [Examples](#examples)
  - [Method Support](#method-support)
  - [Derived Units](#derived-units)
    - [Unit Checking](#unit-checking)
  - [Conversions](#conversions)
  - [Constants](#constants)
  - [Future Work](#future-work)

## Unit Support

The project supports all base SI units as listed by the National Institute of Standards and Technology (NIST) and many units listed by the General Conference on Weights and Measures (CGPM). *Some* American Imperial Units are also supported.

### Lengths

|Unit|Metric|Unit string|
|---|---|---|
|Meter|&check;|`m`|
|Inch||`in`<br/>`inch`[`es`]|
|Foot||`ft`<br/>`feet`<br/>`foot`|
|Yard||`yd`[`s`]<br/>`yard`[`s`]|
|Mile||`mile`[`s`]|
|Astronomical Unit||`AU`|
|Parsec||`pc`|
|Light Year||`lyr`<br/>`lightyear`[`s`]|
|Ångström||`Å`<br/>`angstrom`[`s`]|

### Time

|Unit|Metric|Unit string|
|---|---|---|
|Second|&check;|`s`|
|Minute||`min`<br/>`minute`[`s`]|
|Hour||`h`[`r`]<br/>`hour`[`s`]|
|Day||`d`<br/>`day`[`s`]|

### Mass

|Unit|Metric|Unit string|
|---|---|---|
|Gram|&check;|`g`|
|Grain||`gr`<br/>`grain`[`s`]|
|Ounce||`oz`<br/>`ounce`[`s`]|
|Pound||`lb`[`s`]<br/>`pounds`|

### Electric Current

|Unit|Metric|Unit string|
|---|---|---|
|Ampere|&check;|`A`|

### Electric Charge

|Unit|Metric|Unit string|
|---|---|---|
|Coulomb|&check;|`C`|

### Electric Potential

|Unit|Metric|Unit string|
|---|---|---|
|Volt|&check;|`V`|

### Electric Conductance

|Unit|Metric|Unit string|
|---|---|---|
|Siemens|&check;|`S`|

### Electric Capacitance

|Unit|Metric|Unit string|
|---|---|---|
|Farad|&check;|`F`<br/>`farad`[`s`]|

### Electric Resistance

|Unit|Metric|Unit string|
|---|---|---|
|Ohm|&check;|`Ω`<br/>`O`|

### Electric Inductance

|Unit|Metric|Unit string|
|---|---|---|
|Henry|&check;|`H`|

### Magnetic Flux

|Unit|Metric|Unit string|
|---|---|---|
|Weber|&check;|`Wb`|

### Magnetic Flux Density

|Unit|Metric|Unit string|
|---|---|---|
|Tesla|&check;|`T`|

### Thermal Temperature

|Unit|Metric|Unit string|
|---|---|---|
|Celsius||`c`<br/>`°c`<br/>`°C`|
|Fahrenheit||`f`<br/>`°f`<br/>`°F`|
|Kelvin|**[Future support]**|`K`|

### Substance

|Unit|Metric|Unit string|
|---|---|---|
|Mole|&check;|`mol`|

### Luminous Intensity

|Unit|Metric|Unit string|
|---|---|---|
|Candela|&check;|`cd`|

### Luminous Flux

|Unit|Metric|Unit string|
|---|---|---|
|Lumen|&check;|`lm`|

### Illuminance

|Unit|Metric|Unit string|
|---|---|---|
|Lux|&check;|`lx`|

### Spatial Volume

|Unit|Metric|Unit string|
|---|---|---|
|Liter|&check;|`l`|

### Pressure

|Unit|Metric|Unit string|
|---|---|---|
|Pascal|&check;|`Pa`|
|Bar|&check;|`bar`|
|Torr||`torr`|
|mmHg||`mmHg`|
|inHg||`inHg`|
|Atmospheres||`ATM`<br/>`atm`|
|Pounds per square inch||`PSI`<br/>`psi`|

### Geometric Angle

|Unit|Metric|Unit string|
|---|---|---|
|Degree||`°`<br/>`degree`[`s`]|
|Radian|&check;|`rad`<br/>`radian`[`s`]|
|Milliradian|&check;|`mil`[`s`]<br/>`MIL`|
|Minute of Angle||`moa`<br/>`MOA`|

### Frequency

|Unit|Metric|Unit string|
|---|---|---|
|Hertz|&check;|`Hz`|

### Force

|Unit|Metric|Unit string|
|---|---|---|
|Newton|&check;|`N`|
|Pound Force||`lbfr`<br/>`lbsfr`<br/>`poundforce`<br/>`poundsforce`|

### Energy

|Unit|Metric|Unit string|
|---|---|---|
|Joule|&check;|`J`|
|Calorie|&check;|`cal`|
|Foot pound||`ftlb`[`s`]<br/>`footpound`[`s`]|
|Electron Volt||`eV`|

### Power

|Unit|Metric|Unit string|
|---|---|---|
|Watt|&check;|`W`|

### Radioactivity

|Unit|Metric|Unit string|
|---|---|---|
|Becquerel|&check;|`Bq`|
|Curie||`Ci`|

### **Absorbed** Dosage of Ionizing Radiation

|Unit|Metric|Unit string|
|---|---|---|
|Gray|&check;|`Gy`|
|Röntgen||`R`|
|Rad||`rads`<br/>`Rads`|

### **Equivalent** Dosage of Ionizing Radiation

|Unit|Metric|Unit string|
|---|---|---|
|Sievert|&check;|`Sv`|
|Rem||`rem`<br/>`Rem`|

### Catalytic Activity

|Unit|Metric|Unit string|
|---|---|---|
|Katal|&check;|`kat`|

### Sound Intensity

|Unit|Metric|Unit string|
|---|---|---|
|Bel| &check; |`B`|

### Information

|Unit|Metric|Unit string|
|---|---|---|
|Bit|base 2 metric : Kilo - Yotta|`bits`|
|Byte|base 2 metric : Kilo - Yotta|`b`<br/>`byte`[`s`]|
|Qubit|**[Future Support]**|**[Future Support]**|

### Special Unit Keywords

|Unit|Metric|Unit string|Equivalent|
|---|---|---|---|
|Miles per hour||`mph`|`miles/hr`|
|Kilometers per hour|&check;|`kph`|`km/hr`|
|kilocalorie|&check;|**`C`**`al`|**`kc`**`al`|

### Metric Prefix Identifiers

|Metric name|Prefix string|
|---|---|
|Yotta|`Y`|
|Zetta|`Z`|
|Exa|`E`|
|Peta|`P`|
|Tera|`T`|
|Giga|`G`|
|Mega|`M`|
|Kilo|`k`|
|Hecto|`h`|
|Deca|`da`|
|Deci|`d`|
|Centi|`c`|
|Milli|`m`|
|Micro|`u`<br/>`μ`|
|Nano|`n`|
|Pico|`p`|
|Femto|`f`|
|Atto|`a`|
|Zepto|`z`|
|Yocto|`y`|

Note that some unit strings like `eV` could be indented to be `Exa-Volts` or `Electron Volts`. The library is case sensitive and will default to the 'least complex' unit that matches. So `Electron Volts` will be the parsed result. To get `Exa-Volts`, the user must properly specify `EV` or simply `V` for volts and then convert to the `Exa` metric scaler.

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

## Derived Units

Many of the SI units are derived from other base units. When using the values to conduct arithmetic operations, values can be explicitly asked to be 'complex' or 'reduced'.

Making a complex value means combining different types into a new type.

```rust
use v3::values::Value;

let m:Value = Value::new(2.5, "kg").unwrap();
let acc:Value = Value::new(9.81, "m/s^2").unwrap();

let f1:Value = m*acc;
let f2:Value = (m*acc).complex().unwrap();
```

Variable `f1` will be `24.525 kg*m/s^2` whereas `f2` will be `24.525 N`

Reducing a value means setting a value to its derived units.

```rust
use v3::values::Value;

let mut f:Value = Value::new(24.525, "N").unwrap();

f.reduce("kg*m/s^2").unwrap();
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

Units cannot be converted between disparate types, although there are some exceptions. `ml` to `mm^3` is one such example of volume to a cubed length.

## Constants

Some constants are provided for ease of use:

|Name|Numeric Value|Units|
|---|---|---|
|Absolute Zero|`0`|`K`|
|Avogadro's Number|`6.02214076e23`|`mol^-1`|
|Faraday Constant|`96_485.332_123_310_01`|`C/mol`|
|Atomic Mass Constant|`1.66053906660e-27`|`kg`|
|Molar Gas Constant|`8.314_462_1`|`J/(K*mol)`|
|Coulomb's Constant|`8.987_551`|`mol*-1`|
|The Speed of Light|`299_792_458.0`|`m/s`|
|Boltzmann Constant|`1.380649e-23`|`J/K`|
|Earth's Average Gravitational Acceleration|`9.806_65`|`m/s^2`|
|Newtonian Constant of Gravitation|`6.673015e-5`|`m^3/(kg*s^2)`|
|Charge of an Electron|`1.602176634e_19`|`C`|
|Rydberg Constant|`10_973_731.568_539`|`1/m`|
|Plank's Constant|`6.62607015e-34`|`J/Hz`|
|Vacuum Permittivity|`8.8541878128e-12`|`F/m`|

## Future Work

V3 can and is intended to be improved with some of these goals in mind:

- Support for `<f32>`, `<i32>`, `<i64>`, and `<i128>` numeric types
- Numerator and Denominator numeric variables to ensure floating point accuracy
- Significant digit considerations
- Equation definitions, which can expect a specific `Value` type
- More Imperial measurement support 🤢
- Qubit support
- speed speed speed
