declare_tween!(
    /// An exponenential tween in. See [here](https://easings.net/#easeInExpo)
    pub struct ExpoIn;

    /// Creates a new [ExpoIn] Tweener.
    pub fn expo_in;

    /// Creates a new [ExpoIn] Tweener at the given time.
    pub fn expo_in_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f32) -> Value {
        // weird edge in this tween?
        if percent == 0.0 {
            return value_delta.scale(0.0);
        }

        let percent = 10.0 * (percent - 1.0);

        #[cfg(feature = "libm")]
        let scalar = libm::powf(2.0, percent);

        #[cfg(feature = "std")]
        let scalar = 2.0f32.powf(percent);

        value_delta.scale(scalar)
    }
);

declare_tween!(
    /// An exponenential tween out. See [here](https://easings.net/#easeOutExpo)
    pub struct ExpoOut;

    /// Creates a new [ExpoOut] Tweener.
    pub fn expo_out;

    /// Creates a new [ExpoOut] Tweener at the given time.
    pub fn expo_out_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f32) -> Value {
        if percent == 1.0 {
            value_delta
        } else {
            #[cfg(feature = "libm")]
            let powf = libm::powf(2.0, -10.0 * percent);

            #[cfg(feature = "std")]
            let powf = 2.0f32.powf(-10.0 * percent);

            value_delta.scale(1.0 - powf)
        }
    }
);

declare_tween!(
    /// An exponenential tween in and out. See [here](https://easings.net/#easeInOutExpo)
    pub struct ExpoInOut;

    /// Creates a new [ExpoInOut] Tweener.
    pub fn expo_in_out;

    /// Creates a new [ExpoInOut] Tweener at the given time.
    pub fn expo_in_out_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, mut percent: f32) -> Value {
        if percent == 0.0 {
            return value_delta.scale(0.0);
        }

        if percent == 1.0 {
            return value_delta;
        }

        percent *= 2.0;

        let powf = if percent < 1.0 {
            #[cfg(feature = "libm")]
            let scalar = libm::powf(2.0, 10.0 * (percent - 1.0));

            #[cfg(feature = "std")]
            let scalar = 2.0f32.powf(10.0 * (percent - 1.0));

            scalar / 2.0
        } else {
            let percent = percent - 1.0;

            #[cfg(feature = "libm")]
            let scalar = libm::powf(2.0, -10.0 * percent);

            #[cfg(feature = "std")]
            let scalar = 2.0f32.powf(-10.0 * percent);

            (2.0 - scalar) / 2.0
        };

        value_delta.scale(powf)
    }
);

test_tween!(Expo);
