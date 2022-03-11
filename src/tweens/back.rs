use crate::{Tween, TweenTime, TweenValue};
use std::ops::RangeInclusive;

/// This appears to be a magic constant for Back eases. I have no idea
/// where it's from, but we'll use it 
const BACK_CONST: f64 = 1.70158;

declare_tween!(
    BackIn,
    fn update(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);
        let scalar = 1.0 - (1.0 - t * t).sqrt();
        let new_value = self.value_delta.scale(scalar);

        new_value.add(*self.range.start())
    }
);

declare_tween!(
    BackOut,
    fn update(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) - 1.0;

        let scalar = (1.0 - t * t).sqrt();
        let new_value = self.value_delta.scale(scalar);

        new_value.add(*self.range.start())
    }
);

declare_in_out_tween!(
    BackInOut,
    fn update(&mut self, new_time: TTime) -> TValue {
        let t = TTime::percent(self.duration, new_time) * 2.0;

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
    use easer::functions::{Back, Easing};

    #[test]
    fn tween_in() {
        let mut tweener = BackIn::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;
            println!("t = {}", time);

            let v = tweener.update(time);
            let o = Back::ease_in(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_out() {
        let mut tweener = BackOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let v = tweener.update(time);
            let o = Back::ease_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_in_out() {
        let mut tweener = BackInOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let our_value = tweener.update(time);
            let easer = Back::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(our_value, easer);
        }
    }
}
