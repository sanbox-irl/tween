use crate::{Tween, TweenTime, TweenValue};

mod extrapolator;
mod looper;
mod oscillator;

pub use extrapolator::Extrapolator;
pub use looper::Looper;
pub use oscillator::Oscillator;

/// A Tweener is a wrapper around a Tween. Although you can tween dynamically using just a raw
/// Tween, this struct will manage state and allow for more naturalistic handling.
///
/// When you create a Tweener, you'll provide the `start`, `end,` and `duration` of the Tween (along
/// with the actual tween itself). All of these are in absolute values. Internally, the Tweener, as
/// with this library, works in "parametric" space, which is a fancy way of describing `0.0..=1.0`,
/// or a percentage of the Tween.
///
/// This struct has two important methods: `move_to` and `move_by`. `move_to` effectively just moves
/// your time into parametric space (ie, takes a tween of length 4, and time 2, and gets the percent
/// 0.5) and clamps the output to the `start` and `end` value in case you tried to `move_to` a time
/// beyond the tween's duration (a negative time, or a `time > duration`). `move_by` does the same,
/// but given a relative value.
///
/// ## Clamping
///
/// A Tweener clamps its output value to within the range of `start..=end` given in [Tweener::new].
/// This can easily happen moving a Tweener with [Tweener::move_by]. To check if that's occured, use
/// [Tweener::is_finished], or, if negative movement is possible, invert [Tweener::is_valid].
///
/// For most users of this library, a common pattern will be:
///
/// ```
/// # use tween::Tweener;
///
/// let (start, end) = (0, 100);
/// let duration = 1;
/// let mut tweener = Tweener::linear(start, end, duration);
///
/// // and then in your main loop...
/// let dt = 1;
/// let mut last_value = None;
/// loop {
///     last_value = Some(tweener.move_by(dt));
///     // we'd want to run `!is_valid()` if negative movement was possible, but we know its not.
///     if tweener.is_finished() {
///         break;
///     }
/// }
/// assert_eq!(last_value, Some(100));
/// ```
///
/// What is `finished` and what is `started` exactly, though? In this library **`is_finished`
/// returns `true` when the Time is at or beyond the duration of the Tweener.** This is a fancy way
/// of saying if the Tweener has ran 5 frames, and its duration was 5, it will return `true` on
/// `is_finished`.
///
/// We can demonstrate that here:
///
/// ```
/// # use tween::Tweener;
/// let (start, end) = (0, 2);
/// let duration = 2;
/// let mut tweener = Tweener::linear(start, end, duration);
///
/// let mut accum = 0;
///
/// loop {
///     accum += dbg!(tweener.move_by(1));
///     if tweener.is_finished() {
///         break;
///     }
/// }
///
/// assert_eq!(accum, 3);
/// ```
///
/// NB: this is probably your assumption as to how this library works, but does mean that in a loop
/// constructed like so, you will never actually see a clamp occur within `move_by`.
///
/// Finally, if you'd like this library to actually *not* clamp your tween, take a look at
/// [Extrapolator].
///
/// ## Iterator
///
/// In situations where the same delta time is alwayas used for `move_by`, you can
/// instead convert a [Tweener] into a [FixedTweener] by `into_fixed`. See [FixedTweener]
/// for more information.
#[derive(Debug, PartialEq, Clone, PartialOrd, Copy)]
pub struct Tweener<Value, Time, T: ?Sized> {
    /// The current time of the Tweener. You can change this value at will without running the
    /// Tween, or change it with `move_by`.
    pub current_time: Time,

    /// The Tweener's total duration.
    pub duration: Time,

    values: (Value, Value),
    value_delta: Value,

    /// The actual underlying Tween.
    pub tween: T,
}

impl<Value, Time, T> Tweener<Value, Time, T>
where
    Time: TweenTime,
    Value: TweenValue,
    T: Tween<Value>,
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

    /// Maps a `Tweener<Value, Time, T>` to a `Tweener<Value, Time, R>`. This can be useful for
    /// boxing inner tweens.
    pub fn map<R: Tween<Value>>(self, mut f: impl FnMut(T) -> R) -> Tweener<Value, Time, R> {
        Tweener {
            current_time: self.current_time,
            duration: self.duration,
            values: self.values,
            value_delta: self.value_delta,
            tween: f(self.tween),
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
    #[inline(always)]
    pub fn move_to(&mut self, position: Time) -> Value {
        self.current_time = position;

        let pct = position.to_f32() / self.duration.to_f32();
        if self.tween.is_finite() {
            if pct < 0.0 {
                return self.values.0;
            } else if pct > 1.0 {
                return self.values.1;
            }
        }

        self.tween.tween(self.value_delta, pct) + self.values.0
    }

    /// Drives the [TweenDriver] forward X steps in time.
    ///
    /// If an input higher than the tween's `duration` is given, you will
    /// receive the max value of the tween.
    #[inline]
    pub fn move_by(&mut self, delta: Time) -> Value {
        self.current_time += delta;

        self.move_to(self.current_time)
    }

    /// The initial value a tween was set to start at.
    #[inline]
    pub fn initial_value(&self) -> Value {
        self.values.0
    }

    /// The final value the tween should end at.
    #[inline]
    pub fn final_value(&self) -> Value {
        self.values.1
    }

    /// Returns `true` is the Tweener's [Self::current_time] is greater than or equal to the lower
    /// bound of the tween's percent range, given by [Tween::percent_bounds]. For most tweens, this
    /// just means that we check if the `tweener.current_time >= 0`.
    ///
    /// Note that for tweens without bounds (infinite tweens like [Looper]), this method will always
    /// return `true`. Moreover, this method does not check if a tweener is *finished*. For
    /// that, use [Self::is_finished].
    pub fn is_started(&self) -> bool {
        let pct = self.current_time.to_f32() / self.duration.to_f32();

        if self.tween.is_finite() { pct >= 0.0 } else { true }
    }

    /// Returns `true` is the Tweener's [Self::current_time] is less than or equal to the upper
    /// bound of the tween's percent range, given by [Tween::percent_bounds]. For most tweens, this
    /// just means that we check if the `tweener.current_time <= self.duration()`.
    ///
    /// Note that for tweens without bounds (infinite tweens like [Looper]), this method will always
    /// return `false`. Moreover, this method does not check if a tweener is *started*. For
    /// that, use [Self::is_started].
    pub fn is_finished(&self) -> bool {
        let pct = self.current_time.to_f32() / self.duration.to_f32();

        if self.tween.is_finite() { pct >= 1.0 } else { false }
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
        let pct = self.current_time.to_f32() / self.duration.to_f32();

        if self.tween.is_finite() {
            (0.0..1.0).contains(&pct)
        } else {
            true
        }
    }

    /// Converts this [Tweener] to a [FixedTweener]. See its documentation for more information.
    pub fn into_fixed(self, delta: Time) -> FixedTweener<Value, Time, T> {
        FixedTweener::from_tweener(self, delta)
    }
}

/// A FixedTweener is a [Tweener] wrapper which implements [Iterator]. To do this,
/// it takes a "fixed" delta on its constructor.
///
/// ## Basic Example
///
/// ```
/// # use tween::{FixedTweener, Tweener, Linear};
///
/// // we provide a tweener which goes from 0 up to 4, in 4 ticks,
/// // and we progress it by 1 each time we call it.
/// let (start, end) = (0, 4);
/// let duration = 4;
/// let delta = 1;
/// let mut fixed_tweener = FixedTweener::linear(start, end, duration, delta);
/// assert_eq!(fixed_tweener.next().unwrap(), 1);
/// assert_eq!(fixed_tweener.next().unwrap(), 2);
/// assert_eq!(fixed_tweener.next().unwrap(), 3);
/// assert_eq!(fixed_tweener.next().unwrap(), 4);
/// assert_eq!(fixed_tweener.next(), None);
/// ```
///
/// ## Clamping
///
/// `FixedTweener`, just [Tweener], clamps its output, but **in its [Iterator] implementation,
/// it returns `None` when it would otherwise clamp!**
///
/// Therefore, in all cases where a `fixed_tweener.is_finished()` is `true`,
/// `fixed_tweener.next().is_none` as well.
///
/// If you *don't* want this behavior, you can instead use `move_next()`, which clamps.
///
/// ```
/// # use tween::{FixedTweener, Tweener, Linear};
/// // a single iteration length tween
/// let mut fixed_tweener = FixedTweener::linear(0, 1, 1, 1);
///
/// // move it to its end...
/// assert_eq!(fixed_tweener.next().unwrap(), 1);
///
/// // and now `.next` returns `None`!
/// assert!(fixed_tweener.next().is_none());
/// assert!(fixed_tweener.is_finished());
///
/// // but you can still use `move_next`. Note how it returns `Value`, not `Option<Value>`
/// assert_eq!(fixed_tweener.move_next(), 1);
/// ```
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct FixedTweener<Value, Time, T: ?Sized> {
    /// The delta upon which we move.
    pub delta: Time,

    /// The internal tweener that we've fixed a Delta to.
    pub tweener: Tweener<Value, Time, T>,
}

impl<Value, Time, T> FixedTweener<Value, Time, T>
where
    Value: TweenValue,
    Time: TweenTime,
    T: Tween<Value>,
{
    /// Creates a new [Tweener] out of a [Tween], start and end [TweenValue], [TweenTime]
    /// duration, and [TweenTime] delta.
    pub fn new(start: Value, end: Value, duration: Time, tween: T, delta: Time) -> Self {
        Self::from_tweener(Tweener::new(start, end, duration, tween), delta)
    }

    /// Creates a new [FixedTweener], and takes in the delta time
    /// it will use per tick.
    pub fn from_tweener(tweener: Tweener<Value, Time, T>, delta: Time) -> Self {
        Self { tweener, delta }
    }

    /// This is the exact same as called `next` via [Iterator] except that it doesn't require a
    /// useless `.unwrap()` because it *clamps* instead.
    #[inline]
    pub fn move_next(&mut self) -> Value {
        self.tweener.move_by(self.delta)
    }
}

impl<Value, Time, T> core::ops::Deref for FixedTweener<Value, Time, T> {
    type Target = Tweener<Value, Time, T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.tweener
    }
}

impl<Value, Time, T> core::ops::DerefMut for FixedTweener<Value, Time, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tweener
    }
}

impl<Value, Time, T> Iterator for FixedTweener<Value, Time, T>
where
    Value: TweenValue,
    Time: TweenTime,
    T: Tween<Value>,
{
    type Item = Value;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.tweener.is_valid() {
            Some(self.move_next())
        } else {
            None
        }
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BounceIn, BounceInOut, BounceOut, ElasticIn, Linear};

    #[test]
    fn tweener() {
        let mut tweener = Tweener::new(0, 100, 10, Linear);
        let values: std::vec::Vec<_> = (0..10).map(|_| tweener.move_by(1)).collect();

        assert_eq!(*values, [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
    }

    #[test]
    fn fixed_tweener() {
        let mut tweener = Tweener::new(0, 100, 10, Linear).into_fixed(1);
        let values: std::vec::Vec<_> = (0..10).map(|_| tweener.next().unwrap()).collect();

        assert_eq!(*values, [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);

        let mut fixed_tweener = Tweener::new(0, 4, 4, Linear).into_fixed(1);
        assert_eq!(fixed_tweener.next().unwrap(), 1);
        assert_eq!(fixed_tweener.next().unwrap(), 2);
        assert_eq!(fixed_tweener.next().unwrap(), 3);
        assert_eq!(fixed_tweener.next().unwrap(), 4);
        assert!(fixed_tweener.is_finished());
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

    #[test]
    fn bounds_checker() {
        fn checker<T>(mut tweener: Tweener<i32, i32, T>)
        where
            T: Tween<i32>,
        {
            fn move_and_return<T>(
                tweener: &mut Tweener<i32, i32, T>,
                f: impl FnOnce(&Tweener<i32, i32, T>) -> bool,
            ) -> bool
            where
                T: Tween<i32>,
            {
                tweener.move_by(1);
                f(tweener)
            }

            assert!(move_and_return(&mut tweener, |t| !t.is_finished()));
            assert!(move_and_return(&mut tweener, |t| t.is_finished()));

            tweener.move_to(-2);

            assert!(move_and_return(&mut tweener, |t| !t.is_started()));
            assert!(move_and_return(&mut tweener, |t| t.is_started()));
            assert!(move_and_return(&mut tweener, |t| t.is_started()));
            assert!(move_and_return(&mut tweener, |t| t.is_started()));
            assert!(move_and_return(&mut tweener, |t| t.is_started()));

            tweener.move_to(-2);

            assert!(move_and_return(&mut tweener, |t| !t.is_valid()));
            assert!(move_and_return(&mut tweener, |t| t.is_valid()));
            assert!(move_and_return(&mut tweener, |t| t.is_valid()));
            assert!(move_and_return(&mut tweener, |t| !t.is_valid()));
        }

        checker(Tweener::new(0, 2, 2, Linear));
        checker(Tweener::new(0, 2, 2, ElasticIn));
        checker(Tweener::new(0, 2, 2, BounceInOut));
        checker(Tweener::new(0, 2, 2, BounceIn));
        checker(Tweener::new(0, 2, 2, BounceOut));
    }

    #[test]
    fn shortcuts() {
        Tweener::back_in(0, 0, 0);
        Tweener::back_out(0, 0, 0);
        Tweener::back_in_out(0, 0, 0);
        Tweener::bounce_in(0, 0, 0);
        Tweener::bounce_out(0, 0, 0);
        Tweener::bounce_in_out(0, 0, 0);
        Tweener::circ_in(0, 0, 0);
        Tweener::circ_out(0, 0, 0);
        Tweener::circ_in_out(0, 0, 0);
        Tweener::cubic_in(0, 0, 0);
        Tweener::cubic_out(0, 0, 0);
        Tweener::cubic_in_out(0, 0, 0);
        Tweener::elastic_in(0, 0, 0);
        Tweener::elastic_out(0, 0, 0);
        Tweener::elastic_in_out(0, 0, 0);
        Tweener::expo_in(0, 0, 0);
        Tweener::expo_out(0, 0, 0);
        Tweener::expo_in_out(0, 0, 0);
        Tweener::linear(0, 0, 0);
        Tweener::quad_in(0, 0, 0);
        Tweener::quad_out(0, 0, 0);
        Tweener::quad_in_out(0, 0, 0);
        Tweener::quart_in(0, 0, 0);
        Tweener::quart_out(0, 0, 0);
        Tweener::quart_in_out(0, 0, 0);
        Tweener::quint_in(0, 0, 0);
        Tweener::quint_out(0, 0, 0);
        Tweener::quint_in_out(0, 0, 0);
        Tweener::sine_in(0, 0, 0);
        Tweener::sine_out(0, 0, 0);
        Tweener::sine_in_out(0, 0, 0);
    }
}
