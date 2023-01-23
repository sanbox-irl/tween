declare_tween!(
    /// A cubic tween in. Go [here](https://easings.net/#easeInCubic) for a visual demonstration.
    pub struct CubicIn;

    /// Creates a new [CubicIn] Tweener.
    pub fn cubic_in;

    /// Creates a new [CubicIn] Tweener at the given time.
    pub fn cubic_in_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f32) -> Value {
        value_delta.scale(percent * percent * percent)
    }
);

declare_tween!(
    /// A cubic tween out. Go [here](https://easings.net/#easeOutCubic) for a visual demonstration.
    pub struct CubicOut;

    /// Creates a new [CubicOut] Tweener.
    pub fn cubic_out;

    /// Creates a new [CubicOut] Tweener at the given time.
    pub fn cubic_out_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, mut percent: f32) -> Value {
        percent -= 1.0;

        value_delta.scale(percent * percent * percent + 1.0)
    }
);

declare_tween!(
    /// A cubic tween in and out. Go [here](https://easings.net/#easeInOutCubic) for a visual demonstration.
    pub struct CubicInOut;

    /// Creates a new [CubicInOut] Tweener.
    pub fn cubic_in_out;

    /// Creates a new [CubicInOut] Tweener at the given time.
    pub fn cubic_in_out_at;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, mut percent: f32) -> Value {
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
