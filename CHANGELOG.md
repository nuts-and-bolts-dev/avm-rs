# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2025-01-14

### Added
- Stepped execution mode for debugging TEAL programs with interactive debugger interface
- Basic tracing functionality (optional feature) for execution analysis
- Comprehensive CLI documentation
- Property-based testing with proptest for enhanced test coverage
- Copyright information to all source files

### Changed
- Refactored opcodes module for better organization and maintainability
- Improved CLI options and argument handling
- Enhanced README with stepped execution documentation

### Fixed
- Various clippy warnings for improved code quality
- Test failures and improved test coverage

### Removed
- Unimplemented CLI subcommands from the interface
- Disassemble subcommand (temporarily removed)

## [0.1.1] - 2024-12-29

### Fixed
- Removed mentions of unimplemented commands from CLI help

## [0.1.0] - 2024-12-27

### Added
- Initial release of avm-rs
- Basic AVM execution functionality
- CLI interface for running TEAL programs
- Core instruction set implementation
