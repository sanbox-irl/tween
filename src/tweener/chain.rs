use crate::{Tween, TweenTime};

/// A [Chain] is a wrapper around a Tween which chains two tweens together.
/// When the first tween would end, instead, we move into the second tween.
///
/// NB: [Chain]  implements [Tween], so you can have a [Chain] of [Chain]s.
/// This is how you make a sequence of more than two tweens.
///
/// As always, see [FixedChain] for a fixed-delta variant.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Chain<First, Second = First>
where
    First: Tween<Value = Second::Value, Time = Second::Time>,
    Second: Tween,
{
    position: First::Time,
    durations: [First::Time; 2],
    first: First,
    second: Second,
    fused: bool,
}

impl<First, Second> Chain<First, Second>
where
    First: Tween<Value = Second::Value, Time = Second::Time>,
    Second: Tween,
{
    /// Creates a new chain.
    pub fn new(first: First, second: Second) -> Self {
        Self {
            position: First::Time::ZERO,
            durations: [first.duration(), second.duration()],
            first,
            second,
            fused: false,
        }
    }

    /// Drives the inner [Tweener] forward X steps in time.
    ///
    /// If the delta given is great enough, you may loop around several times.
    pub fn update(&mut self, delta: First::Time) -> Option<First::Value> {
        if self.fused {
            return None;
        }

        // add in that time...
        self.position += delta;

        if First::Time::percent(self.durations[0], self.position) > 1.0 {
            let chunked_pos = self.position - self.durations[0];

            if chunked_pos >= self.durations[1] {
                self.fused = true;

                Some(self.second.final_value())
            } else {
                Some(self.second.run(chunked_pos))
            }
        } else {
            Some(self.first.run(self.position))
        }
    }
}

impl<First, Second> Tween for Chain<First, Second>
where
    First: Tween<Value = Second::Value, Time = Second::Time>,
    Second: Tween,
{
    type Value = First::Value;
    type Time = First::Time;

    fn run(&mut self, new_time: Self::Time) -> Self::Value {
        self.position = new_time;

        if First::Time::percent(self.durations[0], self.position) > 1.0 {
            let chunked_pos = self.position - self.durations[0];

            if chunked_pos >= self.durations[1] {
                self.second.final_value()
            } else {
                self.second.run(chunked_pos)
            }
        } else {
            self.first.run(self.position)
        }
    }

    fn initial_value(&self) -> Self::Value {
        self.first.initial_value()
    }

    fn final_value(&self) -> Self::Value {
        self.second.final_value()
    }

    fn duration(&self) -> Self::Time {
        self.durations[0] + self.durations[1]
    }
}

/// A [FixedChain] is a wrapper around a Tween which chains two tweens together.
/// When the first tween would end, instead, we move into the second tween.
///
/// NB: [FixedChain]  implements [Tween], so you can have a [FixedChain] of [FixedChain]s.
/// This is how you make a sequence of more than two tweens.
///
/// As always, see [Chain] for a variadic-delta variant.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct FixedChain<First, Second = First>
where
    First: Tween<Value = Second::Value, Time = Second::Time>,
    Second: Tween,
{
    position: First::Time,
    durations: [First::Time; 2],
    deltas: [First::Time; 2],
    first: First,
    second: Second,
    fused: bool,
}

impl<First, Second> FixedChain<First, Second>
where
    First: Tween<Value = Second::Value, Time = Second::Time>,
    Second: Tween,
{
    /// Creates a new fixed chain.
    pub fn new(first: First, first_fixed_delta: First::Time, second: Second, second_fixed_delta: First::Time) -> Self {
        Self {
            position: First::Time::ZERO,
            durations: [first.duration(), second.duration()],
            deltas: [first_fixed_delta, second_fixed_delta],
            first,
            second,
            fused: false,
        }
    }
}

impl<First, Second> Tween for FixedChain<First, Second>
where
    First: Tween<Value = Second::Value, Time = Second::Time>,
    Second: Tween,
{
    type Value = First::Value;
    type Time = First::Time;

    fn run(&mut self, new_time: Self::Time) -> Self::Value {
        self.position = new_time;

        if First::Time::percent(self.durations[0], self.position) > 1.0 {
            let chunked_pos = self.position - self.durations[0];

            if chunked_pos >= self.durations[1] {
                self.second.final_value()
            } else {
                self.second.run(chunked_pos)
            }
        } else {
            self.first.run(self.position)
        }
    }

    fn initial_value(&self) -> Self::Value {
        self.first.initial_value()
    }

    fn final_value(&self) -> Self::Value {
        self.second.final_value()
    }

    fn duration(&self) -> Self::Time {
        self.durations[0] + self.durations[1]
    }
}

impl<First, Second> Iterator for FixedChain<First, Second>
where
    First: Tween<Value = Second::Value, Time = Second::Time>,
    Second: Tween,
{
    type Item = First::Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fused {
            return None;
        }

        let delta = if First::Time::percent(self.durations[0], self.position) > 1.0 {
            self.deltas[0]
        } else {
            self.deltas[1]
        };

        // add in that time...
        self.position += delta;

        if First::Time::percent(self.durations[0], self.position) > 1.0 {
            let chunked_pos = self.position - self.durations[0];

            if chunked_pos >= self.durations[1] {
                self.fused = true;

                Some(self.second.final_value())
            } else {
                Some(self.second.run(chunked_pos))
            }
        } else {
            Some(self.first.run(self.position))
        }
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use crate::{FixedLooper, FixedTweenDriver, Linear, Looper, TweenDriver};

    use super::*;

    #[test]
    fn normal() {
        let mut chain = Chain::new(Linear::new(0, 2, 2), Linear::new(2, 4, 2));

        assert_eq!(chain.update(0).unwrap(), 0);
        assert_eq!(chain.update(1).unwrap(), 1);
        assert_eq!(chain.update(1).unwrap(), 2);
        assert_eq!(chain.update(1).unwrap(), 3);
        assert_eq!(chain.update(1).unwrap(), 4);
        assert_eq!(chain.update(1), None);
    }

    #[test]
    fn normal_recursing() {
        let mut chain = Chain::new(
            Linear::new(0, 2, 2),
            Chain::new(Linear::new(2, 4, 2), Linear::new(4, 6, 2)),
        );

        assert_eq!(chain.update(0).unwrap(), 0);
        assert_eq!(chain.update(1).unwrap(), 1);
        assert_eq!(chain.update(1).unwrap(), 2);
        assert_eq!(chain.update(1).unwrap(), 3);
        assert_eq!(chain.update(1).unwrap(), 4);
        assert_eq!(chain.update(1).unwrap(), 5);
        assert_eq!(chain.update(1).unwrap(), 6);
        assert_eq!(chain.update(1), None);
    }

    #[test]
    fn front_recursing() {
        let mut chain = Chain::new(
            Chain::new(Linear::new(2, 4, 2), Linear::new(5, 6, 2)),
            Chain::new(Linear::new(2, 4, 2), Linear::new(5, 6, 2)),
        );

        assert_eq!(chain.update(0).unwrap(), 2);
        assert_eq!(chain.update(1).unwrap(), 3);
        assert_eq!(chain.update(1).unwrap(), 4);
        assert_eq!(chain.update(1).unwrap(), 5);
        assert_eq!(chain.update(1).unwrap(), 6);
        assert_eq!(chain.update(1).unwrap(), 3);
        assert_eq!(chain.update(1).unwrap(), 4);
        assert_eq!(chain.update(1).unwrap(), 5);
        assert_eq!(chain.update(1).unwrap(), 6);
        assert_eq!(chain.update(1), None);
    }

    #[test]
    fn absurd_loop() {
        let mut chain = Looper::new(TweenDriver::new(Chain::new(
            Chain::new(Linear::new(2, 4, 2), Linear::new(5, 6, 2)),
            Chain::new(Linear::new(2, 4, 2), Linear::new(5, 8, 3)),
        )));

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

    #[test]
    fn normal_fixed() {
        let mut chain = FixedChain::new(Linear::new(0, 2, 2), 1, Linear::new(2, 4, 2), 1);

        assert_eq!(chain.next().unwrap(), 1);
        assert_eq!(chain.next().unwrap(), 2);
        assert_eq!(chain.next().unwrap(), 3);
        assert_eq!(chain.next().unwrap(), 4);
        assert_eq!(chain.next(), None);
    }

    #[test]
    fn normal_recursing_fixed() {
        let mut chain = FixedChain::new(
            Linear::new(0, 2, 2),
            1,
            Chain::new(Linear::new(2, 4, 2), Linear::new(4, 6, 2)),
            1,
        );

        assert_eq!(chain.next().unwrap(), 1);
        assert_eq!(chain.next().unwrap(), 2);
        assert_eq!(chain.next().unwrap(), 3);
        assert_eq!(chain.next().unwrap(), 4);
        assert_eq!(chain.next().unwrap(), 5);
        assert_eq!(chain.next().unwrap(), 6);
    }

    #[test]
    fn front_recursing_fixed() {
        let mut chain = FixedChain::new(
            Chain::new(Linear::new(2, 4, 2), Linear::new(5, 6, 2)),
            1,
            Chain::new(Linear::new(2, 4, 2), Linear::new(5, 6, 2)),
            1,
        );

        assert_eq!(chain.next().unwrap(), 3);
        assert_eq!(chain.next().unwrap(), 4);
        assert_eq!(chain.next().unwrap(), 5);
        assert_eq!(chain.next().unwrap(), 6);
        assert_eq!(chain.next().unwrap(), 3);
        assert_eq!(chain.next().unwrap(), 4);
        assert_eq!(chain.next().unwrap(), 5);
        assert_eq!(chain.next().unwrap(), 6);
    }

    #[test]
    fn absurd_loop_fixed() {
        let mut chain = FixedLooper::new(FixedTweenDriver::new(
            FixedChain::new(
                Chain::new(Linear::new(2, 4, 2), Linear::new(5, 6, 2)),
                1,
                Chain::new(Linear::new(2, 4, 2), Linear::new(5, 8, 3)),
                1,
            ),
            1,
        ));

        assert_eq!(chain.next().unwrap(), 3);
        assert_eq!(chain.next().unwrap(), 4);
        assert_eq!(chain.next().unwrap(), 5);
        assert_eq!(chain.next().unwrap(), 6);
        assert_eq!(chain.next().unwrap(), 3);
        assert_eq!(chain.next().unwrap(), 4);
        assert_eq!(chain.next().unwrap(), 6);
        assert_eq!(chain.next().unwrap(), 7);
        assert_eq!(chain.next().unwrap(), 8);

        assert_eq!(chain.next().unwrap(), 3);
        assert_eq!(chain.next().unwrap(), 4);
        assert_eq!(chain.next().unwrap(), 5);
        assert_eq!(chain.next().unwrap(), 6);
        assert_eq!(chain.next().unwrap(), 3);
        assert_eq!(chain.next().unwrap(), 4);
        assert_eq!(chain.next().unwrap(), 6);
        assert_eq!(chain.next().unwrap(), 7);
        assert_eq!(chain.next().unwrap(), 8);
    }
}
