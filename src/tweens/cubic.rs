use core::marker::PhantomData;

use crate::{Tween2, TweenTime, TweenValue};

declare_tween!(
    /// A cubic tween in. Go [here](https://easings.net/#easeInCubic) for a visual demonstration.
    pub struct CubicIn;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self.value_delta.scale(percent_time * percent_time * percent_time);

        new_value + self.initial_value
    }
);

declare_tween!(
    /// A cubic tween out. Go [here](https://easings.net/#easeOutCubic) for a visual demonstration.
    pub struct CubicOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time) - 1.0;
        let new_value = self.value_delta.scale(percent_time * percent_time * percent_time + 1.0);

        new_value + self.initial_value
    }
);

declare_in_out_tween!(
    /// A cubic tween in and out. Go [here](https://easings.net/#easeInOutCubic) for a visual demonstration.
    pub struct CubicInOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time) * 2.0;

        let scalar = if percent_time < 1.0 {
            percent_time * percent_time * percent_time
        } else {
            let p = percent_time - 2.0;
            p * p * p + 2.0
        };
        let new_value = self.half_delta.scale(scalar);

        new_value + self.initial_value
    }
);

pub struct CubicIn2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for CubicIn2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        value_delta.scale(percent * percent * percent)
    }
}

pub struct CubicOut2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for CubicOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent -= 1.0;

        value_delta.scale(percent * percent * percent + 1.0)
    }
}

pub struct CubicInOut2<Value, Time>(PhantomData<Time>, Value);
impl<Value, Time> Tween2<Value> for CubicInOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent *= 2.0;

        let scalar = if percent < 1.0 {
            percent * percent * percent
        } else {
            let p = percent - 2.0;
            p * p * p + 2.0
        };
        self.1.scale(scalar)
    }
}

test_tween!(Cubic);
