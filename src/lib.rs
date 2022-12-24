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

// mod tweener;
mod tweens;

#[cfg(feature = "glam")]
mod glam;

use core::{
    fmt::Debug,
    ops::{Add, AddAssign, Rem, RemAssign, Sub, SubAssign},
};

// pub use tweener::*;
pub use tweens::*;

/// This is the core trait of the Library, which all tweens implement.
pub trait Tween<Value> {
    /// The Time parameter over which this tween operates. You can use a float
    /// to mean "seconds", or, in many games, an integer to mean "frames".
    type Time: TweenTime;

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
    fn percent(&mut self, current_time: Self::Time, duration: Self::Time) -> f64 {
        current_time.to_f64() / duration.to_f64()
    }
}

/// A Tweener is a wrapper around a Tween. Although you can tween dynamically using just a raw
/// Tween, this will manage all state, and allow you to work in *relative absolute time*, rather
/// than *percentage* time.
///
/// In situations where instead of knowing relative absolute time, you instead more easily know
/// delta times, or even fixed delta times, such as in most game engines, you'll want to look at
/// [DeltaTweener] and [FixedTweener].
pub struct Tweener<Value, Time, T> {
    initial_value: Value,
    final_value: Value,
    value_delta: Value,
    duration: Time,
    tween: T,
}

impl<Value, Time, T> Tweener<Value, Time, T>
where
    Value: TweenValue,
    Time: TweenTime,
    T: Tween<Value, Time = Time>,
{
    /// Creates a new [SizedTween] generically. All tweens in this library except [ElasticIn],
    /// [ElasticOut], and [ElasticInOut] implement [SizedTween].
    pub fn new(start: Value, end: Value, duration: Time) -> Self
    where
        T: SizedTween<Value>,
    {
        Self::with_tween(start, end, duration, T::new())
    }

    /// Creates a new [Tweener] out of a tween, end, and duration.
    pub fn with_tween(start: Value, end: Value, duration: Time, tween: T) -> Self {
        Self {
            initial_value: start,
            final_value: end,
            value_delta: end - start,
            duration,
            tween,
        }
    }

    /// Runs the inner Tween, returning the result.
    /// Nb: this does **not** clamp `current_time`, so if you provide a `current_time >
    /// self.duration`, chaotic results may occur.
    pub fn run(&mut self, current_time: Time) -> Value {
        let pct = self.tween.percent(current_time, self.duration);

        self.tween.tween(self.value_delta, pct) + self.initial_value
    }

    /// The initial value a tween was set to start at.
    pub fn initial_value(&self) -> Value {
        self.initial_value
    }

    /// The final value the tween should end at.
    pub fn final_value(&self) -> Value {
        self.final_value
    }

    /// Get a reference to the Tween's total duration.
    pub fn duration(&self) -> Time {
        self.duration
    }
}

impl<Value, Time> Tweener<Value, Time, SineIn<Value, Time>>
where
    Value: TweenValue,
    Time: TweenTime,
    SineIn<Value, Time>: Tween<Value, Time = Time>,
{
    /// Creates a new [SineIn] tween.
    pub fn sine_in(start: Value, end: Value, duration: Time) -> Self {
        Self::new(start, end, duration)
    }
}

#[cfg(feature = "std")]
impl<Value, Time> Tween<Value> for std::boxed::Box<dyn Tween<Value, Time = Time>>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        (**self).tween(value_delta, percent)
    }
}

/// This is a helper trait, which all the tweens in this library support, which gives access
/// to non-object-safe methods.
pub trait SizedTween<Value>: Tween<Value> + Sized {
    /// Creates a new `SizedTween`
    fn new() -> Self;
}

#[cfg(test)]
static_assertions::assert_obj_safe!(Tween<i32, Time = i32>);

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
