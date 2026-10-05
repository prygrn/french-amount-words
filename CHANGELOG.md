# Changelog

All notable changes to this crate are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this crate
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). Each published version
matches the git tag `vX.Y.Z`.

## [Unreleased]

## [0.1.1] - 2026-10-05

### Added

- Link to the docs.rs documentation in the package metadata.
- README: badges (version, documentation, license, CI).
- README: positioning relative to the `french-numbers` and `nb2fr` crates.

### Changed

- Documentation translated to English.

## [0.1.0] - 2026-10-05

### Added

- `number_to_words`: spells out an integer in words, following the traditional spelling.
- `euro_amount_to_words`: spells out in words a euro amount given in cents.
- `MAX_VALUE`: largest convertible value (`999_999_999_999`).
- `AmountWordsError::ValueTooLarge`: error returned above `MAX_VALUE`.
- `AmountWordsError` is `#[non_exhaustive]`: a `match` outside the crate must include a `_`
  arm, which allows adding variants without breaking compatibility.

[Unreleased]: https://github.com/prygrn/french-amount-words/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/prygrn/french-amount-words/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/prygrn/french-amount-words/releases/tag/v0.1.0
