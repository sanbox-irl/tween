use crate::{Tween, TweenTime};

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
    /// Creates a new chain directly.
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
        self.position = self.position.add(delta);

        if First::Time::percent(self.durations[0], self.position) > 1.0 {
            let chunked_pos = self.position.sub(self.durations[0]);

            if chunked_pos.is_complete(self.durations[1]) {
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
            let chunked_pos = self.position.sub(self.durations[0]);

            if chunked_pos.is_complete(self.durations[1]) {
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
        self.durations[0].add(self.durations[1])
    }
}
#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use crate::{Linear, Looper, TweenDriver};

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

        // assert_eq!(chain.update(0), 2);
        assert_eq!(chain.update(1), 3);
        assert_eq!(chain.update(1), 4);
        assert_eq!(chain.update(1), 5);
        assert_eq!(chain.update(1), 6);
        assert_eq!(chain.update(1), 3);
        assert_eq!(chain.update(1), 4);
        assert_eq!(chain.update(1), 6);
        assert_eq!(chain.update(1), 7);
        assert_eq!(chain.update(1), 8);

        // assert_eq!(chain.update(0), 2);
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

    // #[test]
    // fn too_fast() {
    //     let mut looper = Chain::new([
    //         TweenDriver::new(Linear::new(0, 2, 2)),
    //         TweenDriver::new(Linear::new(2, 4, 2)),
    //         TweenDriver::new(Linear::new(6, 8, 2)),
    //     ]);

    //     assert_eq!(looper.update(3).unwrap(), 3);
    //     assert_eq!(looper.update(3).unwrap(), 8);
    //     assert_eq!(looper.update(1), None);
    // }

    // #[test]
    // fn unique() {
    //     // extremely funky 0 length array!
    //     let empty_array: [TweenDriver<_>; 0] = [TweenDriver::new(Linear::new(0, 2, 2)); 0];
    //     let mut looper = Chain::new(empty_array);

    //     assert_eq!(looper.update(3), None);
    // }

    // #[test]
    // fn fixed() {
    //     Chain::new([
    //         TweenDriver::new(Linear::new(0, 2, 2)),
    //         TweenDriver::new(QuadIn::new(2, 4, 2)),
    //     ]);
    // }
}
