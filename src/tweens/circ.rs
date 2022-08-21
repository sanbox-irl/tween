use crate::{Tween, TweenTime, TweenValue};

declare_tween!(
    /// A circular tween in. Go [here](https://easings.net/#easeInCirc) for a visual demonstration.
    pub struct CircIn;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);
        #[cfg(feature = "libm")]
        let scalar = 1.0 - libm::sqrt(1.0 - t * t);

        #[cfg(feature = "std")]
        let scalar = 1.0 - (1.0 - t * t).sqrt();
        let new_value = self.value_delta.scale(scalar);

        new_value.add(self.initial_value)
    }
);

declare_tween!(
    /// A circular tween out. Go [here](https://easings.net/#easeOutCirc) for a visual demonstration.
    pub struct CircOut;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) - 1.0;

        #[cfg(feature = "libm")]
        let scalar = libm::sqrt(1.0 - t * t);

        #[cfg(feature = "std")]
        let scalar = (1.0 - t * t).sqrt();

        let new_value = self.value_delta.scale(scalar);

        new_value.add(self.initial_value)
    }
);

declare_in_out_tween!(
    /// A circular tween in and out. Go [here](https://easings.net/#easeInOutCirc) for a visual demonstration.
    pub struct CircInOut;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) * 2.0;

        let scalar = if t < 1.0 {
            #[cfg(feature = "libm")]
            let o = 1.0 - libm::sqrt(1.0 - t * t);

            #[cfg(feature = "std")]
            let o = 1.0 - (1.0 - t * t).sqrt();

            o
        } else {
            let t = t - 2.0;

            #[cfg(feature = "libm")]
            let o = libm::sqrt(1.0 - t * t) + 1.0;

            #[cfg(feature = "std")]
            let o = (1.0 - t * t).sqrt() + 1.0;

            o
        };
        let new_value = self.half_delta.scale(scalar);

        new_value.add(self.initial_value)
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use easer::functions::{Circ, Easing};

    #[test]
    fn tween_in() {
        let mut tweener = CircIn::new(0.0, 100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let v = tweener.run(time);
            let o = Circ::ease_in(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_out() {
        let mut tweener = CircOut::new(0.0, 100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let v = tweener.run(time);
            let o = Circ::ease_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_in_out() {
        let mut tweener = CircInOut::new(0.0, 100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let our_value = tweener.run(time);
            let easer = Circ::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(our_value, easer);
        }
    }
}
