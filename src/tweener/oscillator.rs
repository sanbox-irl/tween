use crate::{Tween, TweenTime, TweenValue};

/// An [Oscillator] is a wrapper around a [Tweener], which makes it so that
/// every time the tweener *would* fuse (end), it instead starts reversing back to the start.
///
/// You will always get an end edge on both ends for a tick.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Oscillator<Rising, Falling = Rising> {
    rising: Rising,
    falling: Falling,
}

// impl<Rising> Oscillator<Rising>
// where
//     Rising: crate::SizedTween,
// {
//     /// Creates a new Oscillator. If the tweener is already complete, then it will
//     /// reset it, and creates a backwards copy of the tween.
//     ///
//     /// The tween given will be assigned as the `rising` tween, whereas the generated inverse
// will     /// be the `falling` tween.
//     pub fn new(rising: TweenDriver<Rising>) -> Self {
//         let falling = TweenData::new(Rising::new(
//             rising.tween_data.tween.final_value(),
//             rising.tween_data.tween.initial_value(),
//             rising.tween_data.tween.duration(),
//         ));

//         let position = rising.tween_data.position;
//         let duration = rising.tween_data.duration + falling.duration;

//         Self {
//             rising: rising.tween_data,
//             falling,
//             position,
//             total_duration: duration,
//             direction: OscillationDirection::Rising,
//         }
//     }
// }

impl<Rising, Falling> Oscillator<Rising, Falling> {
    /// Creates a new FixedOscillator out of a `falling` and `rising` tween. If either tweener is
    /// complete, then they will be reset.
    ///
    /// Because an arbitrary rising and falling tween are given, you can create piece-wise tweens.
    pub fn with_falling(rising: Rising, falling: Falling) -> Self {
        Self { rising, falling }
    }

    // /// Drives the inner [Tweener] forward X steps in time, oscillating if required.
    // ///
    // /// If the delta given is great enough, you may oscillate around several times.
    // pub fn update(&mut self, delta: Rising::Time) -> Rising::Value {
    //     self.position = (self.position + delta) % self.total_duration;

    //     if self.position == Rising::Time::ZERO {
    //         self.direction = OscillationDirection::Falling;
    //         self.falling.tween.final_value()
    //     } else if self.position.eq(&self.rising.duration) {
    //         self.direction = OscillationDirection::Rising;
    //         self.rising.tween.final_value()
    //     } else if self.position >= self.rising.duration {
    //         self.direction = OscillationDirection::Falling;
    //         self.falling.tween.run(self.position - self.rising.duration)
    //     } else {
    //         self.direction = OscillationDirection::Rising;
    //         self.rising.tween.run(self.position)
    //     }
    // }
}

impl<Value, Time, Rising, Falling> Tween<Value, Time> for Oscillator<Rising, Falling>
where
    Value: TweenValue,
    Time: TweenTime,
    Rising: Tween<Value, Time>,
    Falling: Tween<Value, Time>,
{
    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        if percent > 1.0 {
            self.falling.tween(value_delta, percent)
        } else {
            self.rising.tween(value_delta, percent)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeltaTweener, Linear};

    #[test]
    fn tweener_oscillator() {
        let mut oscillator = DeltaTweener::new(0, 2, 2, Oscillator::with_falling(Linear, Linear));

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
