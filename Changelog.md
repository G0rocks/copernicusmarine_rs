# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

-

### Changed

- 

### Removed

- 

## [1.0.0] - 2025-08-13
Second release, now the crate is way more reliable and descriptive when it can't do things! Woohoo!

### Added

- Changelog.md

### Fixed

- Incorrect date string formatting is now correct even for months like september where YYYY-MM-DD usually was more like YYYY-M-DD because september is month number 9 and had no leading zero.

### Changed

- Better error messages
- Now attempts to get data from copernicusmarine server an unlimited number of times, waiting attempt_counter number of seconds in between attempts
- Exit codes work on linux

### Removed

- nc_to_csv.py because it was unused, unimplemented and unnecessary


## [0.1.0] - 2025-06-14
The first release of package with some structs and enums and a basic subset function wrapper
Glad to be here :)
Looking forward to having some more capabilities.
Was only release on crates.io, not on github.

### Added

- src/lib.rs
- .gitignore
- Cargo.lock
- Cargo.toml
- LICENSE
- README.md
- copernicusmarineservice_2025_06_04_215469291797980_transcript.txt
- nc_to_csv.py

### Fixed

- Nothing was fixed

### Changed

- Nothing was changed

### Removed

- Nothing was removed


## List of releases
[unreleased]: https://github.com/G0rocks/copernicusmarine_rs/compare/v1.0.0...main
[1.0.0]: https://github.com/G0rocks/copernicusmarine_rs/releases/tag/v1.0.0
[0.1.0]: https://github.com/G0rocks/copernicusmarine_rs/releases/tag/v0.1.0