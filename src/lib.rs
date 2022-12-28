#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![no_std]

#[cfg(any(feature = "std"))]
#[macro_use]
extern crate std;

#[cfg(all(feature = "std", feature = "libm"))]
compile_error!("Please disable feature `libm` or disable default features -- both cannot be active at once.");

#[macro_use]
mod macros;

mod tweener;
mod tweens;

#[cfg(feature = "glam")]
mod glam;

pub use tweener::*;
pub use tweens::*;

/// This is the core trait of the Library, which all tweens implement.
pub trait Tween<Value> {
    /// Returns a new value based on the value_delta and the percent.
    ///
    /// [Linear], for example, is implemented simply as:
    /// ```no_test
    /// value_delta.scale(percent)
    /// ```
    /// which is just `value_delta * percent`.
    fn tween(&mut self, value_delta: Value, percent: f32) -> Value;

    /// All Tweens in this library use this default method, except [Looper] and [Oscillator], which
    /// are both unbounded (because they never stop returning values).
    ///
    /// This is used by [Tweener], [DeltaTweener], and [FixedTweener] to determine when to clamp and
    /// when a tween will return true for [Tweener::is_finished].
    ///
    /// If you have a [Tween] which returns valid values at all percentage ranges at all times, you
    /// should return [false].
    ///
    /// If you would like to extrapolate a tween *beyond* its bounds, you can wrap it in
    /// [Extrapolate].
    #[inline(always)]
    fn is_finite(&self) -> bool {
        true
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
static_assertions::assert_obj_safe!(Tween<i32>);

#[cfg(feature = "std")]
impl<Value> Tween<Value> for std::boxed::Box<dyn Tween<Value>>
where
    Value: TweenValue,
{
    #[inline(always)]
    fn tween(&mut self, value_delta: Value, percent: f32) -> Value {
        (**self).tween(value_delta, percent)
    }
}

impl<Value, F> Tween<Value> for F
where
    F: FnMut(Value, f32) -> Value,
    Value: TweenValue,
{
    #[inline(always)]
    fn tween(&mut self, value_delta: Value, percent: f32) -> Value {
        self(value_delta, percent)
    }
}

/// A `TweenValue` is a value which *can* be Tweened. The library fundamentally outputs
/// `TweenValue` eventually.
///
/// If you want to implement your own values to be tweened (for example, your favorite color lib),
/// then you'll need to implement this trait.
///
/// For now, we require `Copy`, but can reduce this to a `Clone` implementation. Please file an
/// issue if that is needed for your workflow.
pub trait TweenValue:
    Copy
    + PartialEq
    + core::fmt::Debug
    + core::ops::Add<Output = Self>
    + core::ops::AddAssign
    + core::ops::Sub<Output = Self>
    + core::ops::SubAssign
{
    /// This should be implemented as a simple multiplication. For f64, for example,
    /// it's implemented as `(self as f32 * scale) as f64`.
    fn scale(self, scale: f32) -> Self;
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
    + core::fmt::Debug
    + core::ops::Add<Output = Self>
    + core::ops::AddAssign
    + core::ops::Rem<Output = Self>
    + core::ops::RemAssign
    + core::ops::Sub<Output = Self>
    + core::ops::SubAssign
{
    /// The ZERO value. This is 0 or 0.0.
    const ZERO: Self;

    /// Converts the given number to an `f32`.
    fn to_f32(self) -> f32;

    /// This is implemented as a simple multipler, such as `self * multiplier`.
    fn scale(self, multiplier: f32) -> Self;
}

declare_time!(u8, i8, i16, u16, i32, i64, u32, u64, i128, u128, usize, isize);

impl TweenTime for f32 {
    const ZERO: Self = 0.0;

    #[inline(always)]
    fn to_f32(self) -> f32 {
        self
    }

    #[inline(always)]
    fn scale(self, other: f32) -> Self {
        (self * other) as Self
    }
}
impl TweenTime for f64 {
    const ZERO: Self = 0.0;

    #[inline(always)]
    fn to_f32(self) -> f32 {
        self as f32
    }

    #[inline(always)]
    fn scale(self, other: f32) -> Self {
        (self as f32 * other) as f64
    }
}

declare_value!(u8, i8, i16, u16, i32, i64, u32, u64, i128, u128, usize, isize);

impl TweenValue for f32 {
    #[inline(always)]
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl TweenValue for f64 {
    #[inline(always)]
    fn scale(self, scale: f32) -> Self {
        (self as f32 * scale) as Self
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
