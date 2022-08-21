use crate::{Tween, TweenTime, TweenValue};

declare_tween!(
    /// An quintic tween in. Go [here](https://easings.net/#easeInQuint) for a visual demonstration.
    pub struct QuintIn;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self
            .value_delta
            .scale(percent_time * percent_time * percent_time * percent_time * percent_time);

        new_value.add(self.initial_value)
    }
);

declare_tween!(
    /// An quintic tween out. Go [here](https://easings.net/#easeOutQuint) for a visual demonstration.
    pub struct QuintOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time) - 1.0;
        let new_value = self
            .value_delta
            .scale(percent_time * percent_time * percent_time * percent_time * percent_time + 1.0);

        new_value.add(self.initial_value)
    }
);

declare_in_out_tween!(
    /// An quintic tween in out. Go [here](https://easings.net/#easeInOutQuint) for a visual demonstration.
    pub struct QuintInOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time) * 2.0;

        let scalar = if percent_time < 1.0 {
            percent_time * percent_time * percent_time * percent_time * percent_time
        } else {
            let p = percent_time - 2.0;
            p * p * p * p * p + 2.0
        };
        let new_value = self.half_delta.scale(scalar);

        new_value.add(self.initial_value)
    }
);

test_tween!(Quint);
