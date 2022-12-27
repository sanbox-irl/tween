use crate::{Tween, TweenValue};

/// An [Extrapolator] is a wrapper around a [Tween], which allows a tween to go beyond the range of
/// `0.0` to `1.0`. Note that for [Looper], [Oscillator], or any [Tween] whose [Tween::is_finite]
/// returns false, wrapping in an Extrapolator is unnecssary, as they are already infinite tween.
///
/// As a note of caution, some Tweens, like [Linear](crate::Linear) handle extrapolation just fine,
/// but many others, like [SineIn](crate::SineIn) will give unhelpful results. They are clamped for
/// a reason!
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
pub struct Extrapolator<T: ?Sized>(pub T);

impl<T> Extrapolator<T> {
    /// Creates a new Extrapolator around a [Tween].
    pub fn new(tween: T) -> Self {
        Extrapolator(tween)
    }
}

impl<Value, T> Tween<Value> for Extrapolator<T>
where
    Value: TweenValue,
    T: Tween<Value>,
{
    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
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
        let mut looper = Tweener::new(0, 2, 2, Extrapolator(Linear));

        assert_eq!(looper.move_to(0), 0);
        assert_eq!(looper.move_to(1), 1);
        assert_eq!(looper.move_to(2), 2);
        assert_eq!(looper.move_to(3), 3);
        assert_eq!(looper.move_to(4), 4);
        assert_eq!(looper.move_to(5), 5);
    }

    #[test]
    fn delta_tweener_loop() {
        let mut looper = Tweener::new(0, 2, 2, Extrapolator::new(Linear));

        assert_eq!(looper.move_by(1), 1);
        assert_eq!(looper.move_by(1), 2);
        assert_eq!(looper.move_by(1), 3);
        assert_eq!(looper.move_by(1), 4);
        assert_eq!(looper.move_by(1), 5);
        assert_eq!(looper.move_by(1), 6);
    }

    #[test]
    fn fixed_delta_tweener_loop() {
        let mut looper = FixedTweener::new(0, 2, 2, Extrapolator(Linear), 1);

        assert_eq!(looper.next().unwrap(), 1);
        assert_eq!(looper.next().unwrap(), 2);
        assert_eq!(looper.next().unwrap(), 3);
        assert_eq!(looper.next().unwrap(), 4);
        assert_eq!(looper.next().unwrap(), 5);
        assert_eq!(looper.next().unwrap(), 6);
    }
}
