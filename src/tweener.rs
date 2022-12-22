use crate::{Tween, TweenTime};

// mod chain;
mod looper;
mod oscillator;

// pub use chain::Chain;
pub use looper::{FixedLooper, Looper};
pub use oscillator::{FixedOscillator, OscillationDirection, Oscillator};

/// Just a bundle of data shared throughout this module
#[derive(Debug, PartialEq, Clone, PartialOrd, Copy)]
pub(crate) struct TweenData<T>
where
    T: Tween,
{
    pub(crate) position: T::Time,
    pub(crate) duration: T::Time,
    pub(crate) tween: T,
}
impl<T> TweenData<T>
where
    T: Tween,
{
    fn new(tween: T) -> Self {
        Self {
            position: T::Time::ZERO,
            duration: tween.duration(),
            tween,
        }
    }
}

/// A delta tweener is "drives" a tween for you, allowing
/// you to provide *deltas* in time, rather than new time values.
///
/// This can be significantly easier as a user in a variadic time loop
/// (ie, you loop as fast as you can), since you can now just provide a delta
/// time as a fixed time.
///
/// If, on the other hand, you use a *fixed* time loop, see [FixedTweenDriver],
/// which provides a simpler interface, and implements Iterator.
///
/// ```
/// # use tween::{TweenDriver, Linear};
///
/// // a tween which takes 10 ticks, and moves a value from 0 to 10.
/// let mut delta_tweener = TweenDriver::new(Linear::new(0, 10, 10));
///
/// assert_eq!(delta_tweener.update(1), Some(1)); // one tick
/// assert_eq!(delta_tweener.update(2), Some(3)); // two ticks
/// assert_eq!(delta_tweener.update(100), Some(10)); // completes the tween, returning end value
/// assert_eq!(delta_tweener.update(100), None); // tween is done forever now.
/// ```
#[derive(Debug, PartialEq, Clone, PartialOrd, Copy)]
pub struct TweenDriver<T>
where
    T: Tween,
{
    tween_data: TweenData<T>,
    fused: bool,
}

impl<T: Tween> TweenDriver<T> {
    /// Creates a new [TweenDriver] out of an existing tween.
    pub fn new(tween: T) -> Self {
        Self {
            tween_data: TweenData::new(tween),
            fused: false,
        }
    }

    /// Drives the [TweenDriver] forward X steps in time.
    ///
    /// If an input higher than the tween's `duration` is given, you will
    /// receive the max value of the tween.
    pub fn update(&mut self, delta: T::Time) -> Option<T::Value> {
        if !self.fused {
            self.tween_data.position = self.tween_data.position.add(delta);

            let output = if self.tween_data.position.is_complete(self.tween_data.duration) {
                self.fused = true;
                self.tween_data.position = self.tween_data.duration;

                self.tween_data.tween.final_value()
            } else {
                self.tween_data.tween.run(self.tween_data.position)
            };

            Some(output)
        } else {
            None
        }
    }

    /// Converts this tweener to a [Looper].
    pub fn looper(self) -> Looper<T> {
        Looper::new(self)
    }
}

/// A FixedTweenDriver "drives" a tween for you, allowing you provide *deltas*
/// instead of concrete values, per call. Moreover, a FixedTweenDriver always works on
/// the same delta per `update`, rather than allowing for a variable delta. If you need a variable
/// delta use [TweenDriver].
///
/// Because fixed tweener works on a fixed delta, it can provide a simple interface, which should be
/// especially useful for games which used a fixed delta update loop.
///
/// ```
/// # use tween::{FixedTweenDriver, Linear};
///
/// // we provide a tweener which goes from 0 up to 4, in 4 ticks,
/// // and we progress it by 1 each time we call it.
/// let mut fixed_tweener = FixedTweenDriver::new(Linear::new(0, 4, 4), 1);
/// assert_eq!(fixed_tweener.next().unwrap(), 1);
/// assert_eq!(fixed_tweener.next().unwrap(), 2);
/// assert_eq!(fixed_tweener.next().unwrap(), 3);
/// assert_eq!(fixed_tweener.next().unwrap(), 4);
/// assert_eq!(fixed_tweener.next(), None);
/// ```
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct FixedTweenDriver<T: Tween> {
    tween_data: TweenData<T>,
    fused: bool,
    delta: T::Time,
}

impl<T> FixedTweenDriver<T>
where
    T: Tween,
{
    /// Creates a new [FixedTweenDriver], and takes in the delta time
    /// it will use per tick.
    pub fn new(tween: T, delta: T::Time) -> Self {
        Self {
            tween_data: TweenData::new(tween),
            fused: false,
            delta,
        }
    }

    /// Allows inspections of a given tween.
    pub fn tween(&self) -> &T {
        &self.tween_data.tween
    }

    /// The current time of the tween.
    pub fn current_time(&self) -> T::Time {
        self.tween_data.position
    }

    /// Converts this tweener to a [FixedLooper].
    pub fn looper(self) -> FixedLooper<T> {
        FixedLooper::new(self)
    }

    /// Creates a new FixedOscillator out of this tween as `rising` and a second `falling` tween. If
    /// either tweener is complete, then they will be reset.
    ///
    /// Use `oscillator` to automatically generate an inverse `falling` tween.
    ///
    /// Because an arbitrary rising and falling tween are given, you can create piece-wise tweens.
    pub fn oscillator_with<Falling: Tween<Time = T::Time, Value = T::Value>>(
        self,
        other: FixedTweenDriver<Falling>,
    ) -> FixedOscillator<T, Falling> {
        FixedOscillator::with_falling(self, other)
    }
}

impl<Rising> FixedTweenDriver<Rising>
where
    Rising: crate::SizedTween,
{
    /// Creates a new FixedOscillator. If the tweener is already complete, then it will
    /// reset it, and creates a backwards copy of the tween.
    ///
    /// The tween given will be assigned as the `rising` tween, whereas the generated inverse
    /// will be the `falling` tween.
    pub fn oscillator(self) -> FixedOscillator<Rising> {
        FixedOscillator::new(self)
    }
}

impl<T> Iterator for FixedTweenDriver<T>
where
    T: Tween,
{
    type Item = T::Value;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.fused {
            self.tween_data.position = self.tween_data.position.add(self.delta);

            if self.tween_data.position.is_complete(self.tween_data.tween.duration()) {
                self.fused = true;
                Some(self.tween_data.tween.final_value())
            } else {
                Some(self.tween_data.tween.run(self.tween_data.position))
            }
        } else {
            None
        }
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Linear;

    #[test]
    fn tweener() {
        let tweener = FixedTweenDriver::new(Linear::new(0, 100, 10), 1);
        let values: std::vec::Vec<_> = tweener.collect();

        assert_eq!(*values, [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
    }
}
