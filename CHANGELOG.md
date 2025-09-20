<!--
Added for new features.
Changed for changes in existing functionality.
Deprecated for soon-to-be removed features.
Removed for now removed features.
Fixed for any bug fixes.
Security in case of vulnerabilities.
-->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic
Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2025-09-20

### Changed

* Improve performance of wildcard `?` cases

## [0.3.0] - 2025-09-17

### Changed

* Swap `pattern` with `haystack` in `DoWild::dowild` and `DoWild::dowild_with`.
* Improve performance of classes, especially if they are invalid. Allocate heap
  memory only when necessary and only the minimum amount instead of the maximum
  amount.
* Improve general performance of `dowild_with`.

## [0.2.0] - 2025-09-14

### Changed

* Rename `SimpleMatch` trait to `DoWild`.

### Fixed

* Library/API documentation and crate description.

## [0.1.0] - 2025-09-14

### Added

* Initial release
