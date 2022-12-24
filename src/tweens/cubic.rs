declare_tween!(
    /// A cubic tween in. Go [here](https://easings.net/#easeInCubic) for a visual demonstration.
    pub struct CubicIn;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        value_delta.scale(percent * percent * percent)
    }
);

declare_tween!(
    /// A cubic tween out. Go [here](https://easings.net/#easeOutCubic) for a visual demonstration.
    pub struct CubicOut;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent -= 1.0;

        value_delta.scale(percent * percent * percent + 1.0)
    }
);

declare_tween!(
    /// A cubic tween in and out. Go [here](https://easings.net/#easeInOutCubic) for a visual demonstration.
    pub struct CubicInOut;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent *= 2.0;

        let scalar = if percent < 1.0 {
            percent * percent * percent
        } else {
            let p = percent - 2.0;
            p * p * p + 2.0
        };
        value_delta.scale(scalar / 2.0)
    }
);

test_tween!(Cubic);
