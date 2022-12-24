use super::{FixedTweenDriver, TweenDriver};
use crate::{Tween, TweenData, TweenTime};

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
    rising: TweenData<Rising>,
    falling: TweenData<Falling>,
    position: Rising::Time,
    total_duration: Rising::Time,
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
    pub fn new(rising: TweenDriver<Rising>) -> Self {
        let falling = TweenData::new(Rising::new(
            rising.tween_data.tween.final_value(),
            rising.tween_data.tween.initial_value(),
            rising.tween_data.tween.duration(),
        ));

        let position = rising.tween_data.position;
        let duration = rising.tween_data.duration + falling.duration;

        Self {
            rising: rising.tween_data,
            falling,
            position,
            total_duration: duration,
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
    pub fn with_falling(rising: TweenDriver<Rising>, falling: TweenDriver<Falling>) -> Self {
        let position = rising.tween_data.position;
        let duration = rising.tween_data.duration + falling.tween_data.duration;

        Self {
            rising: rising.tween_data,
            falling: falling.tween_data,
            position,
            total_duration: duration,
            direction: OscillationDirection::Rising,
        }
    }

    /// Drives the inner [Tweener] forward X steps in time, oscillating if required.
    ///
    /// If the delta given is great enough, you may oscillate around several times.
    pub fn update(&mut self, delta: Rising::Time) -> Rising::Value {
        self.position = (self.position + delta) % self.total_duration;

        if self.position == Rising::Time::ZERO {
            self.direction = OscillationDirection::Falling;
            self.falling.tween.final_value()
        } else if self.position.eq(&self.rising.duration) {
            self.direction = OscillationDirection::Rising;
            self.rising.tween.final_value()
        } else if self.position >= self.rising.duration {
            self.direction = OscillationDirection::Falling;
            self.falling.tween.run(self.position - self.rising.duration)
        } else {
            self.direction = OscillationDirection::Rising;
            self.rising.tween.run(self.position)
        }
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
    rising: TweenData<Rising>,
    rising_delta: Rising::Time,
    falling: TweenData<Falling>,
    falling_delta: Rising::Time,
    position: Rising::Time,
    total_duration: Rising::Time,
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
    pub fn new(rising: FixedTweenDriver<Rising>) -> Self {
        let falling = TweenData::new(Rising::new(
            rising.tween_data.tween.final_value(),
            rising.tween_data.tween.initial_value(),
            rising.tween_data.tween.duration(),
        ));

        let position = rising.tween_data.position;
        let total_duration = rising.tween_data.duration + falling.duration;

        Self {
            rising: rising.tween_data,
            falling,
            position,
            rising_delta: rising.delta,
            falling_delta: rising.delta,
            total_duration,
            direction: OscillationDirection::Rising,
        }
    }
}

impl<Rising, Falling> FixedOscillator<Rising, Falling>
where
    Rising: Tween<Value = Falling::Value, Time = Falling::Time>,
    Falling: Tween,
{
    /// Creates a new FixedOscillator out of a `falling` and `rising` tween. If either tweener is
    /// complete, then they will be reset.
    ///
    /// Because an arbitrary rising and falling tween are given, you can create piece-wise tweens.
    pub fn with_falling(rising: FixedTweenDriver<Rising>, falling: FixedTweenDriver<Falling>) -> Self {
        let position = rising.tween_data.position;
        let total_duration = rising.tween_data.duration + falling.tween_data.duration;

        Self {
            rising: rising.tween_data,
            falling: falling.tween_data,
            position,
            rising_delta: rising.delta,
            falling_delta: falling.delta,
            total_duration,
            direction: OscillationDirection::Rising,
        }
    }

    /// Gets the current direction of oscillation.
    pub fn direction(&self) -> OscillationDirection {
        self.direction
    }
}

impl<Rising, Falling> Iterator for FixedOscillator<Rising, Falling>
where
    Rising: Tween<Value = Falling::Value, Time = Falling::Time>,
    Falling: Tween,
{
    type Item = Rising::Value;

    fn next(&mut self) -> Option<Self::Item> {
        let delta = match self.direction() {
            OscillationDirection::Rising => self.rising_delta,
            OscillationDirection::Falling => self.falling_delta,
        };

        self.position = (self.position + delta) % self.total_duration;

        let o = if self.position == Rising::Time::ZERO {
            self.direction = OscillationDirection::Falling;
            self.falling.tween.final_value()
        } else if self.position.eq(&self.rising.duration) {
            self.direction = OscillationDirection::Rising;
            self.rising.tween.final_value()
        } else if self.position >= self.rising.duration {
            self.direction = OscillationDirection::Falling;
            self.falling.tween.run(self.position - self.rising.duration)
        } else {
            self.direction = OscillationDirection::Rising;
            self.rising.tween.run(self.position)
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
        assert_eq!(oscillator.update(1), 1);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.update(1), 2);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.update(1), 1);
        assert_eq!(oscillator.direction(), OscillationDirection::Falling);
        assert_eq!(oscillator.update(1), 0);
        assert_eq!(oscillator.direction(), OscillationDirection::Falling);
        assert_eq!(oscillator.update(1), 1);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.update(1), 2);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
    }

    #[test]
    fn tweener_oscillator_big_loop() {
        let mut oscillator = Oscillator::new(TweenDriver::new(Linear::new(0, 2, 2)));

        assert_eq!(oscillator.update(2), 2);
        assert_eq!(oscillator.update(1), 1);
        assert_eq!(oscillator.update(2), 1);
    }

    #[test]
    fn fixed_tweener_oscillator() {
        let mut oscillator = FixedOscillator::new(FixedTweenDriver::new(Linear::new(0, 2, 2), 1));

        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.next().unwrap(), 1);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.next().unwrap(), 2);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.next().unwrap(), 1);
        assert_eq!(oscillator.direction(), OscillationDirection::Falling);
        assert_eq!(oscillator.next().unwrap(), 0);
        assert_eq!(oscillator.direction(), OscillationDirection::Falling);
        assert_eq!(oscillator.next().unwrap(), 1);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
        assert_eq!(oscillator.next().unwrap(), 2);
        assert_eq!(oscillator.direction(), OscillationDirection::Rising);
    }

    #[test]
    fn type_test() {
        let _one_type: Oscillator<Linear<i32, i32>>;
        // let _two_type: Oscillator<Linear<i32, i32>, crate::QuadIn<i32, i32>>;

        // let conflict: Oscillator<Linear<i32, i32>, crate::QuadIn<u32, i32>>;
    }
}
