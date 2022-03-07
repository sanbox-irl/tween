#[macro_use]
mod macros;

mod tweener;
mod tweens;

pub use tweener::*;
pub use tweens::*;

use std::ops::RangeInclusive;

/// This is the core trait of the Library, which all `tweens` implement.
///
/// If you choose to use a Tween directly, rather than through a `DeltaTweener`
/// or `FixedDeltaTweener`, you'll rarely deal with this directly.
pub trait Tween: Sized {
    type Value: TweenValue;
    type Time: TweenTime;

    fn update(&mut self, new_time: Self::Time) -> Self::Value;

    fn range(&self) -> &RangeInclusive<Self::Value>;
    fn duration(&self) -> Self::Time;

    // fn to_fixed_tweener(
    //     self,
    //     delta: Self::TTime,
    // ) -> FixedDeltaTweener<Self, Self::TValue, Self::TTime> {
    //     FixedDeltaTweener::new(self, delta)
    // }
}

/// A `TweenValue` is a value which *can* be Tweened. The library fundamentally outputs
/// `TweenValue` eventually.
///
/// If you want to implement your own values to be tweened (for example, your favorite color lib),
/// then you'll need to implement this trait.
///
/// For now, we require `Copy`, but can reduce this to a `Clone` implementation. Please file an issue
/// if that is needed for your workflow.
pub trait TweenValue: Copy {
    fn calculate_delta(destination: Self, start: Self) -> Self;
    fn add(self, other: Self) -> Self;
    fn scale(self, scale: f64) -> Self;
}

/// A `TweenTime` is a representation of Time. The two most common will be `f32`/`f64` for
/// seconds and `u32`/`u64`/`usize` for frames.
///
/// If you want to implement your own time for duration, then you'll need to implement this
/// trait somewhere.
///
/// For now, we require `Copy`, but can reduce this to a `Clone` implementation. Please file an issue
/// if that is needed for your workflow.
pub trait TweenTime: Copy {
    const ZERO: Self;
    fn percent(duration: Self, current_time: Self) -> f64;
    fn add(self, other: Self) -> Self;
    fn is_complete(self, duration: Self) -> bool;
}

declare_time!(i32);
declare_time!(i64);
declare_time!(u32);
declare_time!(u64);
declare_time!(usize);
declare_time!(isize);
declare_time!(float f32);
declare_time!(float f64);

declare_value!(f32);
declare_value!(f64);
declare_value!(i32);
declare_value!(i64);
declare_value!(u32);
declare_value!(u64);
declare_value!(usize);
declare_value!(isize);
