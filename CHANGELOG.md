# Changelog
## [1.7.3] - 2025-03-09
- Fmt and update deps

## [1.7.2] - 2025-02-22
- Update cargo-binstall link

## [1.7.1] - 2025-02-22
- Update CI and tempfile version

## [1.7.0] - 2025-02-22
- Refactor and add promptpay

## [1.6.2] - 2025-02-08
- Fixed readme, modified help

## [1.6.1] - 2025-02-08
- Fixed bad path error

## [1.6.0] - 2025-02-04
- Proportion option for logo

## [1.5.2] - 2025-02-04
- Allow logo transparency

## [1.5.1] - 2025-01-30
- Double logo size

## [1.5.0] - 2025-01-30
- Allow logo overlay

## [1.4.3] - 2025-01-13
- Set single dependency version

## [1.4.2] - 2025-01-13
- Fix dependency version

## [1.4.1] - 2025-01-13
- Update dependencies
- Changelog

## [1.4.0] - 2025-01-12
- Update dependencies
- Compact help 

## [1.3.0] - 2025-01-12
- Default output file name
- Terminal option

## [1.2.0] - 2025-01-11
- Redo options

## [1.1.5] - 2024-11-13
- Update deps

## [1.1.4] - 2024-10-29
- Repo

## [1.1.3] - 2024-10-29
- Cargo-binstall

## [1.1.2] - 2024-10-23
- Update deps

## [1.1.1] - 2024-10-19
- Enhance doc

## [1.1.0] - 2024-10-08
- Completions

## [1.0.1] - 2024-10-05
- Correct manpage

## [1.0.0] - 2024-10-05
### Changed
- Repo name to eqr
- Pushed to crates.io

## [0.3.2] - 2024-09-05
### Changed
- Static (musl) binary

## [0.3.1] - 2024-09-02
### Added
- Release binary

## [0.3.0] - 2024-09-02
### Added
- Short flag `-L` for `--error-correction-level`
- Atrributes width/height on svg element for scale

### Changed
- Repo at github.com/pepa65/qr
- Allow scale for svg
- Fix border on raster images
- Fix help on fg/bg
- No inverted colors on terminal
- No `#` to specify hex colors

### Removed
- xml/DOCTYPE headers from svg
- Tabs from svg
- Attribute stroke from svg
- `jpeg` file format from `-o`/`--output`

## [0.2.0] - 2022-07-18
### Added
- Feature to read string to encode from standard input, which allows to pipe
  commands to `qr-rs` (closing issue #1).
- `border` CLI option to specify border size.
- `error-correction-level` CLI option to specify QR *error-correction-level* as
  one of the following values:
  - **low**;
  - **medium**;
  - **quartile**;
  - **high**.

## [0.1.0] - 2022-07-15
Initial release.
