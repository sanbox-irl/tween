use std::ops::RangeInclusive;

use crate::{Tween, TweenTime, TweenValue};

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

#[cfg(test)]
mod tests {
    use crate::tweener::FixedDeltaTweener;

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
