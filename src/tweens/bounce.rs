const MAGIC: f32 = 7.5625;
const STAGE_ZERO: f32 = 1.0 / 2.75;
const STAGE_ONE: f32 = 2.0 / 2.75;
const STAGE_TWO: f32 = 2.5 / 2.75;

declare_tween!(
    /// A bouncy tween, similar to gravity. Go [here](https://easings.net/#easeInBounce) for a visual demonstration.
    pub struct BounceIn;

    /// Creates a new [BounceIn] Tweener.
    pub fn bounce_in;

    /// Creates a new [BounceIn] Tweener at the given time.
    pub fn bounce_in_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, mut percent: f32) -> Value {
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

);

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

declare_tween!(
    /// A bouncy tween, similar to gravity. Go [here](https://easings.net/#easeInOutBounce) for a visual demonstration.
    pub struct BounceInOut;

    /// Creates a new [BounceInOut] Tweener.
    pub fn bounce_in_out;

    /// Creates a new [BounceInOut] Tweener at the given time.
    pub fn bounce_in_out_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, mut percent: f32) -> Value {
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

);

test_tween!(Bounce);
