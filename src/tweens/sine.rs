use core::f64::consts::PI;

declare_tween!(
    /// An sine based tween in. Go [here](https://easings.net/#easeInSine) for a visual demonstration.
    pub struct SineIn;

    /// Creates a new [SineIn] Tweener.
    pub fn sine_in;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f64) -> Value {
        #[cfg(feature = "libm")]
        let time = libm::cos(percent * PI / 2.0);

        #[cfg(feature = "std")]
        let time = (percent * PI / 2.0).cos();

        value_delta.scale(-time) + value_delta
    }
);

declare_tween!(
    /// An sine based tween out. Go [here](https://easings.net/#easeOutSine) for a visual demonstration.
    pub struct SineOut;

    /// Creates a new [SineOut] Tweener.
    pub fn sine_out;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f64) -> Value {
        #[cfg(feature = "libm")]
        let time = libm::sin(percent * PI / 2.0);

        #[cfg(feature = "std")]
        let time = (percent * PI / 2.0).sin();

        value_delta.scale(time)
    }
);

declare_tween!(
    /// An sine based tween in out. Go [here](https://easings.net/#easeInOutSine) for a visual demonstration.
    pub struct SineInOut;

    /// Creates a new [SineOut] Tweener.
    pub fn sine_in_out;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f64) -> Value
    {
        #[cfg(feature = "libm")]
        let time = libm::cos(percent * PI) - 1.0;

        #[cfg(feature = "std")]
        let time = (percent * PI).cos() - 1.0;
        value_delta.scale(-time / 2.0)
    }
);

test_tween!(Sine);
