use crate::{Tween, TweenTime, TweenValue};

/// This appears to be a magic constant for Back eases. I have no idea
/// where it's from, but we'll use it
const BACK_CONST: f64 = 1.70158;

/// This is another magic constant for the back in out tween.
/// Where it comes from, I do not know!
const BACK_IN_OUT_CONST: f64 = BACK_CONST * 1.525;

declare_tween!(
    /// An tween that goes out and then back in a bit. Go [here](https://easings.net/#easeInBack) for a visual demonstration.
    pub struct BackIn;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);
        let scalar = t * t * ((BACK_CONST + 1.0) * t - BACK_CONST);

        let new_value = self.value_delta.scale(scalar);

        new_value.add(self.initial_value)
    }
);

declare_tween!(
    /// An tween that goes in and then back out a bit. Go [here](https://easings.net/#easeOutBack) for a visual demonstration.
    pub struct BackOut;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) - 1.0;
        let scalar = t * t * ((BACK_CONST + 1.0) * t + BACK_CONST) + 1.0;

        let new_value = self.value_delta.scale(scalar);

        new_value.add(self.initial_value)
    }
);

declare_in_out_tween!(
    /// An tween that goes out, in, and then back in and out a bit. Go [here](https://easings.net/#easeInOutBack) for a visual demonstration.
    pub struct BackInOut;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time) * 2.0;

        let scalar = if t < 1.0 {
            t * t * ((BACK_IN_OUT_CONST + 1.0) * t - BACK_IN_OUT_CONST)
        } else {
            let t = t - 2.0;

            t * t * ((BACK_IN_OUT_CONST + 1.0) * t + BACK_IN_OUT_CONST) + 2.0
        };
        let new_value = self.half_delta.scale(scalar);

        new_value.add(self.initial_value)
    }
);

test_tween!(Back);
