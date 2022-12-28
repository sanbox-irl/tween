use crate::{Tween, TweenValue};

/// An [Oscillator] is a wrapper around a [Tween] which places the Tween into an infinite
/// ping pong.
///
/// This is similar to a [Looper](super::Looper), but instead of restarting the tween at the
/// beginning, it restarts it at the end and travels backwards. For many Tweens in this library,
/// this is the same
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct Oscillator<T: ?Sized>(pub T);

impl<T> Oscillator<T> {
    /// Creates a new Oscillator around a [Tween].
    pub fn new(tween: T) -> Self {
        Self(tween)
    }
}

impl<Value, T> Tween<Value> for Oscillator<T>
where
    Value: TweenValue,
    T: Tween<Value>,
{
    fn tween(&mut self, value_delta: Value, percent: f32) -> Value {
        let temp = percent % 2.0;

        #[cfg(feature = "std")]
        let (which_tween, percent) = { (temp.trunc(), percent.fract()) };

        #[cfg(feature = "libm")]
        let (which_tween, percent) = { (libm::truncf(temp), percent - libm::truncf(percent)) };

        // note: we don't have to worry about 0/1 difference here, since the tween
        // will get us to the same place
        let percent = if which_tween == 0.0 { percent } else { 1.0 - percent };

        self.0.tween(value_delta, percent)
    }

    fn is_finite(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FixedTweener, Linear, Tweener};

    #[test]
    fn div_euclid_fun() {
        assert_eq!(0.0f32 % 2.0, 0.0);
        assert_eq!(0.5f32 % 2.0, 0.5);
        assert_eq!(1.0f32 % 2.0, 1.0);
        assert_eq!(1.5f32 % 2.0, 1.5);
        assert_eq!(2.0f32 % 2.0, 0.0);

        assert_eq!((0.0f32 % 2.0).div_euclid(1.0), 0.0);
        assert_eq!((0.5f32 % 2.0).div_euclid(1.0), 0.0);
        assert_eq!((1.0f32 % 2.0).div_euclid(1.0), 1.0);
        assert_eq!((1.5f32 % 2.0).div_euclid(1.0), 1.0);
        assert_eq!((2.0f32 % 2.0).div_euclid(1.0), 0.0);
        assert_eq!((3.0f32 % 2.0).div_euclid(1.0), 1.0);
        assert_eq!((3.5f32 % 2.0).div_euclid(1.0), 1.0);
        assert_eq!((4.0f32 % 2.0).div_euclid(1.0), 0.0);
        assert_eq!((-0.5f32 % 2.0).div_euclid(1.0), -1.0);

        assert_eq!((0.0f32 % 2.0).div_euclid(1.0), (0.0f32 % 2.0).trunc());
        assert_eq!((0.5f32 % 2.0).div_euclid(1.0), (0.5f32 % 2.0).trunc());
        assert_eq!((1.0f32 % 2.0).div_euclid(1.0), (1.0f32 % 2.0).trunc());
        assert_eq!((1.5f32 % 2.0).div_euclid(1.0), (1.5f32 % 2.0).trunc());
        assert_eq!((2.0f32 % 2.0).div_euclid(1.0), (2.0f32 % 2.0).trunc());
        assert_eq!((3.0f32 % 2.0).div_euclid(1.0), (3.0f32 % 2.0).trunc());
        assert_eq!((3.5f32 % 2.0).div_euclid(1.0), (3.5f32 % 2.0).trunc());
        assert_eq!((4.0f32 % 2.0).div_euclid(1.0), (4.0f32 % 2.0).trunc());
    }

    #[test]
    fn tweener_oscillator() {
        let mut oscillator = Tweener::new(0, 2, 2, Oscillator::new(Linear));

        assert_eq!(oscillator.move_to(0), 0);
        assert_eq!(oscillator.move_to(1), 1);
        assert_eq!(oscillator.move_to(2), 2);
        assert_eq!(oscillator.move_to(3), 1);
        assert_eq!(oscillator.move_to(4), 0);
        assert_eq!(oscillator.move_to(5), 1);
        assert_eq!(oscillator.move_to(6), 2);
    }

    #[test]
    fn delta_tweener_oscillator() {
        let mut oscillator = Tweener::new(0, 2, 2, Oscillator::new(Linear));

        assert_eq!(oscillator.move_by(0), 0);
        assert_eq!(oscillator.move_by(1), 1);
        assert_eq!(oscillator.move_by(1), 2);
        assert_eq!(oscillator.move_by(1), 1);
        assert_eq!(oscillator.move_by(1), 0);
        assert_eq!(oscillator.move_by(1), 1);
        assert_eq!(oscillator.move_by(1), 2);
    }

    #[test]
    fn tweener_oscillator_big_loop() {
        let mut oscillator = Tweener::new(0, 2, 2, Oscillator::new(Linear));

        assert_eq!(oscillator.move_by(2), 2);
        assert_eq!(oscillator.move_by(1), 1);
        assert_eq!(oscillator.move_by(2), 1);
    }

    #[test]
    fn fixed_tweener_oscillator() {
        let mut oscillator = FixedTweener::new(0, 2, 2, Oscillator::new(Linear), 1);

        assert_eq!(oscillator.next().unwrap(), 1);
        assert_eq!(oscillator.next().unwrap(), 2);
        assert_eq!(oscillator.next().unwrap(), 1);
        assert_eq!(oscillator.next().unwrap(), 0);
        assert_eq!(oscillator.next().unwrap(), 1);
        assert_eq!(oscillator.next().unwrap(), 2);
    }

    #[test]
    fn type_test() {
        let _one_type: Oscillator<Linear>;
    }
}
