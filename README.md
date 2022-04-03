# tween

`tween` is an std-optional tweening library, designed for use in games and animations.

This trait exports a trait `Tween`, a variety of structs which implement common tweens (such as `Elastic`, `Quart`, etc), and two `Tweeners`, which wrap around tweens, allowing users to drive them trivially.

First, tweens are simple to create:

```rs
use tween::SineIn;

let range = 0.0..=200.0;
let duration = 60;

let mut sine_in = SineIn::new(range, duration);
let _value0 = sine_in.update(0);
let _value1 = sine_in.update(1);
let _value2 = sine_in.update(2);
let _value3 = sine_in.update(3);
```

Notice how we work over a RangeInclusive to provide values.

However, as the above example shows, it's more typical that you'll want to simply drive a tween through time, rather than giving arbitrary times within a duration. For that, we use tweeners, which come in two kinds: `Tweener` and `FixedTweener`. For both, users provide *deltas*, rather than arbitrary times. A `FixedTweener` uses just *one* delta (appropriate in a game engine with a Fixed Update pipeline), whereas a `Tweener` can take a fixed delta.

```rs
use tween::{SineIn, FixedTweener};

let range = 0.0..=200.0;
let duration = 60;

let sine_in = SineIn::new(range, duration);
let delta = 1;
let sine_in_tweener = FixedTweener::new(sine_in, delta);

for value in sine_in_tweener {
    // FixedTweener provides an iterator interface
}
```

## Std Optional

This library uses `std` with the default feature `std`. Disable default features, and enable `libm`, for a no-std experience. (We need to use `libm` for the floating point math).

## Generics

This library uses generics heavily. There are two core generics used: `TweenValue` and `TweenTime`. All built-in numeric types implement both traits. For your own code, you can implement either trait. For example, your favorite math library could easily implement `TweenValue`.
