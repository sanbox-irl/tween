use super::{FixedTweenDriver, TweenDriver};
use crate::{Tween, TweenData, TweenTime};

/// A [Looper] is a wrapper around a [Tweener], which makes it so that
/// every time the tweener *would* fuse (end), it loops from the start.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Looper<T>(TweenData<T>)
where
    T: Tween;

impl<T: Tween> Looper<T> {
    /// Creates a new Looper around a [Tweener].
    ///
    /// If the [Tweener] is *already* fused, this will reset it to starting
    /// values.
    pub fn new(delta_tweener: TweenDriver<T>) -> Self {
        Self(delta_tweener.tween_data)
    }

    /// Drives the inner [Tweener] forward X steps in time, looping if required.
    ///
    /// If the delta given is great enough, you may loop around several times.
    pub fn update(&mut self, delta: T::Time) -> T::Value {
        // add in that time...
        self.0.position = self.0.position.add(delta).modulo(self.0.duration);

        if self.0.position.is_zero() {
            self.0.tween.final_value()
        } else {
            self.0.tween.run(self.0.position)
        }
    }
}

/// A [FixedLooper] is a wrapper around a [FixedTweener], which makes it so that
/// every time the tweener *would* fuse (end), it instead loops.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FixedLooper<T>(TweenData<T>, T::Time)
where
    T: Tween;

impl<T> FixedLooper<T>
where
    T: Tween,
{
    /// Creates a new FixedLooper. If the tweener is already complete, then it will
    /// reset it.
    pub fn new(tweener: FixedTweenDriver<T>) -> Self {
        Self(tweener.tween_data, tweener.delta)
    }
}

impl<T> Iterator for FixedLooper<T>
where
    T: Tween,
{
    type Item = T::Value;

    fn next(&mut self) -> Option<Self::Item> {
        // add in that time...
        self.0.position = self.0.position.add(self.1).modulo(self.0.duration);
        let o = if self.0.position.is_zero() {
            self.0.tween.final_value()
        } else {
            self.0.tween.run(self.0.position)
        };

        Some(o)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
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

        assert_eq!(looper.update(1), 1);
        assert_eq!(looper.update(1), 2);
        assert_eq!(looper.update(1), 1);
        assert_eq!(looper.update(1), 2);
        assert_eq!(looper.update(1), 1);
    }

    #[test]
    fn tweener_loop_frac() {
        let mut looper = TweenDriver::new(Linear::new(0, 2, 0.5)).looper();

        assert_eq!(looper.update(0.25), 1);
        assert_eq!(looper.update(0.25), 2);
        assert_eq!(looper.update(0.25), 1);
        assert_eq!(looper.update(0.25), 2);
        assert_eq!(looper.update(0.25), 1);
    }

    #[test]
    fn tweener_big_loop() {
        let mut looper = TweenDriver::new(Linear::new(0, 2, 2)).looper();

        assert_eq!(looper.update(3), 1);
        assert_eq!(looper.update(4), 1);
        assert_eq!(looper.update(4), 1);
        assert_eq!(looper.update(5), 2);
    }

    #[test]
    fn type_test() {
        let _check: Looper<Linear<i32, i32>>;
    }
}
