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

    /// Returns a percent, which is used in Tweeners as the `percent` argument in [Tween::tween].
    /// All Tweens in this library *expect* [Bounce] use this default implementation.
    fn percent(&self, current_time: Time, duration: Time) -> f64 {
        current_time.to_f64() / duration.to_f64()
    }

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

/// This is a helper trait, which all the tweens in this library support, which gives access
/// to non-object-safe methods.
pub trait SizedTween<Value, Time: TweenTime>: Tween<Value, Time> + Sized {
    /// Creates a new `SizedTween`
    fn new() -> Self;
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

    /// Does the `div_euclid`. Most types simply have this method already!
    fn div_euclid(self, other: Self) -> Self;
}

declare_time!(u8);
declare_time!(i8);
declare_time!(i32);
declare_time!(i64);
declare_time!(u32);
declare_time!(u64);
declare_time!(usize);
declare_time!(isize);
declare_time!(float f32);
declare_time!(float f64);

declare_value!(u8);
declare_value!(i8);
declare_value!(i32);
declare_value!(i64);
declare_value!(u32);
declare_value!(u64);
declare_value!(usize);
declare_value!(isize);
declare_value!(float f32);
declare_value!(float f64);
