# Changelog for NSE

## `v0.1.2`
### 11.05.2022

- Updated to nse 0.1.2

### Changed

- Fixed Isahc version to 1.7.1 instead of git master branch due to failing curl-sys build
- Commented code better

## `v0.1.1`
### 10.05.2022

- Updatd to nse 0.1.1
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


