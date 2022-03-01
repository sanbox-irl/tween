use std::{marker::PhantomData, ops::RangeInclusive};

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

#[derive(Debug)]
pub struct Linear<TValue, TTime> {
    range: RangeInclusive<TValue>,
    delta: TValue,
    duration: TTime,
}

impl<TValue, TTime> Linear<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    pub fn new(range: RangeInclusive<TValue>, duration: TTime) -> Self {
        let delta = TValue::calculate_delta(*range.end(), *range.start());
        Self {
            range,
            delta,
            duration,
        }
    }
}

impl<V, T> Tween<V, T> for Linear<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    fn update(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self.delta.scale(percent_time);

        new_value.add(*self.range.start())
    }

    fn range(&self) -> &RangeInclusive<V> {
        &self.range
    }

    fn duration(&self) -> T {
        self.duration
    }

    fn delta(&self) -> V {
        self.delta
    }
}

pub trait TweenValue: Copy {
    fn calculate_delta(destination: Self, start: Self) -> Self;
    fn add(self, other: Self) -> Self;
    fn scale(self, scale: f32) -> Self;
}

impl TweenValue for f32 {
    fn add(self, other: Self) -> Self {
        self + other
    }

    fn calculate_delta(destination: Self, start: Self) -> Self {
        destination - start
    }

    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}
impl TweenValue for i32 {
    fn add(self, other: Self) -> Self {
        self + other
    }

    fn calculate_delta(destination: Self, start: Self) -> Self {
        destination - start
    }

    fn scale(self, scale: f32) -> Self {
        (self as f32 * scale) as i32
    }
}

pub trait TweenTime: Copy {
    const ZERO: Self;
    fn percent(duration: Self, current_time: Self) -> f32;
    fn add(self, other: Self) -> Self;
    fn is_complete(self, duration: Self) -> bool;
}
impl TweenTime for f32 {
    const ZERO: Self = 0.0;
    fn percent(duration: Self, current_time: Self) -> f32 {
        current_time / duration
    }

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn is_complete(self, duration: Self) -> bool {
        self >= duration
    }
}
impl TweenTime for usize {
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
impl TweenTime for i32 {
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

pub struct FixedDeltaTweener<Tween, TValue, TTime> {
    tween: Tween,
    last_time: TTime,
    delta: TTime,
    fused: bool,
    _value: PhantomData<fn(&mut Self, TTime) -> TValue>,
}

impl<T, TValue, TTime> FixedDeltaTweener<T, TValue, TTime>
where
    T: Tween<TValue, TTime>,
    TValue: TweenValue,
    TTime: TweenTime,
{
    pub fn new(tween: T, delta: TTime) -> Self {
        Self {
            tween,
            last_time: TTime::ZERO,
            delta,
            fused: false,

            _value: PhantomData,
        }
    }
}

impl<T, TValue, TTime> Iterator for FixedDeltaTweener<T, TValue, TTime>
where
    T: Tween<TValue, TTime>,
    TValue: TweenValue,
    TTime: TweenTime,
{
    type Item = TValue;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.fused {
            self.last_time = self.last_time.add(self.delta);

            if self.last_time.is_complete(self.tween.duration()) {
                self.fused = true;
                Some(*self.tween.range().end())
            } else {
                Some(self.tween.update(self.last_time))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;

    #[test]
    fn linear() {
        let mut value = 0.0;
        let mut tweener = Linear::new(value..=100.0, 10.0);

        for val in 1..=10 {
            value = tweener.update(val as f32);
            assert_ulps_eq!(value, (val * 10) as f32);
        }
    }

    #[test]
    fn linear_over_frames() {
        let mut value = 0;
        let mut tweener = Linear::new(value..=100, 10);

        for val in 1..=10 {
            value = tweener.update(val);
            assert_eq!(value, val * 10);
        }
    }

    #[test]
    fn tweener() {
        let tweener = FixedDeltaTweener::new(Linear::new(0..=100, 10), 1);
        let values: Vec<_> = tweener.collect();

        assert_eq!(*values, [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
    }
}
