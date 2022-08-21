use crate::{Tween, TweenTime, TweenValue};

declare_tween!(
    /// An quadratic tween in. Go [here](https://easings.net/#easeInQuad) for a visual demonstration.
    pub struct QuadIn;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self.value_delta.scale(percent_time * percent_time);

        new_value.add(self.initial_value)
    }
);

declare_tween!(
    /// An quadratic tween out. Go [here](https://easings.net/#easeOutQuad) for a visual demonstration.
    pub struct QuadOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self.value_delta.scale(-percent_time).scale(percent_time - 2.0);

        new_value.add(self.initial_value)
    }
);

declare_in_out_tween!(
    /// An quadratic tween in and out. Go [here](https://easings.net/#easeInOutQuad) for a visual demonstration.
    pub struct QuadInOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time) * 2.0;

        let scalar = if percent_time < 1.0 {
            percent_time * percent_time
        } else {
            let p = percent_time - 1.0;

            (p * (p - 2.0) - 1.0) * -1.0
        };
        let new_value = self.half_delta.scale(scalar);

        new_value.add(self.initial_value)
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use easer::functions::{Easing, Quad};

    #[test]
    fn tween_in() {
        let mut tweener = QuadIn::new(0.0, 100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.run(time);
            let o = Quad::ease_in(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_out() {
        let mut tweener = QuadOut::new(0.0, 100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.run(time);
            let o = Quad::ease_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_in_out() {
        let mut tweener = QuadInOut::new(0.0, 100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let our_value = tweener.run(time);
            let easer = Quad::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(our_value, easer);
        }
    }
}
