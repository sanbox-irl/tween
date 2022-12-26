use core::marker::PhantomData;

use crate::{Tween, TweenTime, TweenValue};

/// A [Chain] is a wrapper around a Tween which chains two tweens together.
/// When the first tween would end, instead, we move into the second tween.
///
/// NB: [Chain] itself implements [Tween], so you can have a [Chain] of [Chain]s.
/// This is how you make a sequence of more than two tweens.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Chain<First, Second = First> {
    first: First,
    second: Second,
    split: f64,
}

impl<First, Second> Chain<First, Second> {
    /// Creates a new chain with a `split`. This should be a number between `0.0` and `1.0`
    /// after which we use `second` to tween by percentage of the tween.
    ///
    /// A reasonable default is `0.5`.
    pub fn new(first: First, second: Second, split: f64) -> Self {
        Self { first, second, split }
    }
}

impl<Value, Time, First, Second> Tween<Value, Time> for Chain<First, Second>
where
    Value: TweenValue,
    Time: TweenTime,
    First: Tween<Value, Time>,
    Second: Tween<Value, Time>,
{
    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        if percent < self.split {
            self.first.tween(value_delta, percent)
        } else {
            self.second.tween(value_delta, percent)
        }
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use crate::{DeltaTweener, Linear, Looper, Tweener};

    use super::*;

    #[test]
    fn normal() {
        let mut chain = Tweener::new(0, 4, 4, Chain::new(Linear, Linear, 0.5));

        assert_eq!(chain.run(0), 0);
        assert_eq!(chain.run(1), 1);
        assert_eq!(chain.run(2), 2);
        assert_eq!(chain.run(3), 3);
        assert_eq!(chain.run(4), 4);

        let mut chain = DeltaTweener::new(0, 4, 4, Chain::new(Linear, Linear, 0.5));

        assert_eq!(chain.update_by(0), Some(0));
        assert_eq!(chain.update_by(1), Some(1));
        assert_eq!(chain.update_by(1), Some(2));
        assert_eq!(chain.update_by(1), Some(3));
        assert_eq!(chain.update_by(1), Some(4));
        assert_eq!(chain.update_by(1), None);
    }

    #[test]
    fn normal_recursing() {
        let mut chain = DeltaTweener::new(0, 4, 4, Chain::new(Linear, Chain::new(Linear, Linear, 0.5), 0.5));

        assert_eq!(chain.update_by(0).unwrap(), 0);
        assert_eq!(chain.update_by(1).unwrap(), 1);
        assert_eq!(chain.update_by(1).unwrap(), 2);
        assert_eq!(chain.update_by(1).unwrap(), 3);
        assert_eq!(chain.update_by(1).unwrap(), 4);
        assert_eq!(chain.update_by(1), None);
    }

    #[test]
    fn front_recursing() {
        let mut chain = DeltaTweener::new(0, 4, 4, Chain::new(Chain::new(Linear, Linear, 0.5), Linear, 0.5));

        assert_eq!(chain.update_by(0).unwrap(), 0);
        assert_eq!(chain.update_by(1).unwrap(), 1);
        assert_eq!(chain.update_by(1).unwrap(), 2);
        assert_eq!(chain.update_by(1).unwrap(), 3);
        assert_eq!(chain.update_by(1).unwrap(), 4);
        assert_eq!(chain.update_by(1), None);
    }

    #[test]
    fn absurd_loop() {
        let mut chain = DeltaTweener::new(
            2,
            8,
            Chain::new(
                Chain::new(Linear::new(2, 4, 2), Linear::new(5, 6, 2)),
                Chain::new(Linear::new(2, 4, 2), Linear::new(5, 8, 3)),
            ),
        );

        assert_eq!(chain.update(1), 3);
        assert_eq!(chain.update(1), 4);
        assert_eq!(chain.update(1), 5);
        assert_eq!(chain.update(1), 6);
        assert_eq!(chain.update(1), 3);
        assert_eq!(chain.update(1), 4);
        assert_eq!(chain.update(1), 6);
        assert_eq!(chain.update(1), 7);
        assert_eq!(chain.update(1), 8);

        assert_eq!(chain.update(1), 3);
        assert_eq!(chain.update(1), 4);
        assert_eq!(chain.update(1), 5);
        assert_eq!(chain.update(1), 6);
        assert_eq!(chain.update(1), 3);
        assert_eq!(chain.update(1), 4);
        assert_eq!(chain.update(1), 6);
        assert_eq!(chain.update(1), 7);
        assert_eq!(chain.update(1), 8);
    }
}
