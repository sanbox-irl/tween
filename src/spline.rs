use crate::{Linear, Tween, TweenTime, Tweener};

/// A collection of Tweens, spliced together into a "Spline".
///
/// We will execute each tween, in order. If any of the tweens are "infinite"
/// tweens, however, ([Looper], [Oscillator], [Extrapolator], or anything whose [is_finite] returns
/// `false`), then we'll never complete that tween within the spline.
///
/// [Looper]: crate::Looper
/// [Oscillator]: crate::Oscillator
/// [Extrapolator]: crate::Extrapolator
/// [is_finite]: crate::Tween::is_finite
#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Copy)]
pub struct Spline<Time, Holder> {
    pub current_time: Time,
    holder: Holder,
}

impl Spline<i32, (Tweener<f32, i32, Linear>, Tweener<f32, i32, Linear>)> {
    /// Creates a new Spline out of two fixed tweeners
    pub fn new(holder: (Tweener<f32, i32, Linear>, Tweener<f32, i32, Linear>)) -> Self {
        Self {
            current_time: i32::ZERO,
            holder,
        }
    }

    #[inline(always)]
    pub fn move_to(&mut self, position: i32) -> f32 {
        self.current_time = position;

        let mut time_left = self.current_time;

        match crate::tweener::try_move_tween(&mut self.holder.0, time_left) {
            Ok(v) => {
                return v;
            }
            Err(e) => match e {
                crate::TweenOutOfBounds::Before => {
                    return self.holder.0.initial_value();
                }
                crate::TweenOutOfBounds::After => {
                    time_left -= self.holder.0.duration;
                }
            },
        };

        // case #2
        match crate::tweener::try_move_tween(&mut self.holder.1, time_left) {
            Ok(v) => {
                return v;
            }
            Err(e) => match e {
                crate::TweenOutOfBounds::Before => {
                    return self.holder.1.initial_value();
                }
                crate::TweenOutOfBounds::After => {
                    // time_left -= self.holder.0.duration;
                }
            },
        };

        // if we're here, that means we're at our final value
        self.holder.1.final_value()
    }

    #[inline]
    pub fn move_by(&mut self, delta: i32) -> f32 {
        self.current_time += delta;

        self.move_to(self.current_time)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn move_linear() {
        let mut spline = Spline::new((
            Tweener::new(0.0f32, 10.0, 100, Linear),
            Tweener::new(10.0f32, 0.0, 10, Linear),
        ));

        for step in 0..100 {
            let v = spline.move_by(1);
            assert_relative_eq!(v, (step + 1) as f32 / 10.0, max_relative = 0.000001, epsilon = 0.0001);
        }

        assert_relative_eq!(spline.move_by(0), 10.0, max_relative = 0.000001, epsilon = 0.0001);

        for step in 0..10 {
            let v = spline.move_by(1);
            assert_relative_eq!(v, (10 - (step + 1)) as f32, max_relative = 0.000001, epsilon = 0.0001);
        }
    }
}
