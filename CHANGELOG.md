# Changelog

## [unreleased]

## [2.0.1] - 2023-02-19

- Added better support for delayed tweens, including an example in `examples/delayed_tween.rs`
- Added `new_at` which situations a tween in time. Added similar methods for shortcut constructors, such as
support for `sine_in_at`.
- Fixed the `new` method for most Tweens and added support for `Copy`, `Ord`, and `Eq`.
- Added support for `Copy`, `Ord`, and `Eq` to all Tweener adapters.
- Upgraded all math libraries

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

[unreleased]: https://github.com/sanbox-irl/tween/compare/v2.0.1...HEAD
[2.0.1]: https://github.com/sanbox-irl/tween/releases/tag/v2.0.1
[2.0.0]: https://github.com/sanbox-irl/tween/releases/tag/v2.0.0
[1.0.1]: https://github.com/sanbox-irl/tween/releases/tag/v1.0.1
[1.0.0]: https://github.com/sanbox-irl/tween/releases/tag/v1.0.0
