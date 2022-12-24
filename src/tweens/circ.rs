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

        new_value + self.initial_value
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

        new_value + self.initial_value
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

        new_value + self.initial_value
    }
);

test_tween!(Circ);
