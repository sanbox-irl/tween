use crate::{Tween, TweenTime, TweenValue};

/// An [Oscillator] is a wrapper around a [Tween] which places the Tween into an infinite
/// ping pong.
///
/// This is similar to a [Looper](super::Looper), but instead of restarting the tween at the
/// beginning, it restarts it at the end and travels backwards. For many Tweens in this library,
/// this is the same
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Oscillator<T>(T);

impl<T> Oscillator<T> {
    /// Creates a new Oscillator around a [Tween].
    pub fn new(tween: T) -> Self {
        Self(tween)
    }
}

impl<Value, Time, T> Tween<Value, Time> for Oscillator<T>
where
    Value: TweenValue,
    Time: TweenTime,
    T: Tween<Value, Time>,
{
    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        let which_tween = (percent % 2.0).div_euclid(1.0);
        let percent = percent.fract();

        // note: we don't have to worry about 0/1 difference here, since the tween
        // will get us to the same place

        let percent = if which_tween == 0.0 { percent } else { 1.0 - percent };

        self.0.tween(value_delta, percent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeltaTweener, Linear, Tweener};

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
    }

    #[test]
    fn tweener_oscillator() {
        let mut oscillator = Tweener::new(0, 2, 2, Oscillator::new(Linear));

        assert_eq!(oscillator.run(0), 0);
        assert_eq!(oscillator.run(1), 1);
        assert_eq!(oscillator.run(2), 2);
        assert_eq!(oscillator.run(3), 1);
        assert_eq!(oscillator.run(4), 0);
        assert_eq!(oscillator.run(5), 1);
        assert_eq!(oscillator.run(6), 2);
    }

    #[test]
    fn delta_tweener_oscillator() {
        let mut oscillator = DeltaTweener::new(0, 2, 2, Oscillator::new(Linear));

        assert_eq!(oscillator.update_by(0), Some(0));
        assert_eq!(oscillator.update_by(1), Some(1));
        assert_eq!(oscillator.update_by(1), Some(2));
        assert_eq!(oscillator.update_by(1), Some(1));
        assert_eq!(oscillator.update_by(1), Some(0));
        assert_eq!(oscillator.update_by(1), Some(1));
        assert_eq!(oscillator.update_by(1), Some(2));
    }

    // #[test]
    // fn tweener_oscillator_big_loop() {
    //     let mut oscillator = Oscillator::new(TweenDriver::new(Linear::new(0, 2, 2)));

    //     assert_eq!(oscillator.update(2), 2);
    //     assert_eq!(oscillator.update(1), 1);
    //     assert_eq!(oscillator.update(2), 1);
    // }

    // #[test]
    // fn fixed_tweener_oscillator() {
    //     let mut oscillator = FixedOscillator::new(FixedTweenDriver::new(Linear::new(0, 2, 2), 1));

    //     assert_eq!(oscillator.next().unwrap(), 1);
    //     assert_eq!(oscillator.next().unwrap(), 2);
    //     assert_eq!(oscillator.next().unwrap(), 1);
    //     assert_eq!(oscillator.next().unwrap(), 0);
    //     assert_eq!(oscillator.next().unwrap(), 1);
    //     assert_eq!(oscillator.next().unwrap(), 2);
    // }

    #[test]
    fn type_test() {
        // let _one_type: Oscillator<Linear<i32, i32>>;
        // let _two_type: Oscillator<Linear<i32, i32>, crate::QuadIn<i32, i32>>;

        // let conflict: Oscillator<Linear<i32, i32>, crate::QuadIn<u32, i32>>;
    }
}
