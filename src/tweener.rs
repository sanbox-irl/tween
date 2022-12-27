use crate::{Tween, TweenTime, TweenValue};

// mod chain;
// mod looper;
// mod oscillator;

// pub use chain::Chain;
// pub use looper::Looper;
// pub use oscillator::{FixedOscillator, OscillationDirection, Oscillator};

/// A Tweener is a wrapper around a Tween. Although you can tween dynamically using just a raw
/// Tween, this struct will manage state and allow for more naturalistic handling.
///
/// ## Clamping
///
/// ## Iterator
///
/// In situations where the same delta time is alwayas used for (move_by)[Self::move_by], you can
/// instead convert a [Tweener] into a [FixedTweener] by [Tweener::to_fixed]. See [FixedTweener] for
/// more information.
#[derive(Debug, PartialEq, Clone, PartialOrd, Copy)]
pub struct Tweener<Value, Time, T> {
    /// The current time of the Tweener. You can change this value at will without running the
    /// Tween, or change it with `move_by`.
    pub current_time: Time,
    /// The Tweener's total duration.
    pub duration: Time,
    /// The actual underlying Tween.
    pub tween: T,

    values: (Value, Value),
    value_delta: Value,
}

impl<Value, Time, T> Tweener<Value, Time, T>
where
    Time: TweenTime,
    Value: TweenValue,
    T: Tween<Value, Time>,
{
    /// Creates a new [Tweener] out of a [Tween], start and end [TweenValue], and [TweenTime]
    /// duration.
    pub fn new(start: Value, end: Value, duration: Time, tween: T) -> Self {
        Self {
            values: (start, end),
            value_delta: end - start,
            duration,
            tween,
            current_time: Time::ZERO,
        }
    }

    /// Moves the tween to a given Time. If this Tween previously was outside
    /// the allowed range given by [Tween::percent_bounds], this can move it back
    /// into bounds.
    ///
    /// Giving [TweenTime::ZERO] to this function effectively resets a tweener.
    ///
    /// Giving a time outside [Tween::percent_bounds] will move the tween there, but **we will
    /// always clamp the output time**.
    pub fn move_to(&mut self, position: Time) -> Value {
        self.current_time = position;
        let pct = self.tween.percent(self.current_time, self.duration);
        if let Some((lower, upper)) = self.tween.percent_bounds() {
            if pct < lower {
                return self.initial_value();
            } else if pct > upper {
                return self.final_value();
            }
        }

        self.tween.tween(self.value_delta, pct) + self.values.0
    }

    /// Drives the [TweenDriver] forward X steps in time.
    ///
    /// If an input higher than the tween's `duration` is given, you will
    /// receive the max value of the tween.
    pub fn move_by(&mut self, delta: Time) -> Value {
        self.current_time += delta;

        self.move_to(self.current_time)
    }

    /// The initial value a tween was set to start at.
    pub fn initial_value(&self) -> Value {
        self.values.0
    }

    /// The final value the tween should end at.
    pub fn final_value(&self) -> Value {
        self.values.1
    }

    /// The current time of the tween.
    pub fn current_time(&self) -> Time {
        self.current_time
    }

    /// Returns `true` is the Tweener's [Self::current_time] is greater than or equal to the lower
    /// bound of the tween's percent range, given by [Tween::percent_bounds]. For most tweens, this
    /// just means that we check if the `tweener.current_time >= 0`.
    ///
    /// Note that for tweens without bounds (infinite tweens like [Looper]), this method will always
    /// return `true`. Moreover, this method does not check if a tweener is *finished*. For
    /// that, use [Self::is_finished].
    pub fn is_started(&self) -> bool {
        let pct = self.tween.percent(self.current_time, self.duration);

        if let Some((lower, _upper)) = self.tween.percent_bounds() {
            pct.partial_cmp(&lower).map(|v| v.is_ge()).unwrap_or(false)
        } else {
            true
        }
    }

    /// Returns `true` is the Tweener's [Self::current_time] is less than or equal to the upper
    /// bound of the tween's percent range, given by [Tween::percent_bounds]. For most tweens, this
    /// just means that we check if the `tweener.current_time <= self.duration()`.
    ///
    /// Note that for tweens without bounds (infinite tweens like [Looper]), this method will always
    /// return `false`. Moreover, this method does not check if a tweener is *started*. For
    /// that, use [Self::is_started].
    pub fn is_finished(&self) -> bool {
        let pct = self.tween.percent(self.current_time, self.duration);

        if let Some((_, upper)) = self.tween.percent_bounds() {
            pct.partial_cmp(&upper).map(|v| v.is_le()).unwrap_or(false)
        } else {
            false
        }
    }

    /// Returns `true` is the Tweener's [Self::current_time] is greater than or equal to the lower
    /// bound and less than or equal to the upper bound given by [Tween::percent_bounds]. For
    /// most tweens, this just means that we check if the ` && tweener.current_time >= 0 &&
    /// tweener.current_time <= self.duration()`.
    ///
    /// Note that for tweens without bounds (infinite tweens like [Looper]), this method will always
    /// return `true`.
    ///
    /// This method is **rarely needed** -- only use it if you are doing some second-order tweening.
    pub fn is_valid(&self) -> bool {
        let pct = self.tween.percent(self.current_time, self.duration);
        if let Some((lower, upper)) = self.tween.percent_bounds() {
            if pct < lower || pct > upper {
                return false;
            }
        }

        true
    }

    /// Converts this [Tweener] to a [FixedTweener]. See its documentation for more information.
    pub fn to_fixed(self, delta: Time) -> FixedTweener<Value, Time, T> {
        FixedTweener::new(self, delta)
    }
}

impl<Value, Time> Tweener<Value, Time, crate::SineIn>
where
    Time: TweenTime,
    Value: TweenValue,
{
    /// Creates a new [SineIn] tween.
    pub fn sine_in(start: Value, end: Value, duration: Time) -> Tweener<Value, Time, crate::SineIn> {
        Tweener::new(start, end, duration, crate::SineIn)
    }
}

/// A FixedTweener is a [Tweener] wrapper which implements [Iterator]. To do this,
/// it takes a "fixed" delta on its constructor.
///
/// ```
/// # use tween::{FixedTweener, Tweener, Linear};
///
/// // we provide a tweener which goes from 0 up to 4, in 4 ticks,
/// // and we progress it by 1 each time we call it.
/// let (start, end) = (0, 4);
/// let duration = 4;
/// let delta = 1;
/// let mut fixed_tweener = FixedTweener::new(Tweener::new(start, end, duration, Linear), delta);
/// assert_eq!(fixed_tweener.next().unwrap(), 1);
/// assert_eq!(fixed_tweener.next().unwrap(), 2);
/// assert_eq!(fixed_tweener.next().unwrap(), 3);
/// assert_eq!(fixed_tweener.next().unwrap(), 4);
/// assert_eq!(fixed_tweener.next(), None);
/// ```
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct FixedTweener<Value, Time, T> {
    /// The internal tweener that we've fixed a Delta to.
    pub tweener: Tweener<Value, Time, T>,

    /// The delta upon which we move.
    pub delta: Time,
}

impl<Value, Time, T> FixedTweener<Value, Time, T>
where
    Value: TweenValue,
    Time: TweenTime,
    T: Tween<Value, Time>,
{
    /// Creates a new [FixedTweener], and takes in the delta time
    /// it will use per tick.
    pub fn new(tweener: Tweener<Value, Time, T>, delta: Time) -> Self {
        Self { tweener, delta }
    }

    /// This is the exact same as called `next` via [Iterator] except **we clamp the output.**
    pub fn move_next(&mut self) -> Value {
        self.tweener.move_by(self.delta)
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

impl<Value, Time, T> std::ops::Deref for FixedTweener<Value, Time, T> {
    type Target = Tweener<Value, Time, T>;

    fn deref(&self) -> &Self::Target {
        &self.tweener
    }
}

impl<Value, Time, T> std::ops::DerefMut for FixedTweener<Value, Time, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tweener
    }
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
        let output = self.move_next();

        if self.tweener.is_valid() { Some(output) } else { None }
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Linear;

    #[test]
    fn tweener() {
        let mut tweener = Tweener::new(0, 100, 10, Linear);
        let values: std::vec::Vec<_> = (0..10).map(|_| tweener.move_by(1)).collect();

        assert_eq!(*values, [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
    }

    #[test]
    fn fixed_tweener() {
        let mut tweener = Tweener::new(0, 100, 10, Linear).to_fixed(1);
        let values: std::vec::Vec<_> = (0..10).map(|_| tweener.next().unwrap()).collect();

        assert_eq!(*values, [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);

        let mut fixed_tweener = Tweener::new(0, 4, 4, Linear).to_fixed(1);
        assert_eq!(fixed_tweener.next().unwrap(), 1);
        assert_eq!(fixed_tweener.next().unwrap(), 2);
        assert_eq!(fixed_tweener.next().unwrap(), 3);
        assert_eq!(fixed_tweener.next().unwrap(), 4);
        assert_eq!(fixed_tweener.next(), None);
    }

    #[test]
    fn tweener_weird() {
        let mut tweener = Tweener::new(0, 2, 2, Linear);

        assert_eq!(tweener.move_by(0), 0);
        assert_eq!(tweener.move_by(1), 1);
        assert_eq!(tweener.move_by(1), 2);
        assert_eq!(tweener.move_by(0), 2);
        assert_eq!(tweener.move_by(0), 2);
        assert_eq!(tweener.move_by(0), 2);
    }
}
