# Changelog

## [unreleased] - 2023-01-01

## [2.0.0] - 2023-01-01

- Rewrote the library:
  - `Tween` is generic about only `Value`,
  - All `Tween`s have become ZSTs, and all state has moved to `Tweener`s.
  - The library no longer uses Ranges.
  - Added `Looper`, `Oscillator`, and `Extrapolator`

## [1.0.1] - 2022-04-08

- Updated typings on the Tweener to be less difficult.
- Added getters to the Tweener.

## [1.0.0] - 2022-04-03

- Initial implementation of the library.

[unreleased]: https://github.com/sanbox-irl/tween/compare/v1.0.0...HEAD
[1.0.1]: https://github.com/sanbox-irl/tween/releases/tag/v1.0.1
[1.0.0]: https://github.com/sanbox-irl/tween/releases/tag/v1.0.0
