/// This appears to be a magic constant for Back eases. I have no idea
/// where it's from, but we'll use it
const BACK_CONST: f64 = 1.70158;

/// This is another magic constant for the back in out tween.
/// Where it comes from, I do not know!
const BACK_IN_OUT_CONST: f64 = BACK_CONST * 1.525;

declare_tween! {
    /// A tween that goes out and then back in a bit. Go [here](https://easings.net/#easeInBack) for a visual demonstration.
    pub struct BackIn;

    /// Creates a new [BackIn] Tweener.
    pub fn back_in;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f64) -> Value {
        let scalar = percent * percent * ((BACK_CONST + 1.0) * percent - BACK_CONST);

        value_delta.scale(scalar)
    }
}

declare_tween! {
    /// A tween that goes in and then back out a bit. Go [here](https://easings.net/#easeOutBack) for a visual demonstration.
    pub struct BackOut;

    /// Creates a new [BackOut] Tweener.
    pub fn back_out;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f64) -> Value {
        let t = percent - 1.0;
        let scalar = t * t * ((BACK_CONST + 1.0) * t + BACK_CONST) + 1.0;

        value_delta.scale(scalar)
    }
}

declare_tween! {
    /// A tween that goes out, in, and then back in and out a bit. Go [here](https://easings.net/#easeInOutBack) for a visual demonstration.
    pub struct BackInOut;

    /// Creates a new [BackInOut] Tweener.
    pub fn back_in_out;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent *= 2.0;

        let scalar = if percent < 1.0 {
            percent * percent * ((BACK_IN_OUT_CONST + 1.0) * percent - BACK_IN_OUT_CONST)
        } else {
            let t = percent - 2.0;

            t * t * ((BACK_IN_OUT_CONST + 1.0) * t + BACK_IN_OUT_CONST) + 2.0
        };

        value_delta.scale(scalar / 2.0)
    }
}

test_tween!(Back);
