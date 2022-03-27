use crate::{Tween, TweenTime, TweenValue};
use std::{f64::consts::PI, ops::RangeInclusive};

/// An elastic tween in. Go [here](https://easings.net/#easeInElastic) for a visual demonstration.
#[derive(Debug, PartialEq, Clone)]
pub struct ElasticIn<TValue, TTime> {
    range: RangeInclusive<TValue>,
    value_delta: TValue,
    duration: TTime,
    three_tenths: f64,
    s: f64,
}

impl<TValue, TTime> ElasticIn<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    /// Creates a new tween out of a range with a duration.
    pub fn new(range: RangeInclusive<TValue>, duration: TTime) -> Self {
        let delta = TValue::calculate_delta(*range.end(), *range.start());
        let three_tenths = duration.as_f64() * 0.3;
        Self {
            range,
            value_delta: delta,
            duration,
            three_tenths,
            s: three_tenths * 0.25,
        }
    }
}

impl<V, T> Tween for ElasticIn<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    type Value = V;
    type Time = T;

    fn update(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);

        if t == 0.0 {
            return *self.range.start();
        }

        if t == 1.0 {
            return *self.range.end();
        }

        let t: f64 = t - 1.0;

        let post_fix = self.value_delta.scale(2f64.powf(t * 10.0));
        let temp = (self.duration.as_f64() * t - self.s) * (2.0 * PI) / self.three_tenths;

        post_fix.scale(-temp.sin()).add(*self.range.start())
    }

    fn range(&self) -> &RangeInclusive<V> {
        &self.range
    }

    fn duration(&self) -> T {
        self.duration
    }
}

/// An elastic tween out. Go [here](https://easings.net/#easeOutElastic) for a visual demonstration.
#[derive(Debug, PartialEq, Clone)]
pub struct ElasticOut<TValue, TTime> {
    range: RangeInclusive<TValue>,
    value_delta: TValue,
    duration: TTime,
    three_tenths: f64,
    s: f64,
}

impl<TValue, TTime> ElasticOut<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    /// Creates a new tween out of a range with a duration.
    pub fn new(range: RangeInclusive<TValue>, duration: TTime) -> Self {
        let delta = TValue::calculate_delta(*range.end(), *range.start());
        let three_tenths = duration.as_f64() * 0.3;
        Self {
            range,
            value_delta: delta,
            duration,
            three_tenths,
            s: three_tenths * 0.25,
        }
    }
}

impl<V, T> Tween for ElasticOut<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    type Value = V;
    type Time = T;

    fn update(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);

        if t == 0.0 {
            return *self.range.start();
        }

        if t == 1.0 {
            return *self.range.end();
        }

        let temp = (t * self.duration.as_f64() - self.s) * (2.0 * PI) / self.three_tenths;

        self.value_delta
            .scale(2f64.powf(-10.0 * t) * temp.sin())
            .add(self.value_delta)
            .add(*self.range.start())
    }

    fn range(&self) -> &RangeInclusive<V> {
        &self.range
    }

    fn duration(&self) -> T {
        self.duration
    }
}

/// An elastic tween in and out. Go [here](https://easings.net/#easeInOutElastic) for a visual demonstration.
#[derive(Debug, PartialEq, Clone)]
pub struct ElasticInOut<TValue, TTime> {
    range: RangeInclusive<TValue>,
    value_delta: TValue,
    duration: TTime,
    p: f64,
    s: f64,
}

impl<TValue, TTime> ElasticInOut<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    /// Creates a new tween out of a range with a duration.
    pub fn new(range: RangeInclusive<TValue>, duration: TTime) -> Self {
        let delta = TValue::calculate_delta(*range.end(), *range.start());
        let p = duration.as_f64() * 0.45;
        Self {
            range,
            value_delta: delta,
            duration,
            p,
            s: p * 0.25,
        }
    }
}

impl<V, T> Tween for ElasticInOut<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    type Value = V;
    type Time = T;

    fn update(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) * 2.0;

        if t == 0.0 {
            return *self.range.start();
        }

        if t == 2.0 {
            return *self.range.end();
        }

        let t = t - 1.0;
        if t < 0.0 {
            let post_fix = self.value_delta.scale(2f64.powf(10.0 * t));
            let temp = (self.duration.as_f64() * t - self.s) * (2.0 * PI) / self.p;
            post_fix.scale(-0.5 * temp.sin()).add(*self.range.start())
        } else {
            let post_fix = self.value_delta.scale(2f64.powf(-10.0 * t));
            let temp = (self.duration.as_f64() * t - self.s) * (2.0 * PI) / self.p;
            post_fix.scale(temp.sin() * 0.5).add(*self.range.end())
        }
    }

    fn range(&self) -> &RangeInclusive<V> {
        &self.range
    }

    fn duration(&self) -> T {
        self.duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use easer::functions::{Easing, Elastic};

    #[test]
    fn tween_in() {
        let mut tweener = ElasticIn::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;
            println!("t = {}", time);

            let v = tweener.update(time);
            let o = Elastic::ease_in(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_out() {
        let mut tweener = ElasticOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let v = tweener.update(time);
            let o = Elastic::ease_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_in_out() {
        let mut tweener = ElasticInOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let our_value = tweener.update(time);
            let easer = Elastic::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(our_value, easer);
        }
    }
}
