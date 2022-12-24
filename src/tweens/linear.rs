use core::marker::PhantomData;

declare_tween!(
    /// A Linear tween is a simple lerp from one value to another.
    pub struct Linear;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = new_time.to_f64() / self.duration().to_f64();
        let new_value = self.value_delta.scale(percent_time);

        new_value + self.initial_value
    }
);

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Linear2<Value, Time>(PhantomData<(Value, Time)>);

impl<Value, Time> Linear2<Value, Time> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Value, Time> crate::Tween2<Value> for Linear2<Value, Time>
where
    Value: crate::TweenValue,
    Time: crate::TweenTime,
{
    type Time = Time;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        value_delta.scale(percent)
    }
}

#[cfg(test)]
mod tests {
    use super::Linear2;
    use approx::assert_ulps_eq;

    #[test]
    fn linear_over_frames() {
        let mut value;
        let mut tweener = crate::Tweener::new(0.0, 100.0, 10, Linear2::new());

        for val in 1..=10 {
            value = tweener.run(val);
            assert_ulps_eq!(value, val as f32 * 10.0);
        }
    }

    #[test]
    fn linear_over_frames_rev() {
        let mut value;
        let mut tweener = crate::Tweener::new(100.0, 0.0, 10, Linear2::new());

        for val in 1..=10 {
            value = tweener.run(val);
            assert_ulps_eq!(value, 100.0 - val as f32 * 10.0);
        }
    }
}
