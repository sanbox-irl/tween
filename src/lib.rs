use std::{marker::PhantomData, ops::RangeInclusive};

pub trait Tween<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    fn update(&mut self, new_time: TTime) -> TValue;
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
}
impl TweenTime for f32 {
    const ZERO: Self = 0.0;
    fn percent(duration: Self, current_time: Self) -> f32 {
        current_time / duration
    }

    fn add(self, other: Self) -> Self {
        self + other
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
}
impl TweenTime for i32 {
    const ZERO: Self = 0;

    fn percent(duration: Self, current_time: Self) -> f32 {
        current_time as f32 / duration as f32
    }

    fn add(self, other: Self) -> Self {
        self + other
    }
}

pub struct DeltaTweener<Tween, TValue, TTime> {
    tween: Tween,
    last_time: TTime,
    delta: TTime,
    _value: PhantomData<fn(&mut Self, TTime) -> TValue>,
}

impl<T, TValue, TTime> DeltaTweener<T, TValue, TTime>
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
            _value: PhantomData,
        }
    }
}

impl<T, TValue, TTime> Iterator for DeltaTweener<T, TValue, TTime>
where
    T: Tween<TValue, TTime>,
    TValue: TweenValue,
    TTime: TweenTime,
{
    type Item = TValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.last_time = self.last_time.add(self.delta);

        Some(self.tween.update(self.last_time))
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
        let mut value = 0;
        let mut tweener = DeltaTweener::new(Linear::new(value..=100, 10), 1);

        value = tweener.next().unwrap();
    }
}
