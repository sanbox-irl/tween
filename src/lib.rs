#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![no_std]

#[cfg(any(feature = "std"))]
#[macro_use]
extern crate std;

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
compile_error!(
    "Please enable feature `libm` (You used `no-default-features`, turning off `std`, but we need `libm` for `f64` math.)"
);

#[macro_use]
mod macros;

mod tweener;
mod tweens;

#[cfg(feature = "glam")]
mod glam;

use core::{
    fmt::Debug,
    ops::{Add, AddAssign, Rem, RemAssign, Sub, SubAssign},
};

pub use tweener::*;
pub use tweens::*;

/// This is the core trait of the Library, which all tweens implement.
pub trait Tween<Value, Time: TweenTime> {
    /// Returns a new value based on the value_delta and the percent.
    ///
    /// [Linear], for example, is implemented simply as:
    /// ```no_test
    /// value_delta.scale(percent)
    /// ```
    /// which is just `value_delta * percent`.
    fn tween(&mut self, value_delta: Value, percent: f64) -> Value;

    /// This returns *inclusive percentage* bounds under which a tween is valid. A "percentage"
    /// range is an f64 denoting the percentage of a tween, and it is inclusive so that top and
    /// bottom numbers are both within the range.
    ///
    /// This is used by [Tweener], [DeltaTweener], and [FixedTweener] to determine returning [Some]
    /// or [None].
    ///
    /// If you have a [Tween] which returns valid values at all percentage sranges at all times, you
    /// should return [None].
    ///
    /// All normal Tweens in this library use the default method, which means that a Tweener,
    /// whereas [Looper] and [Oscillator] both override it to return [None].
    fn percent_bounds(&self) -> Option<(f64, f64)> {
        Some((0.0, 1.0))
    }

    /// Convenience shortcut to turn a tween into a [Looper].
    fn into_loop(self) -> Looper<Self>
    where
        Self: Sized,
    {
        Looper::new(self)
    }

    /// Convenience shortcut to turn a tween into an [Oscillator].
    fn into_oscillator(self) -> Oscillator<Self>
    where
        Self: Sized,
    {
        Oscillator::new(self)
    }
}

#[cfg(test)]
static_assertions::assert_obj_safe!(Tween<i32, i32>);

#[cfg(feature = "std")]
impl<Value, Time> Tween<Value, Time> for std::boxed::Box<dyn Tween<Value, Time>>
where
    Value: TweenValue,
    Time: TweenTime,
{
    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        (**self).tween(value_delta, percent)
    }
}

impl<Value, Time, F> Tween<Value, Time> for F
where
    F: FnMut(Value, f64) -> Value,
    Value: TweenValue,
    Time: TweenTime,
{
    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        self(value_delta, percent)
    }
}

#[cfg(test)]
static_assertions::assert_obj_safe!(Tween<i32, i32>);

/// A `TweenValue` is a value which *can* be Tweened. The library fundamentally outputs
/// `TweenValue` eventually.
///
/// If you want to implement your own values to be tweened (for example, your favorite color lib),
/// then you'll need to implement this trait.
///
/// For now, we require `Copy`, but can reduce this to a `Clone` implementation. Please file an
/// issue if that is needed for your workflow.
pub trait TweenValue:
    Copy + PartialEq + Debug + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign
{
    /// The ZERO value. Generally, this is 0 or 0.0.
    const ZERO: Self;

    /// This should be implemented as a simple multiplication. For f32, for example,
    /// it's implemented as `(self as f64 * scale) as f32`.
    fn scale(self, scale: f64) -> Self;
}

/// A `TweenTime` is a representation of Time. The two most common will be `f32`/`f64` for
/// seconds and `u32`/`u64`/`usize` for frames.
///
/// If you want to implement your own time for duration, then you'll need to implement this
/// trait somewhere.
///
/// For now, we require `Copy`, but can reduce this to a `Clone` implementation. Please file an
/// issue if that is needed for your workflow.
pub trait TweenTime:
    Copy
    + PartialEq
    + PartialOrd
    + Debug
    + Add<Output = Self>
    + AddAssign
    + Rem<Output = Self>
    + RemAssign
    + Sub<Output = Self>
    + SubAssign
{
    /// The ZERO value. This is 0 or 0.0.
    const ZERO: Self;

    /// This should be implemented as a simple division. For f32, for example,
    /// it's implemented as `(current_time / duration) as f64`.
    fn percent(duration: Self, current_time: Self) -> f64;

    /// Converts the given number to an `f32`.
    fn to_f32(self) -> f32;

    /// Converts the self to an `f64`. This is only used in `Elastic` in this library.
    fn to_f64(self) -> f64;

    /// This is implemented as a simple multipler, such as `self * multiplier`.
    fn scale(self, multiplier: f64) -> Self;
}

declare_time!(u8, i8, i16, u16, i32, i64, u32, u64, i128, u128, usize, isize);

impl TweenTime for f32 {
    const ZERO: Self = 0.0;

    fn percent(duration: Self, current_time: Self) -> f64 {
        current_time as f64 / duration as f64
    }

    fn to_f32(self) -> f32 {
        self
    }

    fn to_f64(self) -> f64 {
        self as f64
    }

    fn scale(self, other: f64) -> Self {
        (self as f64 * other) as Self
    }
}
impl TweenTime for f64 {
    const ZERO: Self = 0.0;

    fn percent(duration: Self, current_time: Self) -> f64 {
        current_time / duration
    }

    fn to_f32(self) -> f32 {
        self as f32
    }

    fn to_f64(self) -> f64 {
        self
    }

    fn scale(self, other: f64) -> Self {
        self * other
    }
}

declare_value!(u8, i8, i16, u16, i32, i64, u32, u64, i128, u128, usize, isize);

impl TweenValue for f32 {
    const ZERO: Self = 0.0;

    fn scale(self, scale: f64) -> Self {
        (self as f64 * scale) as f32
    }
}

impl TweenValue for f64 {
    const ZERO: Self = 0.0;

    fn scale(self, scale: f64) -> Self {
        self * scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lambda_test() {
        // this tweener will always return `100` for `4` frames!
        let (start, end) = (0, 1); // this is basically ignored by the Lambda Tween
        let length = 4;
        let mut pct_50_or_over = false;
        let mut tweener = Tweener::new(start, end, length, |_vd, pct| {
            if pct >= 0.5 {
                pct_50_or_over = true;
            }
            100
        });
        assert_eq!(tweener.move_by(1), 100);
        assert_eq!(tweener.move_by(1), 100);
        assert_eq!(tweener.move_by(1), 100);
        assert_eq!(tweener.move_by(1), 100);
        // because we're clamped to the tween's original bounds!
        assert_eq!(tweener.move_by(1), 1);
        assert!(tweener.is_finished());
        assert!(pct_50_or_over);
    }
}
