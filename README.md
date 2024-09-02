<div align="center">
  <h1 align="center">qr</h1>
  ![colored_qr](assets/colored_qr.png)
</div>

Command Line Interface utility to encode URLs or more generally text into QR
codes in various file formats and colors.

## Index
- [Install](#install)
  * [Cargo](#cargo)
    - [Master branch](#master-branch)
- [Usage](#usage)
- [Changelog](#changelog)
- [License](#license)

## Install
The following installation instructions assume a **Rust toolchain** (`rustc >=1.62.0`) installed
on the system. In order to install such toolchain you can use `rusutp`: see
https://www.rust-lang.org/tools/install for further installation instructions and notes.

### Cargo
In order to install using Rust's package manager `cargo` follow instructions below.

#### Master branch
To build and install from master branch run:
```sh
cargo install --git https://github.com/pepa65/qr --branch master
```

## Usage
```
qr 0.3.0
Marco Radocchia <marco.radocchia@outlook.com>, github.com/pepa65
A CLI utility to encode URLs or text into QR codes in various formats and colors.

USAGE:
    qr [OPTIONS] [STRING]

ARGS:
    <STRING>    String to encode

OPTIONS:
    -b, --bg <BG>
            Foreground color (hex code) [default: #FFF]

    -B, --border <BORDER>
            Border size (expressed in unit blocks) [default: 1]

    -L, --error-correction-level <ERROR_CORRECTION_LEVEL>
            QR error orrection level [default: medium] [possible values: low, medium, quartile,
            high]

    -f, --fg <FG>
            Background color (hex code) [default: #000]

    -h, --help
            Print help information

    -o, --output <OUTPUT>
            Output file (supported file extensions: jpg, png, svg); omit to print QR code to
            console

    -s, --scale <SCALE>
            Scale factor (raster image output only) [default: 25]

    -V, --version
            Print version information
```

## Changelog
Complete [CHANGELOG](CHANGELOG.md).

## License
[GPLv3](LICENSE)
