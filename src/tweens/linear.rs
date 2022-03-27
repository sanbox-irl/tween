use crate::{Tween, TweenTime, TweenValue};
use std::ops::RangeInclusive;

declare_tween!(
    /// A Linear tween is a simple lerp from one value to another.
    pub struct Linear;

    fn update(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self.value_delta.scale(percent_time);

        new_value.add(*self.range.start())
    }
);

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
