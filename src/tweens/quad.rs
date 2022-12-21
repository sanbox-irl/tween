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

test_tween!(Quad);
