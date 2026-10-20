const MAGIC: f32 = 7.5625;
const STAGE_ZERO: f32 = 1.0 / 2.75;
const STAGE_ONE: f32 = 2.0 / 2.75;
const STAGE_TWO: f32 = 2.5 / 2.75;

/// The scalar of a [BounceOut] tween at the given percent.
#[inline(always)]
fn bounce(percent: f32) -> f32 {
    if percent < STAGE_ZERO {
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
    }
}

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
        value_delta.scale(bounce(percent))
    }
);

declare_tween!(
    /// A bouncy tween, similar to gravity. Go [here](https://easings.net/#easeInOutBounce) for a visual demonstration.
    pub struct BounceInOut;

    /// Creates a new [BounceInOut] Tweener.
    pub fn bounce_in_out;

    /// Creates a new [BounceInOut] Tweener at the given time.
    pub fn bounce_in_out_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f32) -> Value {
        let scalar = if percent < 0.5 {
            (1.0 - bounce(1.0 - percent * 2.0)) * 0.5
        } else {
            bounce((percent - 0.5) * 2.0) * 0.5 + 0.5
        };

        value_delta.scale(scalar)
    }

);

test_tween!(Bounce);

#[cfg(test)]
mod int_tests {
    use super::*;

    #[test]
    fn bounce_in_out_reaches_odd_int_end() {
        for delta in [3, 7, 255, 12345] {
            assert_eq!(BounceInOut.tween(delta, 1.0), delta);
        }
    }
}
