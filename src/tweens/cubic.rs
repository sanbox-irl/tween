declare_tween!(
    /// A cubic tween in. Go [here](https://easings.net/#easeInCubic) for a visual demonstration.
    pub struct CubicIn;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self.value_delta.scale(percent_time * percent_time * percent_time);

        new_value.add(self.initial_value)
    }
);

declare_tween!(
    /// A cubic tween out. Go [here](https://easings.net/#easeOutCubic) for a visual demonstration.
    pub struct CubicOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time) - 1.0;
        let new_value = self.value_delta.scale(percent_time * percent_time * percent_time + 1.0);

        new_value.add(self.initial_value)
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

        new_value.add(self.initial_value)
    }
);

test_tween!(Cubic);
