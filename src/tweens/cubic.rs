declare_tween!(
    /// A cubic tween in. Go [here](https://easings.net/#easeInCubic) for a visual demonstration.
    pub struct CubicIn;

    /// Creates a new [CubicIn] Tweener.
    pub fn cubic_in;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        value_delta.scale(percent * percent * percent)
    }
);

declare_tween!(
    /// A cubic tween out. Go [here](https://easings.net/#easeOutCubic) for a visual demonstration.
    pub struct CubicOut;

    /// Creates a new [CubicOut] Tweener.
    pub fn cubic_out;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent -= 1.0;

        value_delta.scale(percent * percent * percent + 1.0)
    }
);

declare_tween!(
    /// A cubic tween in and out. Go [here](https://easings.net/#easeInOutCubic) for a visual demonstration.
    pub struct CubicInOut;

    /// Creates a new [CubicInOut] Tweener.
    pub fn cubic_in_out;

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
