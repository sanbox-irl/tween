use core::marker::PhantomData;

use crate::{Tween2, TweenTime, TweenValue};

const MAGIC: f64 = 7.5625;
const STAGE_ZERO: f64 = 1.0 / 2.75;
const STAGE_ONE: f64 = 2.0 / 2.75;
const STAGE_TWO: f64 = 2.5 / 2.75;

declare_tween!(
    /// An bouncy tween, similar to gravity. Go [here](https://easings.net/#easeInBounce) for a visual demonstration.
    pub struct BounceIn;

    fn run(&mut self, new_time: T) -> V {
        let v = {
            let t = T::percent(self.duration, self.duration - new_time);

            let multip = if t < STAGE_ZERO {
                MAGIC * t * t
            } else if t < STAGE_ONE {
                let t = t - 1.5 / 2.75;
                MAGIC * t * t + 0.75
            } else if t < STAGE_TWO {
                let t = t - 2.25 / 2.75;
                MAGIC * t * t + 0.9375
            } else {
                let t = t - 2.625 / 2.75;

                MAGIC * t * t + 0.984375
            };

            self.value_delta.scale(multip)
        };

        self.value_delta - v + self.initial_value
    }
);

declare_tween!(
    /// An bouncy tween, similar to gravity. Go [here](https://easings.net/#easeOutBounce) for a visual demonstration.
    pub struct BounceOut;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);

        let multip = if t < STAGE_ZERO {
            MAGIC * t * t
        } else if t < STAGE_ONE {
            let t = t - 1.5 / 2.75;
            MAGIC * t * t + 0.75
        } else if t < STAGE_TWO {
            let t = t - 2.25 / 2.75;
            MAGIC * t * t + 0.9375
        } else {
            let t = t - 2.625 / 2.75;

            MAGIC * t * t + 0.984375
        };

        self.value_delta.scale(multip) + self.initial_value
    }
);

declare_tween!(
    /// An bouncy tween, similar to gravity. Go [here](https://easings.net/#easeInOutBounce) for a visual demonstration.
    pub struct BounceInOut;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);

        if t < 0.5 {
            let t = T::percent(self.duration, self.duration - new_time.scale(2.0));

            let multip = if t < STAGE_ZERO {
                MAGIC * t * t
            } else if t < STAGE_ONE {
                let t = t - 1.5 / 2.75;
                MAGIC * t * t + 0.75
            } else if t < STAGE_TWO {
                let t = t - 2.25 / 2.75;
                MAGIC * t * t + 0.9375
            } else {
                let t = t - 2.625 / 2.75;

                MAGIC * t * t + 0.984375
            };

            (self.value_delta - self.value_delta.scale(multip)).scale(0.5) + self.initial_value
        } else {
            let t = T::percent(self.duration, new_time.scale(2.0) - self.duration);

            let multip = if t < STAGE_ZERO {
                MAGIC * t * t
            } else if t < STAGE_ONE {
                let t = t - 1.5 / 2.75;
                MAGIC * t * t + 0.75
            } else if t < STAGE_TWO {
                let t = t - 2.25 / 2.75;
                MAGIC * t * t + 0.9375
            } else {
                let t = t - 2.625 / 2.75;

                MAGIC * t * t + 0.984375
            };

            self.value_delta.scale(multip).scale(0.5) + self.value_delta.scale(0.5) + self.initial_value
        }
    }
);

pub struct BounceIn2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for BounceIn2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        let v = {
            let multip = if percent < STAGE_ZERO {
                MAGIC * percent * percent
            } else if percent < STAGE_ONE {
                let t = percent - 1.5 / 2.75;
                MAGIC * t * t + 0.75
            } else if percent < STAGE_TWO {
                let t = percent - 2.25 / 2.75;
                MAGIC * t * t + 0.9375
            } else {
                let t = percent - 2.625 / 2.75;

                MAGIC * t * t + 0.984375
            };

            value_delta.scale(multip)
        };

        value_delta - v
    }

    fn percent(&mut self, current_time: Self::Time, duration: Self::Time) -> f64 {
        (duration - current_time).to_f64() / duration.to_f64()
    }
}

pub struct BounceOut2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for BounceOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        let multip = if percent < STAGE_ZERO {
            MAGIC * percent * percent
        } else if percent < STAGE_ONE {
            let t = percent - 1.5 / 2.75;
            MAGIC * t * t + 0.75
        } else if percent < STAGE_TWO {
            let t = percent - 2.25 / 2.75;
            MAGIC * t * t + 0.9375
        } else {
            let t = percent - 2.625 / 2.75;

            MAGIC * t * t + 0.984375
        };

        value_delta.scale(multip)
    }
}

pub struct BounceInOut2<Value, Time>(PhantomData<(Value, Time)>);
impl<Value, Time> Tween2<Value> for BounceInOut2<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    type Time = Time;

    fn percent(&mut self, current_time: Self::Time, duration: Self::Time) -> f64 {
        let current_time = current_time.to_f64();
        let duration = duration.to_f64();

        let base_pct = current_time / duration;
        if base_pct < 0.5 {
            // we pass in this negative as a hint to the `tween` function.
            // it's ugly, but bounce is *evil*
            -(duration - current_time * 2.0) / duration
        } else {
            (current_time * 2.0 - duration) / duration
        }
    }

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        if percent < 0.0 {
            percent *= -1.0;

            let multip = if percent < STAGE_ZERO {
                MAGIC * percent * percent
            } else if percent < STAGE_ONE {
                let t = percent - 1.5 / 2.75;
                MAGIC * t * t + 0.75
            } else if percent < STAGE_TWO {
                let t = percent - 2.25 / 2.75;
                MAGIC * t * t + 0.9375
            } else {
                let t = percent - 2.625 / 2.75;

                MAGIC * t * t + 0.984375
            };

            (value_delta - value_delta.scale(multip)).scale(0.5)
        } else {
            let multip = if percent < STAGE_ZERO {
                MAGIC * percent * percent
            } else if percent < STAGE_ONE {
                let t = percent - 1.5 / 2.75;
                MAGIC * t * t + 0.75
            } else if percent < STAGE_TWO {
                let t = percent - 2.25 / 2.75;
                MAGIC * t * t + 0.9375
            } else {
                let t = percent - 2.625 / 2.75;

                MAGIC * t * t + 0.984375
            };

            value_delta.scale(multip).scale(0.5) + value_delta.scale(0.5)
        }
    }
}

test_tween!(Bounce);
