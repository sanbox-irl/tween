mod tweener;
mod tweens;

pub use tweener::*;
pub use tweens::*;

use std::ops::RangeInclusive;

/// This is the core trait of the Library, which all `tweens` implement.
///
/// If you choose to use a Tween directly, rather than through a `DeltaTweener`
/// or `FixedDeltaTweener`, you'll rarely deal with this directly.
pub trait Tween<TValue = f32, TTime = f32>: Sized
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    fn update(&mut self, new_time: TTime) -> TValue;

    fn range(&self) -> &RangeInclusive<TValue>;
    fn duration(&self) -> TTime;
    fn delta(&self) -> TValue;

    fn to_fixed_tweener(self, delta: TTime) -> FixedDeltaTweener<Self, TValue, TTime> {
        FixedDeltaTweener::new(self, delta)
    }
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
    fn scale(self, scale: f32) -> Self;
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
    fn percent(duration: Self, current_time: Self) -> f32;
    fn add(self, other: Self) -> Self;
    fn is_complete(self, duration: Self) -> bool;
}

/// This is internal to the library, but allows for simple numeric
/// types to be made into a time value.
macro_rules! create_time_value {
    ($t:ty) => {
        impl TweenTime for $t {
            const ZERO: Self = 0;

            fn percent(duration: Self, current_time: Self) -> f32 {
                current_time as f32 / duration as f32
            }

            fn add(self, other: Self) -> Self {
                self + other
            }

            fn is_complete(self, duration: Self) -> bool {
                self >= duration
            }
        }
    };
    (float $t:ty) => {
        impl TweenTime for $t {
            const ZERO: Self = 0.0;

            fn percent(duration: Self, current_time: Self) -> f32 {
                current_time as f32 / duration as f32
            }

            fn add(self, other: Self) -> Self {
                self + other
            }

            fn is_complete(self, duration: Self) -> bool {
                self >= duration
            }
        }
    };
}

create_time_value!(i32);
create_time_value!(i64);
create_time_value!(u32);
create_time_value!(u64);
create_time_value!(usize);
create_time_value!(isize);
create_time_value!(float f32);
create_time_value!(float f64);

/// This is internal to the library, but allows for simple numeric
/// types to be made into a tween_value.
macro_rules! tween_value_num {
    ($t:ty) => {
        impl TweenValue for $t {
            fn add(self, other: Self) -> Self {
                self + other
            }

            fn calculate_delta(destination: Self, start: Self) -> Self {
                destination - start
            }

            fn scale(self, scale: f32) -> Self {
                (self as f32 * scale) as $t
            }
        }
    };
}

tween_value_num!(f32);
tween_value_num!(f64);
tween_value_num!(i32);
tween_value_num!(i64);
tween_value_num!(u32);
tween_value_num!(u64);
tween_value_num!(usize);
tween_value_num!(isize);
