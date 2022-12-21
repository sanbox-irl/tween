use super::{FixedTweenDriver, TweenDriver};
use crate::{Tween, TweenTime};

/// A [Looper] is a wrapper around a [Tweener], which makes it so that
/// every time the tweener *would* fuse (end), it loops from the start.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Looper<T: Tween>(TweenDriver<T>);

impl<T: Tween> Looper<T> {
    /// Creates a new Looper around a [Tweener].
    ///
    /// If the [Tweener] is *already* fused, this will reset it to starting
    /// values.
    pub fn new(mut delta_tweener: TweenDriver<T>) -> Self {
        // unfuse it...
        if delta_tweener.fused {
            delta_tweener.last_time = T::Time::ZERO;
            delta_tweener.fused = false;
        }

        Self(delta_tweener)
    }

    /// Drives the inner [Tweener] forward X steps in time, looping if required.
    ///
    /// If the delta given is great enough, you may loop around several times.
    pub fn update(&mut self, delta: T::Time) -> Option<T::Value> {
        let output = self.0.update(delta).unwrap(); // we make sure this ALWAYS returns `some`.

        // catch the fused here...
        if self.0.fused {
            self.0.last_time = T::Time::ZERO;
            self.0.fused = false;
        }

        Some(output)
    }
}

/// A [FixedLooper] is a wrapper around a [FixedTweener], which makes it so that
/// every time the tweener *would* fuse (end), it instead loops.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FixedLooper<T: Tween>(FixedTweenDriver<T>);

impl<T> FixedLooper<T>
where
    T: Tween,
{
    /// Creates a new FixedLooper. If the tweener is already complete, then it will
    /// reset it.
    pub fn new(mut tweener: FixedTweenDriver<T>) -> Self {
        // unfuse it...
        if tweener.0.fused {
            tweener.0.last_time = T::Time::ZERO;
            tweener.0.fused = false;
        }

        Self(tweener)
    }
}

impl<T> Iterator for FixedLooper<T>
where
    T: Tween,
{
    type Item = T::Value;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.0.next().unwrap(); // we make sure this ALWAYS returns `some`.

        // catch the fused here...
        if self.0.0.fused {
            self.0.0.last_time = T::Time::ZERO;
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

    #[test]
    fn type_test() {
        let _check: Looper<Linear<i32, i32>>;
    }
}
