[![Cargo build](https://github.com/pepa65/eqr/actions/workflows/rust.yml/badge.svg)](https://github.com/pepa65/eqr/actions/workflows/rust.yml)
[![downloads](https://img.shields.io/crates/d/eqr.svg)](https://crates.io/crates/eqr)

# eqr 1.1.3
**Encode text into svg/png/jpg/terminal-format QR codes**

## Install
### Install standalone single-binary
```
wget https://github.com/pepa65/eqr/releases/download/1.1.2/qr
sudo mv qr /usr/local/bin
sudo chown root:root /usr/local/bin/qr
sudo chmod +x /usr/local/bin/qr
```

### Install with cargo
If not installed yet, install a **Rust toolchain**, see https://www.rust-lang.org/tools/install

#### Direct from crates.io
`cargo install eqr`

#### Direct from repo
`cargo install --git https://github.com/pepa65/eqr`

#### Static build (avoiding GLIBC incompatibilities)
```
git clone https://github.com/pepa65/eqr
cd eqr
rustup target add x86_64-unknown-linux-musl
cargo rel  # Alias in .cargo/config.toml
```

The binary will be at `target/x86_64-unknown-linux-musl/release/qr`

### Install with cargo-binstall
Even without a full Rust toolchain, rust binaries can be installed with the static binary `cargo-binstall`:

```
# Install cargo-binstall for Linux x86_64
# (Other versions are available at https://crates.io/crates/cargo-binstall)
wget github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
tar xf cargo-binstall-x86_64-unknown-linux-musl.tgz
sudo chown root:root cargo-binstall
sudo mv cargo-binstall /usr/local/bin/
```

Only a linux-x86_64 (musl) binary available: `cargo-binstall eqr`

## Usage
```
eqr 1.1.3 - Encode text into svg/png/jpg/terminal-format QR codes
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
