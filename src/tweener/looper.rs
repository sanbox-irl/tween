use super::{FixedTweenDriver, TweenDriver};
use crate::{Tween, TweenTime};

/// A [Looper] is a wrapper around a [Tweener], which makes it so that
/// every time the tweener *would* fuse (end), it loops from the start.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Looper<T: Tween> {
    tween: T,
    position: T::Time,
    duration: T::Time,
}

impl<T: Tween> Looper<T> {
    /// Creates a new Looper around a [Tweener].
    ///
    /// If the [Tweener] is *already* fused, this will reset it to starting
    /// values.
    pub fn new(delta_tweener: TweenDriver<T>) -> Self {
        Self {
            tween: delta_tweener.tween,
            position: delta_tweener.position,
            duration: delta_tweener.duration,
        }
    }

    /// Drives the inner [Tweener] forward X steps in time, looping if required.
    ///
    /// If the delta given is great enough, you may loop around several times.
    pub fn update(&mut self, delta: T::Time) -> T::Value {
        // add in that time...
        self.position = self.position.add(delta).modulo(self.duration);

        if self.position.is_zero() {
            self.tween.final_value()
        } else {
            self.tween.run(self.position)
        }
    }
}

/// A [FixedLooper] is a wrapper around a [FixedTweener], which makes it so that
/// every time the tweener *would* fuse (end), it instead loops.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FixedLooper<T: Tween> {
    tween: T,
    position: T::Time,
    duration: T::Time,
    delta: T::Time,
}

impl<T> FixedLooper<T>
where
    T: Tween,
{
    /// Creates a new FixedLooper. If the tweener is already complete, then it will
    /// reset it.
    pub fn new(tweener: FixedTweenDriver<T>) -> Self {
        Self {
            position: tweener.0.position,
            duration: tweener.0.duration,
            delta: tweener.1,
            tween: tweener.0.tween,
        }
    }
}

impl<T> Iterator for FixedLooper<T>
where
    T: Tween,
{
    type Item = T::Value;

    fn next(&mut self) -> Option<Self::Item> {
        // add in that time...
        self.position = self.position.add(self.delta).modulo(self.duration);
        let o = if self.position.is_zero() {
            self.tween.final_value()
        } else {
            self.tween.run(self.position)
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
