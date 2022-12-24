declare_tween!(
    /// An quartic tween in. Go [here](https://easings.net/#easeInQuart) for a visual demonstration.
    pub struct QuartIn;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        value_delta.scale(percent * percent * percent * percent)
    }
);

declare_tween!(
    /// An quartic tween out. Go [here](https://easings.net/#easeOutQuart) for a visual demonstration.
    pub struct QuartOut;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent -= 1.0;
        value_delta.scale(-(percent * percent * percent * percent - 1.0))
    }
);

declare_tween!(
    /// An quartic tween in and out. Go [here](https://easings.net/#easeInOutQuart) for a visual demonstration.
    pub struct QuartInOut;

    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent *= 2.0;

        let scalar = if percent < 1.0 {
            percent * percent * percent * percent
        } else {
            let p = percent - 2.0;
            -(p * p * p * p - 2.0)
        };
        value_delta.scale(scalar / 2.0)
    }
);

test_tween!(Quart);
