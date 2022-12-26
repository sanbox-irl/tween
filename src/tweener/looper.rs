use crate::{Tween, TweenTime, TweenValue};

/// A [Looper] is a wrapper around a [Tweener], which makes it so that
/// every time the tweener *would* fuse (end), it loops from the start.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Looper<T>(T);

impl<T> Looper<T> {
    /// Creates a new Looper around a [Tweener].
    pub fn new(tween: T) -> Self {
        Self(tween)
    }
}

impl<Value, Time, T> Tween<Value, Time> for Looper<T>
where
    Value: TweenValue,
    Time: TweenTime,
    T: Tween<Value, Time>,
{
    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        // pass through to the underlying tween
        self.0.tween(value_delta, percent)
    }

    // this is the looping work
    fn percent(&mut self, current_time: Time, duration: Time) -> f64 {
        if current_time == Time::ZERO {
            return 0.0;
        }

        let mod_val = current_time % duration;
        if mod_val == Time::ZERO {
            1.0
        } else {
            self.0.percent(mod_val, duration)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{DeltaTweener, FixedTweener, Linear, Tweener};

    use super::*;

    #[test]
    fn tweener_loop() {
        let mut looper = Tweener::new(0, 2, 2, Looper::new(Linear));

        assert_eq!(looper.run(0), 0);
        assert_eq!(looper.run(1), 1);
        assert_eq!(looper.run(2), 2);
        assert_eq!(looper.run(3), 1);
        assert_eq!(looper.run(4), 2);
        assert_eq!(looper.run(5), 1);
        assert_eq!(looper.run(6), 2);
    }

    #[test]
    fn delta_tweener_loop() {
        let mut looper = DeltaTweener::new(0, 2, 2, Looper::new(Linear));

        assert_eq!(looper.update_by(1).unwrap(), 1);
        assert_eq!(looper.update_by(1).unwrap(), 2);
        assert_eq!(looper.update_by(1).unwrap(), 1);
        assert_eq!(looper.update_by(1).unwrap(), 2);
        assert_eq!(looper.update_by(1).unwrap(), 1);
        assert_eq!(looper.update_by(1).unwrap(), 2);
    }

    #[test]
    fn fixed_delta_tweener_loop() {
        let mut looper = FixedTweener::new(0, 2, 2, 1, Looper::new(Linear));

        assert_eq!(looper.next().unwrap(), 1);
        assert_eq!(looper.next().unwrap(), 2);
        assert_eq!(looper.next().unwrap(), 1);
        assert_eq!(looper.next().unwrap(), 2);
        assert_eq!(looper.next().unwrap(), 1);
        assert_eq!(looper.next().unwrap(), 2);
    }

    #[test]
    fn tweener_loop_frac() {
        let mut looper = DeltaTweener::new(0, 2, 0.5, Looper::new(Linear));

        assert_eq!(looper.update_by(0.25).unwrap(), 1);
        assert_eq!(looper.update_by(0.25).unwrap(), 2);
        assert_eq!(looper.update_by(0.25).unwrap(), 1);
        assert_eq!(looper.update_by(0.25).unwrap(), 2);
        assert_eq!(looper.update_by(0.25).unwrap(), 1);
    }

    #[test]
    fn tweener_big_loop() {
        let mut looper = DeltaTweener::new(0, 2, 2, Looper::new(Linear));

        assert_eq!(looper.update_by(3).unwrap(), 1);
        assert_eq!(looper.update_by(4).unwrap(), 1);
        assert_eq!(looper.update_by(4).unwrap(), 1);
        assert_eq!(looper.update_by(5).unwrap(), 2);
    }

    #[test]
    fn type_test() {
        let mut _looper: DeltaTweener<i32, i32, Looper<Linear>> = DeltaTweener::new(0, 2, 2, Looper::new(Linear));
    }
}
