use core::marker::PhantomData;

use crate::{Tween2, TweenTime, TweenValue};

declare_tween!(
    /// An quadratic tween in. Go [here](https://easings.net/#easeInQuad) for a visual demonstration.
    pub struct QuadIn;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self.value_delta.scale(percent_time * percent_time);

        new_value + self.initial_value
    }
);

declare_tween!(
    /// An quadratic tween out. Go [here](https://easings.net/#easeOutQuad) for a visual demonstration.
    pub struct QuadOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self.value_delta.scale(-percent_time).scale(percent_time - 2.0);

        new_value + self.initial_value
    }
);

declare_in_out_tween!(
    /// An quadratic tween in and out. Go [here](https://easings.net/#easeInOutQuad) for a visual demonstration.
    pub struct QuadInOut;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time) * 2.0;

        let scalar = if percent_time < 1.0 {
            percent_time * percent_time
        } else {
            let p = percent_time - 1.0;

            (p * (p - 2.0) - 1.0) * -1.0
        };
        let new_value = self.half_delta.scale(scalar);

        new_value + self.initial_value
    }
);

pub struct QuadIn2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for QuadIn2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        value_delta.scale(percent * percent)
    }
}

pub struct QuadOut2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for QuadOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        value_delta.scale(-percent).scale(percent - 2.0)
    }
}

pub struct QuadInOut2<Value, Time>(PhantomData<Time>, Value);
impl<Value, Time> Tween2<Value> for QuadInOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent *= 2.0;

        let scalar = if percent < 1.0 {
            percent * percent
        } else {
            let p = percent - 1.0;

            (p * (p - 2.0) - 1.0) * -1.0
        };
        self.1.scale(scalar)
    }
}

test_tween!(Quad);
