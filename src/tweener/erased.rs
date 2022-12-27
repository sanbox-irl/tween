use crate::{FixedTweener, Tween};

use super::{TweenTime, TweenValue, Tweener};

/// A dynamically accessed trait for [Tweener] and [FixedTweener]. To get an owned, boxed struct,
/// use [Tweener::into_erased] or [FixedTweener::into_erased].
///
/// The purpose of this trait is to allow users to create Boxed tweeners generic over only type and
/// value, not the inner tween. This allows users to create complex tweens, but store them simply.
pub trait ErasedTweener<Value, Time> {
    /// Moves the tween to a given Time. If this Tween previously was outside
    /// the allowed range given by [Tween::percent_bounds], this can move it back
    /// into bounds.
    ///
    /// Giving [TweenTime::ZERO] to this function effectively resets a tweener.
    ///
    /// Giving a time outside [Tween::percent_bounds] will move the tween there, but **we will
    /// always clamp the output time**.
    fn move_to(&mut self, position: Time) -> Value;

    /// Drives the [TweenDriver] forward X steps in time.
    ///
    /// If an input higher than the tween's `duration` is given, you will
    /// receive the max value of the tween.
    fn move_by(&mut self, delta: Time) -> Value;

    /// The initial value a tween was set to start at.
    fn initial_value(&self) -> Value;

    /// The final value the tween should end at.
    fn final_value(&self) -> Value;

    /// The current time of the tween.
    fn current_time(&self) -> Time;

    /// This sets the current time *without* re-running the inner Tween.
    /// The only reason to prefer this method over [ErasedTween::move_to] is to save
    /// on computational time in the Tween.
    fn set_current_time(&mut self, time: Time);

    /// The Tweener's total duration.
    fn duration(&self) -> Time;

    /// Sets the total duration. This is semi-chaotic to do to a Tween, and could give a very
    /// strange output, especially for In-Out tweens, but for smooth tweens, will work just
    /// fine.
    fn set_duration(&mut self, time: Time);

    /// Returns `true` is the Tweener's [Self::current_time] is greater than or equal to the lower
    /// bound of the tween's percent range, given by [Tween::percent_bounds]. For most tweens, this
    /// just means that we check if the `tweener.current_time >= 0`.
    ///
    /// Note that for tweens without bounds (infinite tweens like [Looper]), this method will always
    /// return `true`. Moreover, this method does not check if a tweener is *finished*. For
    /// that, use [Self::is_finished].
    fn is_started(&self) -> bool;

    /// Returns `true` is the Tweener's [Self::current_time] is less than or equal to the upper
    /// bound of the tween's percent range, given by [Tween::percent_bounds]. For most tweens, this
    /// just means that we check if the `tweener.current_time <= self.duration()`.
    ///
    /// Note that for tweens without bounds (infinite tweens like [Looper]), this method will always
    /// return `false`. Moreover, this method does not check if a tweener is *started*. For
    /// that, use [Self::is_started].
    fn is_finished(&self) -> bool;

    /// Returns `true` is the Tweener's [Self::current_time] is greater than or equal to the lower
    /// bound and less than or equal to the upper bound given by [Tween::percent_bounds]. For
    /// most tweens, this just means that we check if the ` && tweener.current_time >= 0 &&
    /// tweener.current_time <= self.duration()`.
    ///
    /// Note that for tweens without bounds (infinite tweens like [Looper]), this method will always
    /// return `true`.
    ///
    /// This method is **rarely needed** -- only use it if you are doing some second-order tweening.
    fn is_valid(&self) -> bool;

    /// Converts this [ErasedTweener] into an [ErasedFixedTweener]. See its documentation for more
    /// information.
    #[cfg(feature = "std")]
    fn into_fixed(self: std::boxed::Box<Self>, delta: Time) -> std::boxed::Box<dyn FixedErasedTweener<Value, Time>>;
}

impl<Value, Time, T> ErasedTweener<Value, Time> for Tweener<Value, Time, T>
where
    Value: TweenValue + 'static,
    Time: TweenTime + 'static,
    T: Tween<Value> + 'static,
{
    fn move_to(&mut self, position: Time) -> Value {
        self.move_to(position)
    }

    fn move_by(&mut self, delta: Time) -> Value {
        self.move_by(delta)
    }

    fn initial_value(&self) -> Value {
        self.initial_value()
    }

    fn final_value(&self) -> Value {
        self.final_value()
    }

    fn current_time(&self) -> Time {
        self.current_time
    }

    fn set_current_time(&mut self, time: Time) {
        self.current_time = time;
    }

    fn duration(&self) -> Time {
        self.duration
    }

    fn set_duration(&mut self, time: Time) {
        self.duration = time;
    }

    fn is_started(&self) -> bool {
        self.is_started()
    }

    fn is_finished(&self) -> bool {
        self.is_finished()
    }

    fn is_valid(&self) -> bool {
        self.is_valid()
    }

    #[cfg(feature = "std")]
    fn into_fixed(self: std::boxed::Box<Self>, delta: Time) -> std::boxed::Box<dyn FixedErasedTweener<Value, Time>> {
        (*self).into_fixed(delta).into_erased()
    }
}

impl<Value, Time, T> ErasedTweener<Value, Time> for FixedTweener<Value, Time, T>
where
    Value: TweenValue + 'static,
    Time: TweenTime + 'static,
    T: Tween<Value> + 'static,
{
    fn move_to(&mut self, position: Time) -> Value {
        self.tweener.move_to(position)
    }

    fn move_by(&mut self, delta: Time) -> Value {
        self.tweener.move_by(delta)
    }

    fn initial_value(&self) -> Value {
        self.tweener.initial_value()
    }

    fn final_value(&self) -> Value {
        self.tweener.final_value()
    }

    fn current_time(&self) -> Time {
        self.tweener.current_time()
    }

    fn set_current_time(&mut self, time: Time) {
        self.current_time = time;
    }

    fn duration(&self) -> Time {
        self.duration
    }

    fn set_duration(&mut self, time: Time) {
        self.duration = time;
    }

    fn is_started(&self) -> bool {
        self.tweener.is_started()
    }

    fn is_finished(&self) -> bool {
        self.tweener.is_finished()
    }

    fn is_valid(&self) -> bool {
        self.tweener.is_valid()
    }

    #[cfg(feature = "std")]
    fn into_fixed(
        mut self: std::boxed::Box<Self>,
        delta: Time,
    ) -> std::boxed::Box<dyn FixedErasedTweener<Value, Time>> {
        // i guess we'll make sure to apply that delta?
        self.delta = delta;
        self as _
    }
}

/// A dynamically accessed trait for [Tweener] and [FixedTweener]. To get an owned, boxed struct,
/// use [Tweener::into_erased] or [FixedTweener::into_erased].
///
/// The purpose of this trait is to allow users to create Boxed tweeners generic over only type and
/// value, not the inner tween. This allows users to create complex tweens, but store them simply.
pub trait FixedErasedTweener<Value, Time>: ErasedTweener<Value, Time> + Iterator<Item = Value> {
    /// Sets the inner delta time.
    fn set_delta(&mut self, delta: Time);

    /// Gets the inner delta time.
    fn delta(&self) -> Time;

    /// This is the exact same as called `next` via [Iterator] except **we clamp the output.**
    fn move_next(&mut self) -> Value;
}

impl<Value, Time, T> FixedErasedTweener<Value, Time> for FixedTweener<Value, Time, T>
where
    Value: TweenValue + 'static,
    Time: TweenTime + 'static,
    T: Tween<Value> + 'static,
{
    fn set_delta(&mut self, delta: Time) {
        self.delta = delta;
    }

    fn delta(&self) -> Time {
        self.delta
    }

    fn move_next(&mut self) -> Value {
        self.move_next()
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    use crate::Linear;

    use super::*;
    use std::boxed::Box;

    #[test]
    fn erase() {
        let mut tweener: Box<dyn ErasedTweener<i32, i32>> = Tweener::new(0, 2, 2, Linear).into_erased();

        assert_eq!(tweener.move_by(1), 1);
        assert_eq!(tweener.move_by(1), 2);
        assert_eq!(tweener.move_by(1), 2);
        assert!(tweener.is_finished());

        assert_eq!(tweener.move_to(0), 0);
        assert_eq!(tweener.move_to(1), 1);
        assert_eq!(tweener.move_to(2), 2);
        assert_eq!(tweener.move_to(3), 2);
        assert!(tweener.is_finished());
    }

    #[test]
    fn erase_fixed() {
        let mut tweener = Tweener::new(0, 2, 2, Linear).into_fixed(1).into_erased();

        assert_eq!(tweener.move_next(), 1);
        assert_eq!(tweener.move_next(), 2);
        assert_eq!(tweener.move_next(), 2);
        assert!(tweener.is_finished());

        assert_eq!(tweener.move_to(0), 0);
        assert_eq!(tweener.move_next(), 1);
        assert_eq!(tweener.move_next(), 2);
        assert_eq!(tweener.move_next(), 2);
        assert!(tweener.is_finished());
    }

    #[test]
    fn upgrade_in_dyn() {
        let mut tweener = Tweener::new(0, 2, 2, Linear).into_erased().into_fixed(1);

        assert_eq!(tweener.move_next(), 1);
        assert_eq!(tweener.move_next(), 2);
        assert_eq!(tweener.move_next(), 2);
        assert!(tweener.is_finished());

        assert_eq!(tweener.move_to(0), 0);
        assert_eq!(tweener.move_next(), 1);
        assert_eq!(tweener.move_next(), 2);
        assert_eq!(tweener.move_next(), 2);
        assert!(tweener.is_finished());
    }
}
