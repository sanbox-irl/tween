use crate::{Tween, TweenTime, TweenValue};

const MAGIC: f32 = 7.5625;
const STAGE_ZERO: f32 = 1.0 / 2.75;
const STAGE_ONE: f32 = 2.0 / 2.75;
const STAGE_TWO: f32 = 2.5 / 2.75;

/// A bouncy tween, similar to gravity. Go [here](https://easings.net/#easeInBounce) for a visual demonstration.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct BounceIn;

impl BounceIn {
    /// Creates the Tween
    pub fn new() -> Self {
        Self
    }

    /// Run the given Tween with a new time.
    pub fn tween<Value>(&mut self, value_delta: Value, mut percent: f32) -> Value
    where
        Value: TweenValue,
    {
        percent = 1.0 - percent;

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
}

impl<Value> Tween<Value> for BounceIn
where
    Value: TweenValue,
{
    #[inline(always)]
    fn tween(&mut self, value_delta: Value, percent: f32) -> Value {
        self.tween(value_delta, percent)
    }
}

impl<Value, Time> crate::Tweener<Value, Time, BounceIn>
where
    Time: TweenTime,
    Value: TweenValue,
{
    /// Creates a new [BounceIn] tween.
    pub fn bounce_in(start: Value, end: Value, duration: Time) -> crate::Tweener<Value, Time, BounceIn> {
        crate::Tweener::new(start, end, duration, BounceIn)
    }

    /// Creates a new [BounceIn] tween at the given time.
    pub fn bounce_in_at(
        start: Value,
        end: Value,
        duration: Time,
        current_time: Time,
    ) -> crate::Tweener<Value, Time, BounceInOut> {
        crate::Tweener::new_at(start, end, duration, BounceInOut, current_time)
    }
}

declare_tween!(
    /// A bouncy tween, similar to gravity. Go [here](https://easings.net/#easeOutBounce) for a visual demonstration.
    pub struct BounceOut;

    /// Creates a new [BounceOut] Tweener.
    pub fn bounce_out;

    /// Creates a new [BounceOut] Tweener at the given time.
    pub fn bounce_out_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f32) -> Value {
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

    /// Run the given Tween with a new time.
    pub fn tween<Value>(&mut self, value_delta: Value, mut percent: f32) -> Value
    where
        Value: TweenValue,
    {
        if percent < 0.5 {
            percent = 1.0 - percent * 2.0;
            // (duration - current_time * 2.0) / duration

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
            percent = (percent - 0.5) * 2.0;

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

impl<Value> Tween<Value> for BounceInOut
where
    Value: TweenValue,
{
    #[inline(always)]
    fn tween(&mut self, value_delta: Value, percent: f32) -> Value {
        self.tween(value_delta, percent)
    }
}

impl<Value, Time> crate::Tweener<Value, Time, BounceInOut>
where
    Time: TweenTime,
    Value: TweenValue,
{
    /// Creates a new [BounceInOut] tween.
    pub fn bounce_in_out(start: Value, end: Value, duration: Time) -> crate::Tweener<Value, Time, BounceInOut> {
        crate::Tweener::new(start, end, duration, BounceInOut)
    }

    /// Creates a new [BounceInOut] tween at the given time.
    pub fn bounce_in_out_at(
        start: Value,
        end: Value,
        duration: Time,
        current_time: Time,
    ) -> crate::Tweener<Value, Time, BounceInOut> {
        crate::Tweener::new_at(start, end, duration, BounceInOut, current_time)
    }
}

test_tween!(Bounce);
