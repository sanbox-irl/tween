# Changelog

## [unreleased]

- **BREAKING**: `FixedTweener` shortcut constructors ending in `_at`, such as `FixedTweener::linear_at`,
  now take a `delta` and return a `FixedTweener`. They previously returned a `Tweener`.
- Fixed `BounceInOut` never reaching its end value for odd integer deltas.
- Fixed integer tweens rounding more than once in `QuadOut`, `SineIn`, `BounceIn`, `ElasticIn`,
  `ElasticOut`, and `ElasticInOut`.
- Fixed zero duration Tweeners returning `NaN` and never finishing at time `0`.
- Fixed `&mut dyn Tween` always reporting `is_finite` as `true`, and made it available without `std`.
- Fixed finished Tweeners not returning exactly their end value due to floating point error. A
  finite Tween's output is now replaced by the end value at exactly `100%`, not only beyond it.

## [2.2.0] - 2026-02-24
- Bumped math versions again again again.
- Bumped MSRV to current Rust, and move version to `rust_version = 2024`.


## [2.1.0] - 2025-05-12
- **BREAKING**: Removed support for `vek`, `ultraviolet`, and `cgmath`, as they provided a maintenance
  burden without apparent need.
- Bumped math versions again again again.
- Bumped MSRV to current Rust, and move version to `rust_version = 2024`.

## [2.0.4] - 2024-01-02

- Bumped math versions again again.

## [2.0.3] - 2023-04-02

- Bumped math versions again.

## [2.0.2] - 2023-12-10

- Bumped math versions.

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
