declare_tween!(
    /// A Linear tween is a simple lerp from one value to another.
    pub struct Linear;

    fn run(&mut self, new_time: T) -> V {
        let percent_time = new_time.to_f64() / self.duration().to_f64();
        let new_value = self.value_delta.scale(percent_time);

        new_value + self.initial_value
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;

    #[test]
    fn linear_over_frames() {
        let mut value = 0.0;
        let mut tweener = Linear::new(value, 100.0, 10);

        for val in 1..=10 {
            value = tweener.run(val);
            assert_ulps_eq!(value, val as f32 * 10.0);
        }
    }

    #[test]
    fn linear_over_frames_rev() {
        let mut value = 100.0;
        let mut tweener = Linear::new(value, 0.0, 10);

        for val in 1..=10 {
            value = tweener.run(val);
            assert_ulps_eq!(value, 100.0 - val as f32 * 10.0);
        }
    }
}
