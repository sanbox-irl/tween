use crate::{Tween, TweenTime, TweenValue};
use std::{f64::consts::PI, ops::RangeInclusive};

declare_tween!(
    pub struct SineIn;

    fn update(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let time = (percent_time * PI / 2.0).cos();
        let new_value = self.value_delta.scale(-time);

        new_value.add(*self.range.start()).add(self.value_delta)
    }
);

declare_tween!(
    pub struct SineOut;

    fn update(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let time = (percent_time * PI / 2.0).sin();
        let new_value = self.value_delta.scale(time);

        new_value.add(*self.range.start())
    }
);

declare_tween!(
    pub struct SineInOut;

    fn update(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let time = (percent_time * PI).cos() - 1.0;
        let new_value = self.value_delta.scale(-time / 2.0);

        new_value.add(*self.range.start())
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use easer::functions::{Easing, Sine as EaseSine};

    #[test]
    fn tween_in() {
        let mut tweener = SineIn::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.update(time);
            let o = EaseSine::ease_in(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_out() {
        let mut tweener = SineOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.update(time);
            let o = EaseSine::ease_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_in_out() {
        let mut tweener = SineInOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let our_value = tweener.update(time);
            let easer = EaseSine::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(our_value, easer);
        }
    }
}
