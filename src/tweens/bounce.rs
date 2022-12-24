const MAGIC: f64 = 7.5625;
const STAGE_ZERO: f64 = 1.0 / 2.75;
const STAGE_ONE: f64 = 2.0 / 2.75;
const STAGE_TWO: f64 = 2.5 / 2.75;

declare_tween!(
    /// A bouncy tween, similar to gravity. Go [here](https://easings.net/#easeInBounce) for a visual demonstration.
    pub struct BounceIn;

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
);

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

declare_tween!(
    /// A bouncy tween, similar to gravity. Go [here](https://easings.net/#easeInOutBounce) for a visual demonstration.
    pub struct BounceInOut;

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
);

test_tween!(Bounce);
