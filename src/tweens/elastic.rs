use core::f32::consts::TAU;

const SIGMA: f32 = 0.075;
const SIGMA_IN_OUT: f32 = 0.1125;
const THREE_DOT_THREE_REPEATING: f32 = 10.0 / 3.0;
const FORTY_FIVE: f32 = 2.222222;

declare_tween!(
    /// An elastic tween in. Go [here](https://easings.net/#easeInElastic) for a visual demonstration.
    pub struct ElasticIn;

    /// Creates a new [ElasticIn] Tweener.
    pub fn elastic_in;

    /// Creates a new [ElasticIn] Tweener at the given time.
    pub fn elastic_in_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, mut percent: f32) -> Value {
        if percent == 0.0 {
            return value_delta.scale(0.0);
        }

        if percent == 1.0 {
            return value_delta;
        }

        percent -= 1.0;

        #[cfg(feature = "libm")]
        let scalar = libm::powf(2.0, percent * 10.0);

        #[cfg(feature = "std")]
        let scalar = 2f32.powf(percent * 10.0);

        let post_fix = value_delta.scale(scalar);
        let temp = (percent - SIGMA) * TAU * THREE_DOT_THREE_REPEATING;

        #[cfg(feature = "libm")]
        let scalar = -libm::sinf(temp);

        #[cfg(feature = "std")]
        let scalar = -temp.sin();

        post_fix.scale(scalar)
    }
);

declare_tween!(
    /// An elastic tween out. Go [here](https://easings.net/#easeOutElastic) for a visual demonstration.
    pub struct ElasticOut;

    /// Creates a new [ElasticOut] Tweener.
    pub fn elastic_out;

    /// Creates a new [ElasticOut] Tweener at the given time.
    pub fn elastic_out_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value,  percent: f32) -> Value {
        if percent == 0.0 {
            return value_delta.scale(0.0);
        }

        if percent == 1.0 {
            return value_delta;
        }

        let temp = (percent - SIGMA) * TAU * THREE_DOT_THREE_REPEATING;

        #[cfg(feature = "libm")]
        let scalar = libm::powf(2.0, -10.0 * percent) * libm::sinf(temp);

        #[cfg(feature = "std")]
        let scalar = 2f32.powf(-10.0 * percent) * temp.sin();

        value_delta.scale(scalar) + value_delta
    }
);

declare_tween!(
    /// An elastic tween in and out. Go [here](https://easings.net/#easeInOutElastic) for a visual demonstration.
    pub struct ElasticInOut;

    /// Creates a new [ElasticInOut] Tweener.
    pub fn elastic_in_out;

    /// Creates a new [ElasticInOut] Tweener at the given time.
    pub fn elastic_in_out_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, mut percent: f32) -> Value {
        if percent == 0.0 {
            return value_delta.scale(0.0);
        }

        if percent == 1.0 {
            return value_delta;
        }

        percent = (percent * 2.0) - 1.0;

        if percent < 0.0 {
            #[cfg(feature = "libm")]
            let scalar = libm::powf(2.0, percent * 10.0);

            #[cfg(feature = "std")]
            let scalar = 2f32.powf(percent * 10.0);

            let post_fix = value_delta.scale(scalar);
            let temp = (percent - SIGMA_IN_OUT) * TAU * FORTY_FIVE;

            #[cfg(feature = "libm")]
            let temp_sin = libm::sinf(temp);

            #[cfg(feature = "std")]
            let temp_sin = temp.sin();

            post_fix.scale(-0.5 * temp_sin)
        } else {
            #[cfg(feature = "libm")]
            let scalar = libm::powf(2.0, percent * -10.0);

            #[cfg(feature = "std")]
            let scalar = 2f32.powf(-10.0 * percent);

            let post_fix = value_delta.scale(scalar);
            let temp = (percent - SIGMA_IN_OUT) * TAU * FORTY_FIVE;

            #[cfg(feature = "libm")]
            let temp_sin = libm::sinf(temp);

            #[cfg(feature = "std")]
            let temp_sin = temp.sin();

            post_fix.scale(temp_sin * 0.5) + value_delta
        }

    }
);

test_tween!(Elastic);
