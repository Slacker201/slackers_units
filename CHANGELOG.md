# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.3.0...slackers_units-v0.3.1) - 2026-11-11

### Other

- runtime parser test coverage
- implement basic arith tests

## [0.3.0](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.2.4...slackers_units-v0.3.0) - 2026-11-10

### Added

- *(runtime parser)* implement basic mul and div parsing

### Fixed

- *(runtime parser)* fix runtime parser hardcoding 256B ratio

### Other

- reset default converter feature, cleanup backer trait
- update rust.yml settings

## [0.2.4](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.2.3...slackers_units-v0.2.4) - 2026-11-10

### Other

- update ci

## [0.2.3](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.2.2...slackers_units-v0.2.3) - 2026-11-09

### Fixed

- make generated code reflect converter features

### Other

- more tests

## [0.2.2](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.2.1...slackers_units-v0.2.2) - 2026-11-09

### Added

- *(unit conversion)* implement features for 1,2,4,8,16,32,64,128,256 byte integer ratio for prefix conversion
- feat(unit conversion):

### Other

- Merge pull request #27 from Slacker201/dev

## [0.2.1](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.2.0...slackers_units-v0.2.1) - 2026-11-09

### Other

- Merge pull request #25 from Slacker201/fixes
- implement build fail tests for empty system, dup abbrev, dup units, undefined base unit

## [0.2.0](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.1.6...slackers_units-v0.2.0) - 2026-11-09

### Added

- implement identity conversion

### Other

- [**breaking**] fix tests. breaking cuz i forgot b4
- Create rust.yml

## [0.1.6](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.1.5...slackers_units-v0.1.6) - 2026-11-09

### Added

- implement default representation field

## [0.1.5](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.1.4...slackers_units-v0.1.5) - 2026-11-08

### Other

- implement prefix multiplier test

## [0.1.4](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.1.3...slackers_units-v0.1.4) - 2026-11-08

### Added

- *(ci)* implement ci v1

### Fixed

- update changelog again
- *(ci)* try and fix ci

### Other

- Update CHANGELOG.md
- release v0.1.4
- release v0.1.4
- prep 0.1.4
- release v0.1.3

## [0.1.3](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.1.1...slackers_units-v0.1.3) - 2026-11-07

### Added

- rewrite to use Ratio over fastnum::Dec512
- *(macro)* implement support for generic backers

### Fixed

- *(unit macro)* fix off by 1 parsing

### Other

- bump version
- release
- update. also versioning is goofy rn
- *(runtime parser)* refactor runtime parser to decrease bug surface and increase efficiency
- Merge branch 'main' into bug-fix-one
- release v0.1.2
- release

## [0.1.2](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.1.1...slackers_units-v0.1.2) - 2026-11-07

### Added

- rewrite to use Ratio over fastnum::Dec512

### Fixed

- *(unit macro)* fix off by 1 parsing

### Other

- update. also versioning is goofy rn
- *(runtime parser)* refactor runtime parser to decrease bug surface and increase efficiency
- Merge branch 'main' into bug-fix-one
- release v0.1.2

## [0.1.1](https://github.com/Slacker201/slackers_units/compare/slackers_units-v0.1.0...slackers_units-v0.1.1) - 2026-11-05

### Added

- *(macro)* implement support for generic backers

## [0.1.0](https://github.com/Slacker201/slackers_si_units/releases/tag/slackers_units-v0.1.0) - 2026-11-03

### Added

- *(macro)* implement super basic compile time unit parser macro
- *(macro)* implement stub for `unit` macro rewrite
- *(macro)* implement basic `unit!` macro
- *(generator)* implement is_prefixable for runtime parser
- *(generator)* implement generation of the Unit type
- *(validator)* make validation return `ValidatedSystem` with extra metadata
- *(generator)* hook up parser generator to main lib exports
- *(span)* implement spans for all types in `system!` macro
- *(types)* [**breaking**] basic algebraic type system
- *(serialization)* implement serialization and deserialization for backers that support it
- *(macro)* implement support for Unit types
- *(backers)* implement Backer for FixedI128<U8/U16/U32/U64/U96>
- *(unit types)* implement macro for generating unit types
- *(f64 backer)* implement basic support for f64 backers
- *(unit macro)* implement basic macro parsing

### Fixed

- *(serde)* add serde/derive to serde feature

### Other

- rename crates for 0.1.0 release
- *(cleanup)* apply fmt and clippy
- [**breaking**] delete old library code
- *(macro)* [**breaking**] move si_unit_parser_macros implementation in si_unit_macro
- implement ast
- release
- add serde dep
- release
- *(version)* bump version
- *(deps)* fix typo
- *(deps)* add description
- *(licence)* add licence
- *(deps)* add version to deps
- *(dev testing)* remove testing crate
- *(dev testing)* ignore test crate
- chore(fmt)
- *(dependencies)* added and re-exported the dependancies
- *(types)* init types crate
- initial commit
