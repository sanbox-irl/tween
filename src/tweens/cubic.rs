use crate::{Tween, TweenTime, TweenValue};

declare_tween!(
    /// A cubic tween in. Go [here](https://easings.net/#easeInCubic) for a visual demonstration.
    pub struct CubicIn;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self.value_delta.scale(percent_time * percent_time * percent_time);

        new_value.add(self.initial_value())
    }
);

declare_tween!(
    /// A cubic tween out. Go [here](https://easings.net/#easeOutCubic) for a visual demonstration.
    pub struct CubicOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time) - 1.0;
        let new_value = self.value_delta.scale(percent_time * percent_time * percent_time + 1.0);

        new_value.add(self.initial_value())
    }
);

declare_in_out_tween!(
    /// A cubic tween in and out. Go [here](https://easings.net/#easeInOutCubic) for a visual demonstration.
    pub struct CubicInOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time) * 2.0;

        let scalar = if percent_time < 1.0 {
            percent_time * percent_time * percent_time
        } else {
            let p = percent_time - 2.0;
            p * p * p + 2.0
        };
        let new_value = self.half_delta.scale(scalar);

        new_value.add(self.initial_value())
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use easer::functions::{Cubic as EaseCubic, Easing};

    #[test]
    fn cubic_in() {
        let mut tweener = CubicIn::new(0.0, 100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.run(time);
            let o = EaseCubic::ease_in(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn cubic_out() {
        let mut tweener = CubicOut::new(0.0, 100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.run(time);
            let o = EaseCubic::ease_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn cubic_in_out() {
        let mut tweener = CubicInOut::new(0.0, 100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let our_value = tweener.run(time);
            let easer = EaseCubic::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(our_value, easer);
        }
    }
}
