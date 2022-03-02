mod tweener;
mod tweens;

pub use tweener::*;
pub use tweens::*;

use std::ops::RangeInclusive;

pub trait Tween<TValue, TTime>: Sized
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

pub trait TweenValue: Copy {
    fn calculate_delta(destination: Self, start: Self) -> Self;
    fn add(self, other: Self) -> Self;
    fn scale(self, scale: f32) -> Self;
}

macro_rules! create_tween_value {
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

create_tween_value!(f32);
create_tween_value!(f64);
create_tween_value!(i32);
create_tween_value!(i64);
create_tween_value!(u32);
create_tween_value!(u64);
create_tween_value!(usize);
create_tween_value!(isize);

pub trait TweenTime: Copy {
    const ZERO: Self;
    fn percent(duration: Self, current_time: Self) -> f32;
    fn add(self, other: Self) -> Self;
    fn is_complete(self, duration: Self) -> bool;
}

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
    ($t:ty,$d:expr) => {
        impl TweenTime for $t {
            const ZERO: Self = $d;

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
create_time_value!(f32, 0.0);
create_time_value!(f64, 0.0);
