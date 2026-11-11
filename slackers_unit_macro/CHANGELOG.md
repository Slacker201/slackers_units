# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.10](https://github.com/Slacker201/slackers_units/compare/slackers_unit_macro-v0.1.9...slackers_unit_macro-v0.1.10) - 2026-11-11

### Added

- *(runtime parser)* implement number suffix parsing with inferred multiply

### Fixed

- *(generator)* fix generated div impl adding unit vectors
- *(generator)* add arith bounds to backing repr for arith impls

### Other

- better error message for missing default representation declaration
- make rust analyzer default to braces for system! macro

## [0.1.9](https://github.com/Slacker201/slackers_units/compare/slackers_unit_macro-v0.1.8...slackers_unit_macro-v0.1.9) - 2026-11-10

### Added

- *(runtime parser)* implement basic mul and div parsing

### Fixed

- *(runtime parser)* make suffix parser return correct remainder
- *(runtime parser)* fix runtime parser hardcoding 256B ratio

### Other

- reset default converter feature, cleanup backer trait
- *(runtime parser)* refactor runtime parser

## [0.1.8](https://github.com/Slacker201/slackers_units/compare/slackers_unit_macro-v0.1.7...slackers_unit_macro-v0.1.8) - 2026-11-09

### Fixed

- make generated code reflect converter features

## [0.1.7](https://github.com/Slacker201/slackers_units/compare/slackers_unit_macro-v0.1.6...slackers_unit_macro-v0.1.7) - 2026-11-09

### Added

- *(unit conversion)* implement features for 1,2,4,8,16,32,64,128,256 byte integer ratio for prefix conversion
- feat(unit conversion):

### Other

- Merge pull request #27 from Slacker201/dev

## [0.1.6](https://github.com/Slacker201/slackers_units/compare/slackers_unit_macro-v0.1.5...slackers_unit_macro-v0.1.6) - 2026-11-09

### Other

- implement build fail tests for empty system, dup abbrev, dup units, undefined base unit

## [0.1.5](https://github.com/Slacker201/slackers_units/compare/slackers_unit_macro-v0.1.4...slackers_unit_macro-v0.1.5) - 2026-11-09

### Added

- implement default representation field
- *(generator)* increased macro hygiege

## [0.1.4](https://github.com/Slacker201/slackers_units/compare/slackers_unit_macro-v0.1.3...slackers_unit_macro-v0.1.4) - 2026-11-08

### Added

- *(generator)* make `unit!` work regardless of where system! is declared, so long as it is reachable
- *(generator)* make generation more hygienic

### Fixed

- *(changelog)* fix dup entries

### Other

- Update CHANGELOG.md
- release v0.1.4
- release v0.1.4
- prep 0.1.4
- release v0.1.3

## [0.1.3](https://github.com/Slacker201/slackers_units/compare/slackers_unit_macro-v0.1.1...slackers_unit_macro-v0.1.3) - 2026-11-07

### Added

- rewrite to use Ratio over fastnum::Dec512
- *(runtime parser)* implement better error messages
- *(macro)* implement support for generic backers

### Fixed

- *(unit macro)* fix off by 1 parsing
- *(repo url)* fix repo url
- *(validator)* make validator use declaration order for base-type mapping
- *(generator)* make `Unit` struct actually use B as backer type
- *(parser)* make words sharing the same prefix not overwrite each other
- *(generator)* fix None prefix type being emitted multiple times
- *(compile time parser)* fix bad error
- *(parser)* apply prefix, group power, and sign
- *(generator)* replace old library name with correct path

### Other

- bump version
- release
- remove eprintln
- *(generator)* remove translation tables
- *(generator)* rename variable to be clearer
- update. also versioning is goofy rn
- apply fmt and clippy
- rename functions for clarity
- add explanation for unsafe usage
- *(runtime parser)* refactor runtime parser to decrease bug surface and increase efficiency
- Merge branch 'main' into bug-fix-one
- release v0.1.2
- release

## [0.1.2](https://github.com/Slacker201/slackers_units/compare/slackers_unit_macro-v0.1.1...slackers_unit_macro-v0.1.2) - 2026-11-07

### Added

- rewrite to use Ratio over fastnum::Dec512
- *(runtime parser)* implement better error messages
- *(macro)* implement support for generic backers

### Fixed

- *(unit macro)* fix off by 1 parsing
- *(repo url)* fix repo url
- *(validator)* make validator use declaration order for base-type mapping
- *(generator)* make `Unit` struct actually use B as backer type
- *(parser)* make words sharing the same prefix not overwrite each other
- *(generator)* fix None prefix type being emitted multiple times
- *(compile time parser)* fix bad error
- *(parser)* apply prefix, group power, and sign
- *(generator)* replace old library name with correct path

### Other

- remove eprintln
- *(generator)* remove translation tables
- *(generator)* rename variable to be clearer
- update. also versioning is goofy rn
- apply fmt and clippy
- rename functions for clarity
- add explanation for unsafe usage
- *(runtime parser)* refactor runtime parser to decrease bug surface and increase efficiency
- Merge branch 'main' into bug-fix-one
- release v0.1.2
- release

## [0.1.1](https://github.com/Slacker201/slackers_units/compare/slackers_unit_macro-v0.1.0...slackers_unit_macro-v0.1.1) - 2026-11-05

### Added

- *(macro)* implement support for generic backers

### Fixed

- *(validator)* make validator use declaration order for base-type mapping
- *(generator)* make `Unit` struct actually use B as backer type
- *(parser)* make words sharing the same prefix not overwrite each other
- *(generator)* fix None prefix type being emitted multiple times
- *(compile time parser)* fix bad error
- *(parser)* apply prefix, group power, and sign
- *(generator)* replace old library name with correct path

## [0.1.0](https://github.com/Slacker201/slackers_si_units/releases/tag/slackers_unit_macro-v0.1.0) - 2026-11-03

### Other

- rename crates for 0.1.0 release
