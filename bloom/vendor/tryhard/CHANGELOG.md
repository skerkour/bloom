# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- N/A

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.2.0] - 2020-11-25
### Changed
- Changed from using Tokio 0.2 by default to using Tokio 0.3. You can switch back to Tokio 0.2 by declaring your dependency with `tryhard = { version = "your-version", default-features = false, features = ["tokio-02"] }`.

## [0.1.0] - 2020-11-21
### Added
- First release!

[Unreleased]: https://github.com/EmbarkStudios/tryhard/compare/0.2.0...HEAD
[0.2.0]: https://github.com/EmbarkStudios/tryhard/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/EmbarkStudios/tryhard/releases/tag/0.1.0
