use core::f64::consts::PI;

declare_tween!(
    /// An sine based tween in. Go [here](https://easings.net/#easeInSine) for a visual demonstration.
    pub struct SineIn;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);

        #[cfg(feature = "libm")]
        let time = libm::cos(percent_time * PI / 2.0);

        #[cfg(feature = "std")]
        let time = (percent_time * PI / 2.0).cos();

        let new_value = self.value_delta.scale(-time);

        new_value + self.initial_value + self.value_delta
    }
);

declare_tween!(
    /// An sine based tween out. Go [here](https://easings.net/#easeOutSine) for a visual demonstration.
    pub struct SineOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);

        #[cfg(feature = "libm")]
        let time = libm::sin(percent_time * PI / 2.0);

        #[cfg(feature = "std")]
        let time = (percent_time * PI / 2.0).sin();

        let new_value = self.value_delta.scale(time);

        new_value + self.initial_value
    }
);

declare_tween!(
    /// An sine based tween in out. Go [here](https://easings.net/#easeInOutSine) for a visual demonstration.
    pub struct SineInOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        #[cfg(feature = "libm")]
        let time = libm::cos(percent_time * PI) - 1.0;

        #[cfg(feature = "std")]
        let time = (percent_time * PI).cos() - 1.0;
        let new_value = self.value_delta.scale(-time / 2.0);

        new_value + self.initial_value
    }
);

test_tween!(Sine);
