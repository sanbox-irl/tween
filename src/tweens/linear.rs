use crate::{Tween, TweenTime, TweenValue};
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Linear<TValue, TTime> {
    range: RangeInclusive<TValue>,
    value_delta: TValue,
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
            value_delta: delta,
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
        let new_value = self.value_delta.scale(percent_time);

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
        let mut tweener = Linear::new(value..=100, 10);

        for val in 1..=10 {
            value = tweener.update(val);
            assert_eq!(value, val * 10);
        }
    }
}
