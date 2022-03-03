use crate::{Tween, TweenTime, TweenValue};
use std::ops::RangeInclusive;

/*

    fn ease_in(t: F, b: F, c: F, d: F) -> F {
        let t = t / d;
        c * (t * t * t) + b
    }

    fn ease_out(t: F, b: F, c: F, d: F) -> F {
        let t = t / d - f(1.0);
        c * ((t * t * t) + f(1.0)) + b
    }

    fn ease_in_out(t: F, b: F, c: F, d: F) -> F {
        let t = t / (d / f(2.0));
        if t < f(1.0) {
            c / f(2.0) * (t * t * t) + b
        }
        else {
            let t = t - f(2.0);
            c / f(2.0) * (t * t * t + f(2.0)) + b
        }
    }

*/

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CubicIn<TValue = f32, TTime = f32> {
    range: RangeInclusive<TValue>,
    value_delta: TValue,
    duration: TTime,
}

impl<TValue, TTime> CubicIn<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    pub fn new(range: RangeInclusive<TValue>, duration: TTime) -> Self {
        let delta = TValue::calculate_delta(*range.end(), *range.start());
        Self {
            range,
            value_delta: delta,
            duration,
        }
    }
}

impl<V, T> Tween<V, T> for CubicIn<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    fn update(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self
            .value_delta
            .scale(percent_time * percent_time * percent_time);

        new_value.add(*self.range.start())
    }

    fn range(&self) -> &RangeInclusive<V> {
        &self.range
    }

    fn duration(&self) -> T {
        self.duration
    }

    fn delta(&self) -> V {
        self.value_delta
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CubicOut<TValue = f32, TTime = f32> {
    range: RangeInclusive<TValue>,
    value_delta: TValue,
    duration: TTime,
}

impl<TValue, TTime> CubicOut<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    pub fn new(range: RangeInclusive<TValue>, duration: TTime) -> Self {
        let delta = TValue::calculate_delta(*range.end(), *range.start());
        Self {
            range,
            value_delta: delta,
            duration,
        }
    }
}

impl<V, T> Tween<V, T> for CubicOut<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    fn update(&mut self, new_time: T) -> V {
        let percent_time = 1.0 - T::percent(self.duration, new_time);
        let new_value = self
            .value_delta
            .scale(percent_time * percent_time * percent_time);

        new_value.add(*self.range.start())
    }

    fn range(&self) -> &RangeInclusive<V> {
        &self.range
    }

    fn duration(&self) -> T {
        self.duration
    }

    fn delta(&self) -> V {
        self.value_delta
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CubicInOut<TValue = f32, TTime = f32> {
    range: RangeInclusive<TValue>,
    value_delta: TValue,
    duration: TTime,
}

impl<TValue, TTime> CubicInOut<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    pub fn new(range: RangeInclusive<TValue>, duration: TTime) -> Self {
        let delta = TValue::calculate_delta(*range.end(), *range.start());
        Self {
            range,
            value_delta: delta,
            duration,
        }
    }
}

impl<V, T> Tween<V, T> for CubicInOut<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    fn update(&mut self, new_time: T) -> V {
        // let t = t / (d / f(2.0));
        // if t < f(1.0) {
        //     c / f(2.0) * (t * t * t) + b
        // }
        // else {
        //     let t = t - f(2.0);
        //     c / f(2.0) * (t * t * t + f(2.0)) + b
        // }

        let percent_time = T::percent(self.duration, new_time);
        if percent_time < 0.5 {
            self.value_delta
                .scale(percent_time * percent_time * percent_time);
        } else {
            let percent_time = 1.0 - percent_time;
            self.value_delta
        }
        let new_value = self
            .value_delta
            .scale(percent_time * percent_time * percent_time);

        new_value.add(*self.range.start())
    }

    fn range(&self) -> &RangeInclusive<V> {
        &self.range
    }

    fn duration(&self) -> T {
        self.duration
    }

    fn delta(&self) -> V {
        self.value_delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_over_frames() {
        let mut value = 0;
        let mut tweener = CubicIn::new(value..=100, 10);

        for val in 1..=10 {
            value = tweener.update(val);
            assert_eq!(value, val * 10);
        }
    }
}
