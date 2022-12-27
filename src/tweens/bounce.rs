use crate::{Tween, TweenTime, TweenValue};

const MAGIC: f64 = 7.5625;
const STAGE_ZERO: f64 = 1.0 / 2.75;
const STAGE_ONE: f64 = 2.0 / 2.75;
const STAGE_TWO: f64 = 2.5 / 2.75;

/// A bouncy tween, similar to gravity. Go [here](https://easings.net/#easeInBounce) for a visual demonstration.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct BounceIn;

impl BounceIn {
    /// Creates the Tween
    pub fn new() -> Self {
        Self
    }

    /// Calculate what a percent into the Tween based on time. For almost all Tweens,
    /// this is simply `current_time / duration` (`Bounce` and `Elastic` are the exceptions).
    pub fn percent<Value, Time>(&mut self, current_time: Time, duration: Time) -> f64
    where
        Value: TweenValue,
        Time: TweenTime,
    {
        <Self as Tween<Value, Time>>::percent(self, current_time, duration)
    }

    /// Run the given Tween with a new time.
    pub fn tween<Value, Time>(&mut self, value_delta: Value, percent: f64) -> Value
    where
        Value: TweenValue,
        Time: TweenTime,
    {
        // we pass this through so that we don't require users to (annoyingly) import
        // a trait. Inherent methods in traits pls!
        <Self as Tween<Value, Time>>::tween(self, value_delta, percent)
    }
}

impl<Value, Time> Tween<Value, Time> for BounceIn
where
    Value: TweenValue,
    Time: TweenTime,
{
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

    fn percent(&self, current_time: Time, duration: Time) -> f64 {
        (duration - current_time).to_f64() / duration.to_f64()
    }
}

declare_tween!(
    /// A bouncy tween, similar to gravity. Go [here](https://easings.net/#easeOutBounce) for a visual demonstration.
    pub struct BounceOut;

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
);

/// A bouncy tween, similar to gravity. Go [here](https://easings.net/#easeInOutBounce) for a visual demonstration.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct BounceInOut;

impl BounceInOut {
    /// Creates the Tween
    pub fn new() -> Self {
        Self
    }

    /// Calculate what a percent into the Tween based on time. For almost all Tweens,
    /// this is simply `current_time / duration` (`Bounce` and `Elastic` are the exceptions).
    pub fn percent<Value, Time>(&mut self, current_time: Time, duration: Time) -> f64
    where
        Value: TweenValue,
        Time: TweenTime,
    {
        <Self as Tween<Value, Time>>::percent(self, current_time, duration)
    }

    /// Run the given Tween with a new time.
    pub fn tween<Value, Time>(&mut self, value_delta: Value, percent: f64) -> Value
    where
        Value: TweenValue,
        Time: TweenTime,
    {
        // we pass this through so that we don't require users to (annoyingly) import
        // a trait. Inherent methods in traits pls!
        <Self as Tween<Value, Time>>::tween(self, value_delta, percent)
    }
}

impl<Value, Time> Tween<Value, Time> for BounceInOut
where
    Value: TweenValue,
    Time: TweenTime,
{
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

    fn percent(&self, current_time: Time, duration: Time) -> f64 {
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

    // because we do some demonic shit here to keep this a ZST, we allow for -1 to 1 ranges.
    fn percent_bounds(&self) -> Option<(f64, f64)> {
        Some((-1.0, 1.0))
    }
}

test_tween!(Bounce);
