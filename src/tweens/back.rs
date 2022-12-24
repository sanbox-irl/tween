use core::marker::PhantomData;

use crate::{Tween2, TweenTime, TweenValue};

/// This appears to be a magic constant for Back eases. I have no idea
/// where it's from, but we'll use it
const BACK_CONST: f64 = 1.70158;

/// This is another magic constant for the back in out tween.
/// Where it comes from, I do not know!
const BACK_IN_OUT_CONST: f64 = BACK_CONST * 1.525;

declare_tween!(
    /// An tween that goes out and then back in a bit. Go [here](https://easings.net/#easeInBack) for a visual demonstration.
    pub struct BackIn;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);
        let scalar = t * t * ((BACK_CONST + 1.0) * t - BACK_CONST);

        let new_value = self.value_delta.scale(scalar);

        new_value + self.initial_value
    }
);

declare_tween!(
    /// An tween that goes in and then back out a bit. Go [here](https://easings.net/#easeOutBack) for a visual demonstration.
    pub struct BackOut;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) - 1.0;
        let scalar = t * t * ((BACK_CONST + 1.0) * t + BACK_CONST) + 1.0;

        let new_value = self.value_delta.scale(scalar);

        new_value + self.initial_value
    }
);

declare_in_out_tween!(
    /// An tween that goes out, in, and then back in and out a bit. Go [here](https://easings.net/#easeInOutBack) for a visual demonstration.
    pub struct BackInOut;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) * 2.0;

        let scalar = if t < 1.0 {
            t * t * ((BACK_IN_OUT_CONST + 1.0) * t - BACK_IN_OUT_CONST)
        } else {
            let t = t - 2.0;

            t * t * ((BACK_IN_OUT_CONST + 1.0) * t + BACK_IN_OUT_CONST) + 2.0
        };
        let new_value = self.half_delta.scale(scalar);

        new_value + self.initial_value
    }
);

pub struct BackIn2<Value, Time>(PhantomData<(Value, Time)>);

impl<Value, Time> BackIn2<Value, Time> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Value, Time> Tween2<Value> for BackIn2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        let scalar = percent * percent * ((BACK_CONST + 1.0) * percent - BACK_CONST);

        value_delta.scale(scalar)
    }
}

pub struct BackOut2<Value, Time>(PhantomData<(Value, Time)>);

impl<Value, Time> BackOut2<Value, Time> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Value, Time> Tween2<Value> for BackOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        let t = percent - 1.0;
        let scalar = t * t * ((BACK_CONST + 1.0) * t + BACK_CONST) + 1.0;

        value_delta.scale(scalar)
    }
}

pub struct BackInOut2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> BackInOut2<Value, Time> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Value, Time> Tween2<Value> for BackInOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent *= 2.0;

        let scalar = if percent < 1.0 {
            percent * percent * ((BACK_IN_OUT_CONST + 1.0) * percent - BACK_IN_OUT_CONST)
        } else {
            let t = percent - 2.0;

            t * t * ((BACK_IN_OUT_CONST + 1.0) * t + BACK_IN_OUT_CONST) + 2.0
        };

        value_delta.scale(scalar / 2.0)
    }
}

test_tween!(Back);
