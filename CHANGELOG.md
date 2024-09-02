# Changelog
## [0.3.0] - 2024-09-02
### Added
- Short flag `-L` for `--error-correction-level`
## Changed
- Repo at github.com/pepa65/qr
- No `#` to specify hex colors
### Removed
- xml/DOCTYPE headers from svg
- Tabs from svg
- Attributes stroke, width, height from svg
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
