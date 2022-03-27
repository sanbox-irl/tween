use crate::{Tween, TweenTime, TweenValue};
use std::ops::RangeInclusive;

declare_tween!(
    pub struct CircIn;

    fn update(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);
        let scalar = 1.0 - (1.0 - t * t).sqrt();
        let new_value = self.value_delta.scale(scalar);

        new_value.add(*self.range.start())
    }
);

declare_tween!(
    pub struct CircOut;

    fn update(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) - 1.0;

        let scalar = (1.0 - t * t).sqrt();
        let new_value = self.value_delta.scale(scalar);

        new_value.add(*self.range.start())
    }
);

declare_in_out_tween!(
    pub struct CircInOut;

    fn update(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) * 2.0;

        let scalar = if t < 1.0 {
            1.0 - (1.0 - t * t).sqrt()
        } else {
            let t = t - 2.0;

            (1.0 - t * t).sqrt() + 1.0
        };
        let new_value = self.half_delta.scale(scalar);

        new_value.add(*self.range.start())
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use easer::functions::{Circ, Easing};

    #[test]
    fn tween_in() {
        let mut tweener = CircIn::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;
            println!("t = {}", time);

            let v = tweener.update(time);
            let o = Circ::ease_in(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_out() {
        let mut tweener = CircOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let v = tweener.update(time);
            let o = Circ::ease_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_in_out() {
        let mut tweener = CircInOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let our_value = tweener.update(time);
            let easer = Circ::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(our_value, easer);
        }
    }
}
