use crate::{Tween, TweenTime, TweenValue, Tweener};

// mod chain;
mod looper;
mod oscillator;

// pub use chain::Chain;
pub use looper::Looper;
// pub use oscillator::{FixedOscillator, OscillationDirection, Oscillator};

// Just a bundle of data shared throughout this module
// #[derive(Debug, PartialEq, Clone, PartialOrd, Copy)]
// pub(crate) struct TweenData<T>
// where
//     T: Tween,
// {
//     pub(crate) position: T::Time,
//     pub(crate) duration: T::Time,
//     pub(crate) tween: T,
// }
// impl<T> TweenData<T>
// where
//     T: Tween,
// {
//     fn new(tween: T) -> Self {
//         Self {
//             position: T::Time::ZERO,
//             duration: tween.duration(),
//             tween,
//         }
//     }
// }

/// A delta tweener is "drives" a tween for you, allowing
/// you to provide *deltas* in time, rather than new time values.
///
/// This can be significantly easier as a user in a variadic time loop
/// (ie, you loop as fast as you can), since you can now just provide a delta
/// time as a fixed time.
///
/// **NB: [DeltaTweener] will always return at least one value, once. If you provide
/// a delta greater than the duration of the tween, we will return `final_value`, and then
/// will return nothing else afterwards.**
///
/// If, on the other hand, you use a *fixed* time loop, see [FixedTweenDriver],
/// which provides a simpler interface, and implements Iterator.
///
/// ```
/// # use tween::{DeltaTweener, Linear};
///
/// // a tween which takes 10 ticks, and moves a value from 0 to 10.
/// let mut delta_tweener = DeltaTweener::new(0, 100, 10, Linear);
///
/// assert_eq!(delta_tweener.update_by(1), Some(10)); // one tick
/// assert_eq!(delta_tweener.update_by(2), Some(30)); // third tick
/// assert_eq!(delta_tweener.update_by(10), Some(100)); // moves the `delta_tweener` to tick "13", which finishes the tween
/// assert_eq!(delta_tweener.update_by(100), None); // this is way outside
/// ```
#[derive(Debug, PartialEq, Clone, PartialOrd, Copy)]
pub struct DeltaTweener<Value, Time, T>
where
    Time: TweenTime,
    T: Tween<Value, Time>,
{
    tweener: Tweener<Value, Time, T>,
    position: Time,
    has_emitted_final_value: bool,
}

impl<Value, Time, T> DeltaTweener<Value, Time, T>
where
    Value: TweenValue,
    Time: TweenTime,
    T: Tween<Value, Time>,
{
    /// Creates a new [TweenDriver] out of an existing tween.
    pub fn new(start: Value, end: Value, duration: Time, tween: T) -> Self {
        Self {
            tweener: Tweener::new(start, end, duration, tween),
            position: Time::ZERO,
            has_emitted_final_value: false,
        }
    }

    /// Drives the [TweenDriver] forward X steps in time.
    ///
    /// If an input higher than the tween's `duration` is given, you will
    /// receive the max value of the tween.
    pub fn update_by(&mut self, delta: Time) -> Option<Value> {
        self.position += delta;

        Self::_update(&mut self.tweener, &mut self.has_emitted_final_value, self.position)
    }

    fn _update(
        tweener: &mut Tweener<Value, Time, T>,
        has_emitted_final_value: &mut bool,
        position: Time,
    ) -> Option<Value> {
        // we always re-question the percent, since some tweens do funky
        // stuff in there
        let pct = tweener.tween.percent(position, tweener.duration);

        // we made it past 100%. Let's see if we should do anything.
        if pct > 1.0 {
            return if !*has_emitted_final_value {
                *has_emitted_final_value = true;
                Some(tweener.final_value)
            } else {
                None
            };
        }

        // okay, we made it to the last percent....so we don't want to emit the final
        // value after this
        if pct == 1.0 {
            *has_emitted_final_value = true;
        }

        Some(tweener.tween.tween(tweener.value_delta, pct) + tweener.initial_value)
    }
}

/// A FixedTweenDriver "drives" a tween for you, allowing you provide *deltas*
/// instead of concrete values, per call. Moreover, a FixedTweenDriver always works on
/// the same delta per `update`, rather than allowing for a variable delta. If you need a
/// variable delta use [TweenDriver].
///
/// Because fixed tweener works on a fixed delta, it can provide a simple interface, which should
/// be especially useful for games which used a fixed delta update loop.
///
/// ```
/// # use tween::{FixedTweener, Linear};
///
/// // we provide a tweener which goes from 0 up to 4, in 4 ticks,
/// // and we progress it by 1 each time we call it.
/// let mut fixed_tweener = FixedTweener::new(0, 4, 4, 1, Linear);
/// assert_eq!(fixed_tweener.next().unwrap(), 1);
/// assert_eq!(fixed_tweener.next().unwrap(), 2);
/// assert_eq!(fixed_tweener.next().unwrap(), 3);
/// assert_eq!(fixed_tweener.next().unwrap(), 4);
/// assert_eq!(fixed_tweener.next(), None);
/// ```
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct FixedTweener<Value, Time, T> {
    tweener: Tweener<Value, Time, T>,
    position: Time,
    delta: Time,
    has_emitted_final_value: bool,
}

impl<Value, Time, T> FixedTweener<Value, Time, T>
where
    Value: TweenValue,
    Time: TweenTime,
    T: Tween<Value, Time>,
{
    /// Creates a new [FixedTweenDriver], and takes in the delta time
    /// it will use per tick.
    pub fn new(start: Value, end: Value, duration: Time, delta: Time, tween: T) -> Self {
        Self {
            tweener: Tweener::new(start, end, duration, tween),
            position: Time::ZERO,
            has_emitted_final_value: false,
            delta,
        }
    }

    /// Allows inspections of a given tween.
    pub fn tween(&self) -> &T {
        &self.tweener.tween
    }

    /// The current time of the tween.
    pub fn current_time(&self) -> Time {
        self.position
    }

    // /// Converts this tweener to a [FixedLooper].
    // pub fn looper(self) -> FixedLooper<T> {
    //     FixedLooper::new(self)
    // }

    // /// Creates a new FixedOscillator out of this tween as `rising` and a second `falling` tween.
    // /// If either tweener is complete, then they will be reset.
    // ///
    // /// Use `oscillator` to automatically generate an inverse `falling` tween.
    // ///
    // /// Because an arbitrary rising and falling tween are given, you can create piece-wise tweens.
    // pub fn oscillator_with<Falling: Tween<Time = T::Time, Value = T::Value>>(
    //     self,
    //     other: FixedTweener<Falling>,
    // ) -> FixedOscillator<T, Falling> {
    //     FixedOscillator::with_falling(self, other)
    // }
}

// impl<Rising> FixedTweener<Rising>
// where
//     Rising: crate::SizedTween,
// {
//     /// Creates a new FixedOscillator. If the tweener is already complete, then it will
//     /// reset it, and creates a backwards copy of the tween.
//     ///
//     /// The tween given will be assigned as the `rising` tween, whereas the generated inverse
//     /// will be the `falling` tween.
//     pub fn oscillator(self) -> FixedOscillator<Rising> {
//         FixedOscillator::new(self)
//     }
// }

impl<Value, Time, T> Iterator for FixedTweener<Value, Time, T>
where
    Value: TweenValue,
    Time: TweenTime,
    T: Tween<Value, Time>,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.position += self.delta;
        DeltaTweener::_update(&mut self.tweener, &mut self.has_emitted_final_value, self.position)
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Linear;

    #[test]
    fn tweener() {
        let mut tweener = DeltaTweener::new(0, 100, 10, Linear);
        let values: std::vec::Vec<_> = (0..10).map(|_| tweener.update_by(1).unwrap()).collect();

        assert_eq!(*values, [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
    }

    #[test]
    fn fixed_tweener() {
        let mut tweener = FixedTweener::new(0, 100, 10, 1, Linear);
        let values: std::vec::Vec<_> = (0..10).map(|_| tweener.next().unwrap()).collect();

        assert_eq!(*values, [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);

        let mut fixed_tweener = FixedTweener::new(0, 4, 4, 1, Linear);
        assert_eq!(fixed_tweener.next().unwrap(), 1);
        assert_eq!(fixed_tweener.next().unwrap(), 2);
        assert_eq!(fixed_tweener.next().unwrap(), 3);
        assert_eq!(fixed_tweener.next().unwrap(), 4);
        assert_eq!(fixed_tweener.next(), None);
    }

    #[test]
    fn tweener_weird() {
        let mut tweener = DeltaTweener::new(0, 2, 2, Linear);

        assert_eq!(tweener.update_by(0), Some(0));
        assert_eq!(tweener.update_by(1), Some(1));
        assert_eq!(tweener.update_by(1), Some(2));
        assert_eq!(tweener.update_by(0), Some(2));
        assert_eq!(tweener.update_by(0), Some(2));
        assert_eq!(tweener.update_by(0), Some(2));
    }
}
