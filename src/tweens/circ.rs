declare_tween!(
    /// A circular tween in. Go [here](https://easings.net/#easeInCirc) for a visual demonstration.
    pub struct CircIn;

    /// Creates a new [CircIn] Tweener.
    pub fn circ_in;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f32) -> Value {
        #[cfg(feature = "libm")]
        let scalar = 1.0 - libm::sqrt(1.0 - percent * percent);

        #[cfg(feature = "std")]
        let scalar = 1.0 - (1.0 - percent * percent).sqrt();

        value_delta.scale(scalar)
    }
);

declare_tween!(
    /// A circular tween out. Go [here](https://easings.net/#easeOutCirc) for a visual demonstration.
    pub struct CircOut;

    /// Creates a new [CircOut] Tweener.
    pub fn circ_out;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f32) -> Value {
        let t = percent - 1.0;

        #[cfg(feature = "libm")]
        let scalar = libm::sqrt(1.0 - t * t);

        #[cfg(feature = "std")]
        let scalar = (1.0 - t * t).sqrt();

        value_delta.scale(scalar)
    }
);

declare_tween!(
    /// A circular tween in and out. Go [here](https://easings.net/#easeInOutCirc) for a visual demonstration.
    pub struct CircInOut;

    /// Creates a new [CircInOut] Tweener.
    pub fn circ_in_out;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, mut percent: f32) -> Value {
        percent *= 2.0;

        let scalar = if percent < 1.0 {
            #[cfg(feature = "libm")]
            let o = 1.0 - libm::sqrt(1.0 - percent * percent);

            #[cfg(feature = "std")]
            let o = 1.0 - (1.0 - percent * percent).sqrt();

            o
        } else {
            let percent = percent - 2.0;

            #[cfg(feature = "libm")]
            let o = libm::sqrt(1.0 - percent * percent) + 1.0;

            #[cfg(feature = "std")]
            let o = (1.0 - percent * percent).sqrt() + 1.0;

            o
        };

        value_delta.scale(scalar / 2.0)
    }
);

test_tween!(Circ);
