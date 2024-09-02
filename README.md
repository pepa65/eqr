[![Cargo build](https://github.com/pepa65/qr/actions/workflows/rust.yml/badge.svg)](https://github.com/pepa65/qr/actions/workflows/rust.yml)

# qr 0.3.1
**Encode text into svg/png/jpg/terminal format QR codes**

## Index
- [Install](#install)
  * [Cargo](#cargo)
    - [Master branch](#master-branch)
- [Usage](#usage)
- [Changelog](#changelog)
- [License](#license)

## Install
The following installation instructions assume a **Rust toolchain** (`rustc >=1.62.0`) installed
on the system. In order to install such toolchain you can use `rustup`: see
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
qr 0.3.1
Marco Radocchia <marco.radocchia@outlook.com>, github.com/pepa65
Encode text into svg/png/jpg/terminal format QR codes

USAGE:
    qr [OPTIONS] [STRING]

ARGS:
    <STRING>    String to encode

OPTIONS:
    -b, --bg <BG>
            Background RGB color (hex code) [default: fff]

    -B, --border <BORDER>
            Border size (expressed in unit blocks) [default: 1]

    -f, --fg <FG>
            Foreground RGB color (hex code) [default: 000]

    -h, --help
            Print help information

    -L, --error-correction-level <ERROR_CORRECTION_LEVEL>
            QR error correction level [default: medium] [possible values: low, medium, quartile,
            high]

    -o, --output <OUTPUT>
            Output file (supported file extensions: jpg, png, svg); omit to print QR code to console

    -s, --scale <SCALE>
            Scale factor (1..255) [default: 16]

    -V, --version
            Print version information
```

## Changelog
Complete [CHANGELOG](CHANGELOG.md).

## License
[GPLv3](LICENSE)
