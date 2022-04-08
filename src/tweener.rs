use crate::{Tween, TweenTime};

/// A delta tweener is "drives" a tween for you, allowing
/// you to provide *deltas* in time, rather than new time values.
///
/// This can be significantly easier as a user in a variadic time loop
/// (ie, you loop as fast as you can), since you can now just provide a delta
/// time as a fixed time.
///
/// If, on the other hand, you use a *fixed* time loop, see [FixedTweener],
/// which provides a simpler interface, and implements Iterator.
///
/// ```
/// # use tween::{Tweener, Linear};
///
/// // a tween which takes 10 ticks, and moves a value from 0 to 10.
/// let mut delta_tweener = Tweener::new(Linear::new(0..=10, 10));
///
/// assert_eq!(delta_tweener.update(1), Some(1)); // one tick
/// assert_eq!(delta_tweener.update(2), Some(3)); // two ticks
/// assert_eq!(delta_tweener.update(100), Some(10)); // completes the tween, returning end value
/// assert_eq!(delta_tweener.update(100), None); // tween is done forever now.
/// ```
pub struct Tweener<T: Tween> {
    tween: T,
    last_time: T::Time,
    fused: bool,
}

impl<T> Tweener<T>
where
    T: Tween,
{
    /// Creates a new [Tweener] out of an existing tween.
    pub fn new(tween: T) -> Self {
        Self {
            tween,
            last_time: T::Time::ZERO,
            fused: false,
        }
    }

    /// Drives the [Tweener] forward X steps in time.
    ///
    /// If an input higher than the tween's `duration` is given, you will
    /// receive the max value of the tween.
    pub fn update(&mut self, delta: T::Time) -> Option<T::Value> {
        if !self.fused {
            self.last_time = self.last_time.add(delta);

            if self.last_time.is_complete(self.tween.duration()) {
                self.fused = true;
                Some(*self.tween.range().end())
            } else {
                Some(self.tween.run(self.last_time))
            }
        } else {
            None
        }
    }

    // /// Converts this tweener to a [Looper].
    // pub fn looper(self) -> Looper<T, TValue, TTime> {
    //     Looper::new(self)
    // }
}

// /// A [Looper] is a wrapper around a [Tweener], which makes it so that
// /// every time the tweener *would*
// pub struct Looper<T, TValue, TTime>(Tweener<T, TValue, TTime>);

// impl<T, TValue, TTime> Looper<T, TValue, TTime>
// where
//     TValue: TweenValue,
//     TTime: TweenTime,
//     T: Tween<Time = TTime, Value = TValue>,
// {
//     /// Creates a new Looper around a [Tweener].
//     ///
//     /// If the [Tweener] is *already* fused, this will reset it to starting
//     /// values.
//     pub fn new(mut delta_tweener: Tweener<T, TValue, TTime>) -> Self {
//         // unfuse it...
//         if delta_tweener.fused {
//             delta_tweener.last_time = TTime::ZERO;
//             delta_tweener.fused = false;
//         }

//         Self(delta_tweener)
//     }

//     /// Drives the inner [Tweener] forward X steps in time, looping if required.
//     ///
//     /// If the delta given is great enough, you may loop around several times.
//     pub fn update(&mut self, delta: TTime) -> Option<TValue> {
//         let output = self.0.update(delta).unwrap(); // we make sure this ALWAYS returns `some`.

//         // catch the fused here...
//         if self.0.fused {
//             self.0.last_time = TTime::ZERO;
//             self.0.fused = false;
//         }

//         Some(output)
//     }
// }

/// A FixedTweener "drives" a tween for you, allowing you provide *deltas*
/// instead of concrete values, per call. Moreover, a FixedTweener always works on
/// the same delta per `update`, rather than allowing for a variable delta. If you need a variable
/// delta use [Tweener].
///
/// Because fixed tweener works on a fixed delta, it can provide a simple interface, which should be
/// especially useful for games which used a fixed delta update loop.
///
/// ```
/// # use tween::{FixedTweener, Linear};
///
/// // we provide a tweener which goes from 0 up to 4, in 4 ticks,
/// // and we progress it by 1 each time we call it.
/// let mut fixed_tweener = FixedTweener::new(Linear::new(0..=4, 4), 1);
/// assert_eq!(fixed_tweener.next().unwrap(), 1);
/// assert_eq!(fixed_tweener.next().unwrap(), 2);
/// assert_eq!(fixed_tweener.next().unwrap(), 3);
/// assert_eq!(fixed_tweener.next().unwrap(), 4);
/// assert_eq!(fixed_tweener.next(), None);
/// ```
pub struct FixedTweener<T: Tween> {
    tween: T,
    last_time: T::Time,
    delta: T::Time,
    fused: bool,
}

impl<T> FixedTweener<T>
where
    T: Tween,
{
    /// Creates a new [FixedTweener], and takes in the delta time
    /// it will use per tick.
    pub fn new(tween: T, delta: T::Time) -> Self {
        Self {
            tween,
            last_time: T::Time::ZERO,
            delta,
            fused: false,
        }
    }

    // /// Converts this tweener to a [FixedLooper].
    // pub fn looper(self) -> FixedLooper<T, TValue, TTime> {
    //     FixedLooper::new(self)
    // }
}

impl<T> Iterator for FixedTweener<T>
where
    T: Tween,
{
    type Item = T::Value;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.fused {
            self.last_time = self.last_time.add(self.delta);

            if self.last_time.is_complete(self.tween.duration()) {
                self.fused = true;
                Some(*self.tween.range().end())
            } else {
                Some(self.tween.run(self.last_time))
            }
        } else {
            None
        }
    }
}

// /// A [FixedLooper] is a wrapper around a [FixedTweener], which makes it so that
// /// every time the tweener *would* fuse (end), it instead loops.
// pub struct FixedLooper<T, TValue, TTime>(FixedTweener<T, TValue, TTime>);

// impl<T, TValue, TTime> FixedLooper<T, TValue, TTime>
// where
//     T: Tween,
//     TValue: TweenValue,
//     TTime: TweenTime,
// {
//     /// Creates a new FixedLooper. If the tweener is already complete, then it will
//     /// reset it.
//     pub fn new(mut tweener: FixedTweener<T, TValue, TTime>) -> Self {
//         // unfuse it...
//         if tweener.fused {
//             tweener.last_time = TTime::ZERO;
//             tweener.fused = false;
//         }

//         Self(tweener)
//     }
// }

// impl<T, TValue, TTime> Iterator for FixedLooper<T, TValue, TTime>
// where
//     TValue: TweenValue,
//     TTime: TweenTime,
//     T: Tween<Time = TTime, Value = TValue>,
// {
//     type Item = TValue;

//     fn next(&mut self) -> Option<Self::Item> {
//         let output = self.0.next().unwrap(); // we make sure this ALWAYS returns `some`.

//         // catch the fused here...
//         if self.0.fused {
//             self.0.last_time = TTime::ZERO;
//             self.0.fused = false;
//         }

//         Some(output)
//     }
// }

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Linear;

    #[test]
    fn tweener() {
        let tweener = FixedTweener::new(Linear::new(0..=100, 10), 1);
        let values: std::vec::Vec<_> = tweener.collect();

        assert_eq!(*values, [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
    }

    // #[test]
    // fn fixed_tweener_loop() {
    //     let mut looper = FixedTweener::new(Linear::new(0..=2, 2), 1).looper();

    //     assert_eq!(looper.next().unwrap(), 1);
    //     assert_eq!(looper.next().unwrap(), 2);
    //     assert_eq!(looper.next().unwrap(), 1);
    //     assert_eq!(looper.next().unwrap(), 2);
    // }

    // #[test]
    // fn tweener_loop() {
    //     let mut looper = Tweener::new(Linear::new(0..=2, 2)).looper();

    //     assert_eq!(looper.update(1).unwrap(), 1);
    //     assert_eq!(looper.update(1).unwrap(), 2);
    //     assert_eq!(looper.update(1).unwrap(), 1);
    //     assert_eq!(looper.update(1).unwrap(), 2);
    // }
}
