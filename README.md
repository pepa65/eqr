[![Cargo build](https://github.com/pepa65/eqr/actions/workflows/rust.yml/badge.svg)](https://github.com/pepa65/eqr/actions/workflows/rust.yml)
[![Dependencies](https://deps.rs/repo/github/pepa65/eqr/status.svg)](https://deps.rs/repo/github/pepa65/eqr)
[![Docs](https://img.shields.io/badge/Docs-eqr-blue)](https://docs.rs/repo/github/pepa65/eqr)
[![License](https://img.shields.io/github/license/pepa65/eqr)](https://github.com/pepa65/eqr/blob/main/LICENSE)
[![Downloads](https://img.shields.io/crates/d/eqr.svg)](https://crates.io/crates/eqr)

# eqr 1.4.1
**Encode text into svg/png/jpg/terminal-format QR codes**

## Install
### Install standalone single-binary
```
wget https://github.com/pepa65/eqr/releases/download/1.4.1/qr
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

It will be installed in `~/.cargo/bin/` which still needs to be added to `PATH`!

## Usage
```
eqr 1.4.1 - Encode text into svg/png/jpg/terminal-format QR codes
Usage: qr [OPTIONS] [STRING]
Arguments:
  [STRING]  String to encode (can also be piped in)

Options:
  -o, --output <OUTPUT>                 Output file (jpg/png/svg) [default: qr.png]
  -t, --terminal                        Output to terminal
  -l, --level <ERROR_CORRECTION_LEVEL>  QR error correction level (L: 7%, M: 15%, Q: 25%, H: 30%) [default: M]
  -e, --edge <BORDER>                   Edge size (in unit blocks) [default: 2]
  -f, --fg <FG>                         Foreground RGB color (hex code) [default: 000]
  -b, --bg <BG>                         Background RGB color (hex code) [default: fff]
  -s, --scale <SCALE>                   Scale factor (1..255) [default: 16]
  -h, --help                            Print help
  -V, --version                         Print version
```

## Changelog
Complete [CHANGELOG](CHANGELOG.md).

## License
[GPLv3](LICENSE)
