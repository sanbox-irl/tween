use super::{FixedTweenDriver, TweenDriver};
use crate::{Tween, TweenTime};

/// An [Oscillator] is a wrapper around a [Tweener], which makes it so that
/// every time the tweener *would* fuse (end), it instead starts reversing back to the start.
///
/// You will always get an end edge on both ends for a tick.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Oscillator<T: Tween> {
    rising: TweenDriver<T>,
    falling: TweenDriver<T>,
    direction: OscillationDirection,
}

impl<T> Oscillator<T>
where
    T: crate::SizedTween,
{
    /// Creates a new Oscillator. If the tweener is already complete, then it will
    /// reset it, and creates a backwards copy of the tween.
    ///
    /// The tween given will be assigned as the `rising` tween, whereas the generated inverse will
    /// be the `falling` tween.
    pub fn new(mut rising: TweenDriver<T>) -> Self {
        // unfuse it...
        if rising.fused {
            rising.last_time = T::Time::ZERO;
            rising.fused = false;
        }

        let falling = TweenDriver::new(T::new(
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

    /// Gets the current direction of oscillation.
    pub fn direction(&self) -> OscillationDirection {
        self.direction
    }
}

impl<T> Oscillator<T>
where
    T: Tween,
{
    /// Creates a new FixedOscillator out of a `falling` and `rising` tween. If either tweener is
    /// complete, then they will be reset.
    ///
    /// Because an arbitrary rising and falling tween are given, you can create piece-wise tweens.
    pub fn with_falling(mut rising: TweenDriver<T>, mut falling: TweenDriver<T>) -> Self {
        // unfuse it...
        if rising.fused {
            rising.last_time = T::Time::ZERO;
            rising.fused = false;
        }

        // unfuse it...
        if falling.fused {
            falling.last_time = T::Time::ZERO;
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
    pub fn update(&mut self, delta: T::Time) -> Option<T::Value> {
        let tweener = match self.direction {
            OscillationDirection::Rising => &mut self.rising,
            OscillationDirection::Falling => &mut self.falling,
        };
        // we make sure this ALWAYS returns `some`.
        let output = tweener.update(delta).unwrap();

        // catch the fused here...
        if tweener.fused {
            tweener.fused = false;
            tweener.last_time = T::Time::ZERO;

            // and flip our direction...
            self.direction = match self.direction {
                OscillationDirection::Rising => OscillationDirection::Falling,
                OscillationDirection::Falling => OscillationDirection::Rising,
            }
        }

        Some(output)
    }
}

/// A [FixedOscillator] is a wrapper around a [FixedTweener], which makes it so that
/// every time the tweener *would* fuse (end), it instead starts reversing back to the start.
///
/// You will always get an end edge on both ends for a tick.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FixedOscillator<T: Tween> {
    rising: FixedTweenDriver<T>,
    falling: FixedTweenDriver<T>,
    direction: OscillationDirection,
}

impl<T> FixedOscillator<T>
where
    T: crate::SizedTween,
{
    /// Creates a new FixedOscillator. If the tweener is already complete, then it will
    /// reset it, and creates a backwards copy of the tween.
    ///
    /// The tween given will be assigned as the `rising` tween, whereas the generated inverse will
    /// be the `falling` tween.
    pub fn new(mut rising: FixedTweenDriver<T>) -> Self {
        // unfuse it...
        if rising.0.fused {
            rising.0.last_time = T::Time::ZERO;
            rising.0.fused = false;
        }

        let falling = FixedTweenDriver::new(
            T::new(
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

impl<T> FixedOscillator<T>
where
    T: Tween,
{
    /// Creates a new FixedOscillator out of a `falling` and `rising` tween. If either tweener is
    /// complete, then they will be reset.
    ///
    /// Because an arbitrary rising and falling tween are given, you can create piece-wise tweens.
    pub fn with_falling(mut rising: FixedTweenDriver<T>, mut falling: FixedTweenDriver<T>) -> Self {
        // unfuse it...
        if rising.0.fused {
            rising.0.last_time = T::Time::ZERO;
            rising.0.fused = false;
        }

        // unfuse it...
        if falling.0.fused {
            falling.0.last_time = T::Time::ZERO;
            falling.0.fused = false;
        }

        Self {
            rising,
            falling,
            direction: OscillationDirection::Rising,
        }
    }
}

impl<T> Iterator for FixedOscillator<T>
where
    T: Tween,
{
    type Item = T::Value;

    fn next(&mut self) -> Option<Self::Item> {
        let tweener = match self.direction {
            OscillationDirection::Rising => &mut self.rising,
            OscillationDirection::Falling => &mut self.falling,
        };
        // we make sure this ALWAYS returns `some`.
        let output = tweener.next().unwrap();

        // catch the fused here...
        if tweener.0.fused {
            tweener.0.fused = false;
            tweener.0.last_time = T::Time::ZERO;

            // and flip our direction...
            self.direction = match self.direction {
                OscillationDirection::Rising => OscillationDirection::Falling,
                OscillationDirection::Falling => OscillationDirection::Rising,
            }
        }

        Some(output)
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
        let mut oscillator: FixedOscillator<Linear<i32, i32>> =
            FixedOscillator::new(FixedTweenDriver::new(Linear::new(0, 2, 2), 1));

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
}
