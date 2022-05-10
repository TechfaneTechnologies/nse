# Changelog for Ferium

## `v0.1.0`
### 11.05.2022

- Update to nse 0.0.9
- Improved upgrade code to be faster and more clean
- Immediately fail if rate limit error occured
- Show the http requests response status alongwith timetaken and no of retries it took while completing the request.

### Added

- This changelog
- Command line interface and corresponding code (under progress with `cli.rs` and `cli.yaml`)
- `NetworkData` & `OhlcvRecord` struct for nsefetch
- JSON file write
- `does_exist()` function for checking if a mod exists
- Error codes

### Changed

- Moved util.rs to `nse` mod folder
- Made all panics into `println!`s and exits (with descriptive error codes)
- Commented code better
