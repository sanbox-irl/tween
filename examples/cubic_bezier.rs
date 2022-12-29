//! In this example, we implement a Cubic Bezier function as a tween.
//!
//! We do it two ways -- first as a closure, which we pass around unnamed (which means,
//! of course, we can't store it in a struct), and then as a struct with an implementation.
//! For fun, that second implementation also overrides `is_finite`, producing a Bezier Tween
//! which goes...on and on...and on! You probably would never want to do that directly on a Tween
//! which isn't about "looping" (for that use `Looper` or `Oscillator`). Instead, if you actually
//! wanted that, leave the default implementation and then wrap our `CubicBezier` in `Extrapolator`
//! when you want to extrapolate.

use std::ops::{Add, Sub};

use tween::{Tween, TweenValue, Tweener};

fn main() {
    let start = Point(0.0, 0.0);
    let destination = Point(10.0, 0.0);
    let duration = 10.0;

    let quarter_pt = Point(start.0 + destination.0 * 0.25, 10.0);
    let three_quarter_pt = Point(start.0 + destination.0 * 0.75, 10.0);

    // there are a couple ways to do this:
    //
    // first, let's implement it as just as closure:

    let mut tweener = cubic_bezier_closure(start, destination, duration, quarter_pt, three_quarter_pt);
    assert_eq!(tweener.move_to(5.0), Point(5.0, 7.5));
    assert_eq!(tweener.move_to(10.0), Point(10.0, 0.0));

    // secondly, let's implement it using a for real struct
    // notice how we override `is_finite` so you can do some terrible, weird cubic beziers that are way
    // too smooth.
    let mut tweener = cubic_bezier_for_real(start, destination, duration, quarter_pt, three_quarter_pt);
    assert_eq!(tweener.move_to(5.0), Point(5.0, 7.5));
    assert_eq!(tweener.move_to(10.0), Point(10.0, 0.0));

    // normal stuff...
    assert_eq!(tweener.move_to(20.0), Point(5.0, -60.0));
}

// notice the `impl` type here
fn cubic_bezier_closure(
    start: Point,
    destination: Point,
    duration: f32,
    quarter_pt: Point,
    three_quarter_pt: Point,
) -> Tweener<Point, f32, impl Tween<Point>> {
    // this closure takes a `value_delta` and a `t`:
    //
    // `value_delta` is just `destination - start`.
    // `t` is an f32 in the range (0.0..=1.0).
    //
    // Notice that we *don't* use `start` and `destination` in the closure.
    // That's because a `Tweener` automatically adds the result of this closure
    // back to the `start` value, so we don't want to use `start` ourselves. If we did,
    // we would end up "double adding" `start`. In this example, `start` is `(0.0, 0.0)`,
    // so it wouldn't matter, but if we weren't tweening off the origin, it would be
    // very important!
    //
    // finally, we've written our own lerp for simplicity, but we actually could use `Linear`
    // within the tween ourself, but it'd have to be `Linear.tween(b - a) + a`, which is not exactly
    // clear.
    Tweener::new(start, destination, duration, move |delta, t| {
        let a = Point(0.0, 0.0).lerp(quarter_pt, t);
        let b = quarter_pt.lerp(three_quarter_pt, t);
        let c = three_quarter_pt.lerp(delta, t);

        let d = a.lerp(b, t);
        let e = b.lerp(c, t);

        d.lerp(e, t)
    })
}

/// These are our two control points
pub struct CubicBezier<T>(T, T);

/// This is going to be a strictly speaking **better* implementation:
/// we're going to implement our Tween generically here. That means, although we'll
/// only use Points in this example, you could use this to cubic bezier tween anything.
fn cubic_bezier_for_real(
    start: Point,
    destination: Point,
    duration: f32,
    quarter_pt: Point,
    three_quarter_pt: Point,
) -> Tweener<Point, f32, CubicBezier<Point>> {
    impl<T: TweenValue> Tween<T> for CubicBezier<T> {
        fn tween(&mut self, delta: T, t: f32) -> T {
            // we need to write our own lerp with the generic functions available to us
            fn lerp<T: TweenValue>(a: T, b: T, t: f32) -> T {
                (b - a).scale(t) + a
            }

            // cheeky way to get a zero
            let zero = delta.scale(0.0);

            let a = lerp(zero, self.0, t);
            let b = lerp(self.0, self.1, t);
            let c = lerp(self.1, delta, t);

            let d = lerp(a, b, t);
            let e = lerp(b, c, t);

            lerp(d, e, t)
        }

        // oh yeah, we're wild
        fn is_finite(&self) -> bool {
            false
        }
    }

    Tweener::new(start, destination, duration, CubicBezier(quarter_pt, three_quarter_pt))
}

// <-- Below is math stuff that any math lib would have -->

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(f32, f32);

impl Point {
    /// Moves us towards the other Point by a factor of `t`
    fn lerp(self, other: Self, t: f32) -> Self {
        self.scale(1.0 - t) + other.scale(t)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl TweenValue for Point {
    fn scale(self, scale: f32) -> Self {
        Self(self.0 * scale, self.1 * scale)
    }
}
