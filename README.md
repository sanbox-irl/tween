# tween

![docs.rs](https://img.shields.io/docsrs/tween)
![Crates.io](https://img.shields.io/crates/v/tween)
![Crates.io](https://img.shields.io/crates/l/tween)

`tween` is an std-optional tweening library, designed for use in games and animations.

## Quick Start

To install, add the following to your Cargo.toml:

```toml
tween = { git = "https://github.com/sanbox-irl/tween" } 
```

You can make a tweener, which drives a tween, like this:

```rust
use tween::Tweener;

let (start, end) = (0, 100);
let duration = 15.0;

let mut tweener = Tweener::sine_in_out(start, end, duration);
let mut position = 0;

const DT: f32 = 1.0 / 60.0;

// and then in your main loop...
loop {
    position = tweener.move_by(DT);
    if tweener.is_finished() {
        break;
    }
}

assert_eq!(position, 100, "we've moved to the end of the tween");
```

## Overview

This library exposes three kinds of structs:

- Zero-Sized Tweens which implement the `Tween` trait. They also expose the method `tween` inherently, so you can tween easily with them, like `tween::Linear.tween`.
- Wrapper Tweens which implement the `Tween` trait. These are `Looper`, `Oscillator` and `Extrapolator`. These all wrap *around* other Tweens. See their documentation for more information.
- `Tweener` and `FixedTweener`, both of which "drive" a `Tween`. You should use `FixedTweener` in a fixed timestep application; otherwise, use `Tweener`. Although you can use a `Tween` directly, a `Tweener` manages all the Tween state for you.

For 99% of users, you'll want to construct `Tweener`s or `FixedTweener`s with a Tween for this library, occasionally looping or oscillating them.

## Making Tweens Yourself

If you'd like to make your own tween, you absolutely can! For that, you'll need to see the main trait of this library: `Tween`. You can prototype your own `Tween` implementations with a simple closure, since `FnMut(value_delta: Value, percent: f32) -> Value` implements `Tween`, or just use that yourself. Closures are very nice because you can add, or even composite, tweens in interesting ways.

For example, here's a `Linear` tween averaged with a `SineIn` tween:

```rust
use tween::{Tweener, SineIn, Linear};

Tweener::new(0.0, 10.0, 10, |value_delta, percent| {
    (Linear.tween(value_delta, percent) + SineIn.tween(value_delta, percent)) / 2.0
});
```

To see a documented example of a Cubic Bezier Tween, see `examples/cubic_bezier.rs`.

## Storing Tweens

Very often in a game or animation engine, you'll want to store Tweens by what they act *on*, without caring about what kind of Tween example it is. To do that, you'll want to *erase* the Tweener.

```rust no_run
use tween::{Tweener, ErasedTweener, Looper, Linear, SineIn};

let mut my_tweener: Box<dyn ErasedTweener<i32, u32>> = Tweener::new(0, 100, 100, Linear).into_erased();
let mut going_up = true;

// we lerp from 0 to 100 over 100 frames, and then we flip our tween back into a SineIn tween over 10 frames, so this looks like a slowwwwwww buildup and then a SHARP drop down.
loop {
    let _output_assigned_somewhere = my_tweener.move_by(1);
    if my_tweener.is_finished() {
        my_tweener = if going_up {
            Tweener::new(100, 0, 10, SineIn).into_erased()
        } else {
            Tweener::new(0, 100, 100, Linear).into_erased()
        };
        going_up = !going_up;
    }
}
```

## Implementing `TweenValue`

This library uses two traits: `TweenTime` and `TweenValue`. You can implement these yourself, but implementing `TweenTime` would only have fairly obscure uses.

On the other hand, `TweenValue` needs to be implemented for any tweenable value. By default, all numerical types are already implemented in this library. Additionally, several math libs have a feature flag (see below) which gates an implementation for their structs as appropriate.

If you'd like to PR a math library's types to implement `TweenValue` in this crate, please let me know.

## Going Fast ⚡️

This library is, ultimately, a math library, and benefits enormously from being in release mode and having link-time optimization enabled. If Debug releases feel slow, consider having this compiled in release mode nonetheless, and enabling LTO.

## Features

`tween` has the following features:

- `std`: **enabled by default**, gives access to faster floating point math and helper methods with `Box`
- `libm`: enable this, without default features, for no-std tweening
- `glam`: enable this for `glam` types to implement `TweenValue`

## Std Optional

This library uses `std` with the default feature `std`. Disable default features, and enable `libm`, for a no-std experience. (We need to use `libm` for the floating point math), like so:

```toml
tween = { verison = "2.0.0", default_features = false, features = ["libm"] }
```

## MSRV and Safety

This crate has no MSRV yet. If it sees good adoption, an MSRV policy will be decided.

Additionally, this crate is `#![deny(unsafe_code)]`, since no unsafe code was needed. Changing this policy would constitute a minor breaking change.

## Roadmap

Next up for this library is handling Splines of Tweeners.

## License

Dual-licensed under MIT or APACHE 2.0.
