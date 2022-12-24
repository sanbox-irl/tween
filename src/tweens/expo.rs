use core::marker::PhantomData;

use crate::{Tween2, TweenTime, TweenValue};

declare_tween!(
    /// An exponenential tween in. See [here](https://easings.net/#easeInExpo)
    pub struct ExpoIn;

    fn run(&mut self, new_time: T) -> V {
        if new_time == T::ZERO {
            self.initial_value
        } else {
            let percent_time = 10.0 * (T::percent(self.duration, new_time) - 1.0);
            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, percent_time);

            #[cfg(feature = "std")]
            let scalar = 2.0f64.powf(percent_time);

            let new_value = self.value_delta.scale(scalar);

            new_value + self.initial_value
        }
    }
);

declare_tween!(
    /// An exponenential tween out. See [here](https://easings.net/#easeOutExpo)
    pub struct ExpoOut;

    fn run(&mut self, new_time: T) -> V {
        if new_time == self.duration {
            self.final_value
        } else {
            #[cfg(feature = "libm")]
            let powf = libm::pow(2.0, -10.0 * T::percent(self.duration, new_time));

            #[cfg(feature = "std")]
            let powf = 2.0f64.powf(-10.0 * T::percent(self.duration, new_time));

            let new_value = self.value_delta.scale(1.0 - powf);

            new_value + self.initial_value
        }
    }
);

declare_tween!(
    /// An exponenential tween in and out. See [here](https://easings.net/#easeInOutExpo)
    pub struct ExpoInOut;

    fn run(&mut self, new_time: T) -> V {
        if new_time == T::ZERO {
            return self.initial_value;
        }

        if new_time == self.duration {
            return self.final_value;
        }

        let t = T::percent(self.duration, new_time) * 2.0;

        let powf = if t < 1.0 {
            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, 10.0 * (t - 1.0));

            #[cfg(feature = "std")]
            let scalar = 2.0f64.powf(10.0 * (t - 1.0));

            scalar / 2.0
        } else {
            let t = t - 1.0;

            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, -10.0 * t);

            #[cfg(feature = "std")]
            let scalar = 2.0f64.powf(-10.0 * t);

            (2.0 - scalar) / 2.0
        };

        let new_value = self.value_delta.scale(powf);

        new_value + self.initial_value
    }
);

pub struct ExpoIn2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for ExpoIn2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        // weird edge in this tween?
        if percent == 0.0 {
            return Value::ZERO;
        }

        let percent = 10.0 * (percent - 1.0);

        #[cfg(feature = "libm")]
        let scalar = libm::pow(2.0, percent_time);

        #[cfg(feature = "std")]
        let scalar = 2.0f64.powf(percent);

        value_delta.scale(scalar)
    }
}

pub struct ExpoOut2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for ExpoOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

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
}

pub struct ExpoInOut2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for ExpoInOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

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
            let t = percent - 1.0;

            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, -10.0 * percent);

            #[cfg(feature = "std")]
            let scalar = 2.0f64.powf(-10.0 * t);

            (2.0 - scalar) / 2.0
        };

        value_delta.scale(powf)
    }
}

test_tween!(Expo);
