# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
