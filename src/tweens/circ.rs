use core::marker::PhantomData;

use crate::{Tween2, TweenTime, TweenValue};

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

pub struct CircIn2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for CircIn2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        #[cfg(feature = "libm")]
        let scalar = 1.0 - libm::sqrt(1.0 - percent * percent);

        #[cfg(feature = "std")]
        let scalar = 1.0 - (1.0 - percent * percent).sqrt();

        value_delta.scale(scalar)
    }
}

pub struct CircOut2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for CircOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        let t = percent - 1.0;

        #[cfg(feature = "libm")]
        let scalar = libm::sqrt(1.0 - t * t);

        #[cfg(feature = "std")]
        let scalar = (1.0 - t * t).sqrt();

        value_delta.scale(scalar)
    }
}

pub struct CircInOut2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for CircInOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent *= 2.0;

        let scalar = if percent < 1.0 {
            #[cfg(feature = "libm")]
            let o = 1.0 - libm::sqrt(1.0 - percent * percent);

            #[cfg(feature = "std")]
            let o = 1.0 - (1.0 - percent * percent).sqrt();

            o
        } else {
            let t = percent - 2.0;

            #[cfg(feature = "libm")]
            let o = libm::sqrt(1.0 - percent * percent) + 1.0;

            #[cfg(feature = "std")]
            let o = (1.0 - t * t).sqrt() + 1.0;

            o
        };

        value_delta.scale(scalar / 2.0)
    }
}

test_tween!(Circ);
