declare_tween!(
    /// An quintic tween in. Go [here](https://easings.net/#easeInQuint) for a visual demonstration.
    pub struct QuintIn;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        value_delta.scale(percent * percent * percent * percent * percent)
    }
);

declare_tween!(
    /// An quintic tween out. Go [here](https://easings.net/#easeOutQuint) for a visual demonstration.
    pub struct QuintOut;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent -= 1.0;
        value_delta.scale(percent * percent * percent * percent * percent + 1.0)
    }
);

declare_tween!(
    /// An quintic tween in out. Go [here](https://easings.net/#easeInOutQuint) for a visual demonstration.
    pub struct QuintInOut;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent *= 2.0;

        let scalar = if percent < 1.0 {
            percent * percent * percent * percent * percent
        } else {
            let p = percent - 2.0;
            p * p * p * p * p + 2.0
        };
        value_delta.scale(scalar / 2.0)
    }
);

test_tween!(Quint);
