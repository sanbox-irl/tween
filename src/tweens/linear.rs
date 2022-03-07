use crate::{Tween, TweenTime, TweenValue};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl<V, T> Tween for Linear<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    type Value = V;
    type Time = T;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;

    #[test]
    fn linear_over_frames() {
        let mut value = 0.0;
        let mut tweener = Linear::new(value..=100.0, 10);

        for val in 1..=10 {
            value = tweener.update(val);
            assert_ulps_eq!(value, val as f32 * 10.0);
        }
    }
}
