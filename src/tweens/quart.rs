use crate::{Tween, TweenTime, TweenValue};
use std::ops::RangeInclusive;

declare_tween!(
    /// An quartic tween in. Go [here](https://easings.net/#easeInQuart) for a visual demonstration.
    pub struct QuartIn;

    fn update(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self
            .value_delta
            .scale(percent_time * percent_time * percent_time * percent_time);

        new_value.add(*self.range.start())
    }
);

declare_tween!(
    /// An quartic tween out. Go [here](https://easings.net/#easeOutQuart) for a visual demonstration.
    pub struct QuartOut;

    fn update(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time) - 1.0;
        let new_value = self
            .value_delta
            .scale(-(percent_time * percent_time * percent_time * percent_time - 1.0));

        new_value.add(*self.range.start())
    }
);

declare_in_out_tween!(
    /// An quartic tween in and out. Go [here](https://easings.net/#easeInOutQuart) for a visual demonstration.
    pub struct QuartInOut;

    fn update(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time) * 2.0;

        let scalar = if percent_time < 1.0 {
            percent_time * percent_time * percent_time * percent_time
        } else {
            let p = percent_time - 2.0;
            -(p * p * p * p - 2.0)
        };
        let new_value = self.half_delta.scale(scalar);

        new_value.add(*self.range.start())
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use easer::functions::{Easing, Quart};

    #[test]
    fn tween_in() {
        let mut tweener = QuartIn::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.update(time);
            let o = Quart::ease_in(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_out() {
        let mut tweener = QuartOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.update(time);
            let o = Quart::ease_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_in_out() {
        let mut tweener = QuartInOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let our_value = tweener.update(time);
            let easer = Quart::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(our_value, easer);
        }
    }
}
