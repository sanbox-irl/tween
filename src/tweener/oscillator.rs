use super::{FixedTweenDriver, TweenDriver};
use crate::{Tween, TweenTime};

/// An [Oscillator] is a wrapper around a [Tweener], which makes it so that
/// every time the tweener *would* fuse (end), it instead starts reversing back to the start.
///
/// You will always get an end edge on both ends for a tick.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Oscillator<Rising, Falling = Rising>
where
    Rising: Tween<Value = Falling::Value, Time = Falling::Time>,
    Falling: Tween,
{
    rising: TweenDriver<Rising>,
    falling: TweenDriver<Falling>,
    direction: OscillationDirection,
}

impl<Rising> Oscillator<Rising>
where
    Rising: crate::SizedTween,
{
    /// Creates a new Oscillator. If the tweener is already complete, then it will
    /// reset it, and creates a backwards copy of the tween.
    ///
    /// The tween given will be assigned as the `rising` tween, whereas the generated inverse will
    /// be the `falling` tween.
    pub fn new(mut rising: TweenDriver<Rising>) -> Self {
        // unfuse it...
        if rising.fused {
            rising.last_time = Rising::Time::ZERO;
            rising.fused = false;
        }

        let falling = TweenDriver::new(Rising::new(
            rising.tween.final_value(),
            rising.tween.initial_value(),
            rising.tween.duration(),
        ));

        Self {
            rising,
            falling,
            direction: OscillationDirection::Rising,
        }
    }
}

impl<Rising, Falling> Oscillator<Rising, Falling>
where
    Rising: Tween<Value = Falling::Value, Time = Falling::Time>,
    Falling: Tween,
{
    /// Creates a new FixedOscillator out of a `falling` and `rising` tween. If either tweener is
    /// complete, then they will be reset.
    ///
    /// Because an arbitrary rising and falling tween are given, you can create piece-wise tweens.
    pub fn with_falling(mut rising: TweenDriver<Rising>, mut falling: TweenDriver<Falling>) -> Self {
        // unfuse it...
        if rising.fused {
            rising.last_time = Rising::Time::ZERO;
            rising.fused = false;
        }

        // unfuse it...
        if falling.fused {
            falling.last_time = Rising::Time::ZERO;
            falling.fused = false;
        }

        Self {
            rising,
            falling,
            direction: OscillationDirection::Rising,
        }
    }

    /// Drives the inner [Tweener] forward X steps in time, oscillating if required.
    ///
    /// If the delta given is great enough, you may oscillate around several times.
    pub fn update(&mut self, delta: Rising::Time) -> Option<Rising::Value> {
        fn _update<T: Tween>(
            driver: &mut TweenDriver<T>,
            delta: T::Time,
            direction: &mut OscillationDirection,
        ) -> T::Value {
            // we make sure this ALWAYS returns `some`.
            let output = driver.update(delta).unwrap();

            // catch the fused here...
            if driver.fused {
                driver.fused = false;
                driver.last_time = T::Time::ZERO;

                // and flip our direction...
                *direction = match *direction {
                    OscillationDirection::Rising => OscillationDirection::Falling,
                    OscillationDirection::Falling => OscillationDirection::Rising,
                }
            }

            output
        }

        let o = match self.direction {
            OscillationDirection::Rising => _update(&mut self.rising, delta, &mut self.direction),
            OscillationDirection::Falling => _update(&mut self.falling, delta, &mut self.direction),
        };

        Some(o)
    }

    /// Gets the current direction of oscillation.
    pub fn direction(&self) -> OscillationDirection {
        self.direction
    }
}

/// A [FixedOscillator] is a wrapper around a [FixedTweener], which makes it so that
/// every time the tweener *would* fuse (end), it instead starts reversing back to the start.
///
/// You will always get an end edge on both ends for a tick.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FixedOscillator<Rising: Tween, Falling: Tween = Rising> {
    rising: FixedTweenDriver<Rising>,
    falling: FixedTweenDriver<Falling>,
    direction: OscillationDirection,
}

impl<Rising> FixedOscillator<Rising, Rising>
where
    Rising: crate::SizedTween,
{
    /// Creates a new FixedOscillator. If the tweener is already complete, then it will
    /// reset it, and creates a backwards copy of the tween.
    ///
    /// The tween given will be assigned as the `rising` tween, whereas the generated inverse will
    /// be the `falling` tween.
    pub fn new(mut rising: FixedTweenDriver<Rising>) -> Self {
        // unfuse it...
        if rising.0.fused {
            rising.0.last_time = Rising::Time::ZERO;
            rising.0.fused = false;
        }

        let falling = FixedTweenDriver::new(
            Rising::new(
                rising.0.tween.final_value(),
                rising.0.tween.initial_value(),
                rising.0.tween.duration(),
            ),
            rising.1,
        );

        Self {
            rising,
            falling,
            direction: OscillationDirection::Rising,
        }
    }

    /// Gets the current direction of oscillation.
    pub fn direction(&self) -> OscillationDirection {
        self.direction
    }
}

impl<Rising, Falling> FixedOscillator<Rising, Falling>
where
    Rising: Tween,
    Falling: Tween,
{
    /// Creates a new FixedOscillator out of a `falling` and `rising` tween. If either tweener is
    /// complete, then they will be reset.
    ///
    /// Because an arbitrary rising and falling tween are given, you can create piece-wise tweens.
    pub fn with_falling(mut rising: FixedTweenDriver<Rising>, mut falling: FixedTweenDriver<Falling>) -> Self {
        // unfuse it...
        if rising.0.fused {
            rising.0.last_time = Rising::Time::ZERO;
            rising.0.fused = false;
        }

        // unfuse it...
        if falling.0.fused {
            falling.0.last_time = Falling::Time::ZERO;
            falling.0.fused = false;
        }

        Self {
            rising,
            falling,
            direction: OscillationDirection::Rising,
        }
    }
}

impl<Rising, Falling> Iterator for FixedOscillator<Rising, Falling>
where
    Rising: Tween<Value = Falling::Value, Time = Falling::Time>,
    Falling: Tween,
{
    type Item = Rising::Value;

    fn next(&mut self) -> Option<Self::Item> {
        fn _update<T>(driver: &mut FixedTweenDriver<T>, direction: &mut OscillationDirection) -> T::Value
        where
            T: Tween,
        {
            // we make sure this ALWAYS returns `some`.
            let output = driver.next().unwrap();

            // catch the fused here...
            if driver.0.fused {
                driver.0.fused = false;
                driver.0.last_time = T::Time::ZERO;

                // and flip our direction...
                *direction = match *direction {
                    OscillationDirection::Rising => OscillationDirection::Falling,
                    OscillationDirection::Falling => OscillationDirection::Rising,
                }
            }

            output
        }

        let o = match self.direction {
            OscillationDirection::Rising => _update(&mut self.rising, &mut self.direction),
            OscillationDirection::Falling => _update(&mut self.falling, &mut self.direction),
        };

        Some(o)
    }
}

/// This is the direction we are currently travelling. In concerete terms,
/// Oscillating tweens either use their "up" tween or their "down" tween,
/// and this determines which one of those two they choose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum OscillationDirection {
    /// This is the *first* tween chosen. This is the tween generally given in the `new` method.
    #[default]
    Rising,

    /// This is the *second* tween chosen. This is the tween generally created internally as an
    /// inverse.
    Falling,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Linear;

    #[test]
    fn tweener_oscillator() {
        let mut oscillator = Oscillator::new(TweenDriver::new(Linear::new(0, 2, 2)));

        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.update(1).unwrap(), 1);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.update(1).unwrap(), 2);
        assert_eq!(oscillator.direction(), OscillationDirection::Falling);
        assert_eq!(oscillator.update(1).unwrap(), 1);
        assert_eq!(oscillator.direction(), OscillationDirection::Falling);
        assert_eq!(oscillator.update(1).unwrap(), 0);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.update(1).unwrap(), 1);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.update(1).unwrap(), 2);
        assert_eq!(oscillator.direction(), OscillationDirection::Falling);
    }

    #[test]
    fn fixed_tweener_oscillator() {
        let mut oscillator = FixedOscillator::new(FixedTweenDriver::new(Linear::new(0, 2, 2), 1));

        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.next().unwrap(), 1);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.next().unwrap(), 2);
        assert_eq!(oscillator.direction(), OscillationDirection::Falling);
        assert_eq!(oscillator.next().unwrap(), 1);
        assert_eq!(oscillator.direction(), OscillationDirection::Falling);
        assert_eq!(oscillator.next().unwrap(), 0);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.next().unwrap(), 1);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.next().unwrap(), 2);
        assert_eq!(oscillator.direction(), OscillationDirection::Falling);
    }

    #[test]
    fn type_test() {
        let _one_type: Oscillator<Linear<i32, i32>>;
        let _two_type: Oscillator<Linear<i32, i32>, crate::QuadIn<i32, i32>>;

        // let conflict: Oscillator<Linear<i32, i32>, crate::QuadIn<u32, i32>>;
    }
}
