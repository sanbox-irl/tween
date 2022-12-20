use super::{FixedTweenDriver, TweenDriver};
use crate::{Tween, TweenTime, TweenValue};

/// A [Looper] is a wrapper around a [Tweener], which makes it so that
/// every time the tweener *would* fuse (end), it loops from the start.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Looper<T, Value, Time>(TweenDriver<T, Value, Time>)
where
    T: Tween<Value, Time>,
    Value: TweenValue,
    Time: TweenTime;

impl<T, Value, Time> Looper<T, Value, Time>
where
    T: Tween<Value, Time>,
    Value: TweenValue,
    Time: TweenTime,
{
    /// Creates a new Looper around a [Tweener].
    ///
    /// If the [Tweener] is *already* fused, this will reset it to starting
    /// values.
    pub fn new(mut delta_tweener: TweenDriver<T, Value, Time>) -> Self {
        // unfuse it...
        if delta_tweener.fused {
            delta_tweener.last_time = Time::ZERO;
            delta_tweener.fused = false;
        }

        Self(delta_tweener)
    }

    /// Drives the inner [Tweener] forward X steps in time, looping if required.
    ///
    /// If the delta given is great enough, you may loop around several times.
    pub fn update(&mut self, delta: Time) -> Option<Value> {
        let output = self.0.update(delta).unwrap(); // we make sure this ALWAYS returns `some`.

        // catch the fused here...
        if self.0.fused {
            self.0.last_time = Time::ZERO;
            self.0.fused = false;
        }

        Some(output)
    }
}

/// A [FixedLooper] is a wrapper around a [FixedTweener], which makes it so that
/// every time the tweener *would* fuse (end), it instead loops.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FixedLooper<T, Value, Time>(FixedTweenDriver<T, Value, Time>)
where
    T: Tween<Value, Time>,
    Value: TweenValue,
    Time: TweenTime;

impl<T, Value, Time> FixedLooper<T, Value, Time>
where
    T: Tween<Value, Time>,
    Value: TweenValue,
    Time: TweenTime,
{
    /// Creates a new FixedLooper. If the tweener is already complete, then it will
    /// reset it.
    pub fn new(mut tweener: FixedTweenDriver<T, Value, Time>) -> Self {
        // unfuse it...
        if tweener.0.fused {
            tweener.0.last_time = Time::ZERO;
            tweener.0.fused = false;
        }

        Self(tweener)
    }
}

impl<T, Value, Time> Iterator for FixedLooper<T, Value, Time>
where
    T: Tween<Value, Time>,
    Time: TweenTime,
    Value: TweenValue,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.0.next().unwrap(); // we make sure this ALWAYS returns `some`.

        // catch the fused here...
        if self.0.0.fused {
            self.0.0.last_time = Time::ZERO;
            self.0.0.fused = false;
        }

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::Linear;

    use super::*;

    #[test]
    fn fixed_tweener_loop() {
        let mut looper = FixedTweenDriver::new(Linear::new(0, 2, 2), 1).looper();

        assert_eq!(looper.next().unwrap(), 1);
        assert_eq!(looper.next().unwrap(), 2);
        assert_eq!(looper.next().unwrap(), 1);
        assert_eq!(looper.next().unwrap(), 2);
    }

    #[test]
    fn tweener_loop() {
        let mut looper = TweenDriver::new(Linear::new(0, 2, 2)).looper();

        assert_eq!(looper.update(1).unwrap(), 1);
        assert_eq!(looper.update(1).unwrap(), 2);
        assert_eq!(looper.update(1).unwrap(), 1);
        assert_eq!(looper.update(1).unwrap(), 2);
    }
}
