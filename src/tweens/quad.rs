declare_tween!(
    /// An quadratic tween in. Go [here](https://easings.net/#easeInQuad) for a visual demonstration.
    pub struct QuadIn;

    /// Creates a new [QuadIn] Tweener.
    pub fn quad_in;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f64) -> Value {
        value_delta.scale(percent * percent)
    }
);

declare_tween!(
    /// An quadratic tween out. Go [here](https://easings.net/#easeOutQuad) for a visual demonstration.
    pub struct QuadOut;

    /// Creates a new [QuadOut] Tweener.
    pub fn quad_out;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f64) -> Value {
        value_delta.scale(-percent).scale(percent - 2.0)
    }
);

declare_tween!(
    /// An quadratic tween in and out. Go [here](https://easings.net/#easeInOutQuad) for a visual demonstration.
    pub struct QuadInOut;

    /// Creates a new [QuadInOut] Tweener.
    pub fn quad_in_out;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent *= 2.0;

        let scalar = if percent < 1.0 {
            percent * percent
        } else {
            let p = percent - 1.0;

            (p * (p - 2.0) - 1.0) * -1.0
        };
        value_delta.scale(scalar / 2.0)
    }
);

test_tween!(Quad);
