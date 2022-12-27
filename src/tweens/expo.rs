declare_tween!(
    /// An exponenential tween in. See [here](https://easings.net/#easeInExpo)
    pub struct ExpoIn;

    /// Creates a new [ExpoIn] Tweener.
    pub fn expo_in;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        // weird edge in this tween?
        if percent == 0.0 {
            return Value::ZERO;
        }

        let percent = 10.0 * (percent - 1.0);

        #[cfg(feature = "libm")]
        let scalar = libm::pow(2.0, percent);

        #[cfg(feature = "std")]
        let scalar = 2.0f64.powf(percent);

        value_delta.scale(scalar)
    }
);

declare_tween!(
    /// An exponenential tween out. See [here](https://easings.net/#easeOutExpo)
    pub struct ExpoOut;

    /// Creates a new [ExpoOut] Tweener.
    pub fn expo_out;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        if percent == 1.0 {
            value_delta
        } else {
            #[cfg(feature = "libm")]
            let powf = libm::pow(2.0, -10.0 * percent);

            #[cfg(feature = "std")]
            let powf = 2.0f64.powf(-10.0 * percent);

            value_delta.scale(1.0 - powf)
        }
    }
);

declare_tween!(
    /// An exponenential tween in and out. See [here](https://easings.net/#easeInOutExpo)
    pub struct ExpoInOut;

    /// Creates a new [ExpoInOut] Tweener.
    pub fn expo_in_out;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        if percent == 0.0 {
            return Value::ZERO;
        }

        if percent == 1.0 {
            return value_delta;
        }

        percent *= 2.0;

        let powf = if percent < 1.0 {
            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, 10.0 * (percent - 1.0));

            #[cfg(feature = "std")]
            let scalar = 2.0f64.powf(10.0 * (percent - 1.0));

            scalar / 2.0
        } else {
            let percent = percent - 1.0;

            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, -10.0 * percent);

            #[cfg(feature = "std")]
            let scalar = 2.0f64.powf(-10.0 * percent);

            (2.0 - scalar) / 2.0
        };

        value_delta.scale(powf)
    }
);

test_tween!(Expo);
