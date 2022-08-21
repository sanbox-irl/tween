use crate::{Tween, TweenTime, TweenValue};

/// This appears to be a magic constant for Back eases. I have no idea
/// where it's from, but we'll use it
const BACK_CONST: f64 = 1.70158;

/// This is another magic constant for the back in out tween.
/// Where it comes from, I do not know!
const BACK_IN_OUT_CONST: f64 = BACK_CONST * 1.525;

declare_tween!(
    /// An tween that goes out and then back in a bit. Go [here](https://easings.net/#easeInBack) for a visual demonstration.
    pub struct BackIn;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);
        let scalar = t * t * ((BACK_CONST + 1.0) * t - BACK_CONST);

        let new_value = self.value_delta.scale(scalar);

        new_value.add(self.initial_value)
    }
);

declare_tween!(
    /// An tween that goes in and then back out a bit. Go [here](https://easings.net/#easeOutBack) for a visual demonstration.
    pub struct BackOut;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) - 1.0;
        let scalar = t * t * ((BACK_CONST + 1.0) * t + BACK_CONST) + 1.0;

        let new_value = self.value_delta.scale(scalar);

        new_value.add(self.initial_value)
    }
);

declare_in_out_tween!(
    /// An tween that goes out, in, and then back in and out a bit. Go [here](https://easings.net/#easeInOutBack) for a visual demonstration.
    pub struct BackInOut;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) * 2.0;

        let scalar = if t < 1.0 {
            t * t * ((BACK_IN_OUT_CONST + 1.0) * t - BACK_IN_OUT_CONST)
        } else {
            let t = t - 2.0;

            t * t * ((BACK_IN_OUT_CONST + 1.0) * t + BACK_IN_OUT_CONST) + 2.0
        };
        let new_value = self.half_delta.scale(scalar);

        new_value.add(self.initial_value)
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use easer::functions::{Back, Easing};

    #[test]
    fn tween_in() {
        let mut tweener = BackIn::new(0.0, 100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let v = tweener.run(time);
            let o = Back::ease_in(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_out() {
        let mut tweener = BackOut::new(0.0, 100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let v = tweener.run(time);
            let o = Back::ease_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_in_out() {
        let mut tweener = BackInOut::new(0.0, 100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let our_value = tweener.run(time);
            let easer = Back::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(our_value, easer);
        }
    }
}
