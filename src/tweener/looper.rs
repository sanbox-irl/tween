use crate::{Tween, TweenValue};

/// A [Looper] is a wrapper around a [Tween], which makes it so that
/// every time the tweener *would* fuse (end), it loops from the start.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
pub struct Looper<T: ?Sized>(pub T);

impl<T> Looper<T> {
    /// Creates a new Looper around a [Tween].
    pub fn new(tween: T) -> Self {
        Self(tween)
    }
}

impl<Value, T> Tween<Value> for Looper<T>
where
    Value: TweenValue,
    T: Tween<Value>,
{
    fn tween(&mut self, value_delta: Value, mut percent: f64) -> Value {
        if percent == 0.0 {
            return self.0.tween(value_delta, percent);
        }

        percent %= 1.0;
        if percent == 0.0 {
            percent = 1.0
        }

        // pass through to the underlying tween
        self.0.tween(value_delta, percent)
    }

    fn is_finite(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{FixedTweener, Linear, Tweener};

    use super::*;

    #[test]
    fn tweener_loop() {
        let mut looper = Tweener::new(0, 2, 2, Looper::new(Linear));

        assert_eq!(looper.move_to(0), 0);
        assert_eq!(looper.move_to(1), 1);
        assert_eq!(looper.move_to(2), 2);
        assert_eq!(looper.move_to(3), 1);
        assert_eq!(looper.move_to(4), 2);
        assert_eq!(looper.move_to(5), 1);
        assert_eq!(looper.move_to(6), 2);
    }

    #[test]
    fn delta_tweener_loop() {
        let mut looper = Tweener::new(0, 2, 2, Looper::new(Linear));

        assert_eq!(looper.move_by(1), 1);
        assert_eq!(looper.move_by(1), 2);
        assert_eq!(looper.move_by(1), 1);
        assert_eq!(looper.move_by(1), 2);
        assert_eq!(looper.move_by(1), 1);
        assert_eq!(looper.move_by(1), 2);
    }

    #[test]
    fn fixed_delta_tweener_loop() {
        let mut looper = FixedTweener::new(0, 2, 2, Looper::new(Linear), 1);

        assert_eq!(looper.next().unwrap(), 1);
        assert_eq!(looper.next().unwrap(), 2);
        assert_eq!(looper.next().unwrap(), 1);
        assert_eq!(looper.next().unwrap(), 2);
        assert_eq!(looper.next().unwrap(), 1);
        assert_eq!(looper.next().unwrap(), 2);
    }

    #[test]
    fn tweener_loop_frac() {
        let mut looper = Tweener::new(0, 2, 0.5, Looper::new(Linear));

        assert_eq!(looper.move_by(0.25), 1);
        assert_eq!(looper.move_by(0.25), 2);
        assert_eq!(looper.move_by(0.25), 1);
        assert_eq!(looper.move_by(0.25), 2);
        assert_eq!(looper.move_by(0.25), 1);
    }

    #[test]
    fn tweener_big_loop() {
        let mut looper = Tweener::new(0, 2, 2, Looper::new(Linear));

        assert_eq!(looper.move_by(3), 1);
        assert_eq!(looper.move_by(4), 1);
        assert_eq!(looper.move_by(4), 1);
        assert_eq!(looper.move_by(5), 2);
    }

    #[test]
    fn type_test() {
        let mut _looper: FixedTweener<i32, i32, Looper<Linear>> = FixedTweener::new(0, 2, 2, Looper::new(Linear), 2);
    }
}
