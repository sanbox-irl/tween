use crate::{Tween, Tweener};

/// A [Chain] is a wrapper around a [Tweener], which makes it so that
/// every time the tweener *would* end, we move onto the next tweener in the sequence.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Chain<I, T>
where
    I: Iterator<Item = Tweener<T>>,
    T: Tween,
{
    chain: I,
    current: Option<Tweener<T>>,
}

impl<I, T> Chain<I, T>
where
    I: Iterator<Item = Tweener<T>>,
    T: Tween,
{
    /// Creates a new Chain around an iterator of [Tweener]s.
    pub fn new<It>(chain: It) -> Self
    where
        It: IntoIterator<Item = Tweener<T>, IntoIter = I>,
    {
        Self {
            chain: chain.into_iter(),
            current: None,
        }
    }

    /// Drives the inner [Tweener] forward X steps in time, moving onto the next tweener if
    /// required.
    ///
    /// If the delta given is great enough, you may move ahead quite a bit.
    pub fn update(&mut self, delta: T::Time) -> Option<T::Value> {
        use crate::TweenTime;

        match self.current.as_mut() {
            Some(current) => {
                let current_time = current.last_time;
                let output = current.update(delta).unwrap();

                if current.fused {
                    let new_delta = current_time.add(delta).sub(current.last_time);
                    self.current = self.chain.next();

                    // we have to check if delta is zero here because we could have non-contiguous
                    // tweens, in which case you'd get to see the final tween
                    if self.current.is_some() && new_delta != T::Time::ZERO {
                        self.update(new_delta)
                    } else {
                        Some(output)
                    }
                } else {
                    Some(output)
                }
            }
            None => {
                self.current = Some(self.chain.next()?);
                println!("trying one more time with current: {:#?}", self.current);
                self.update(delta)
            }
        }
    }
}

// /// A [FixedChain] is a wrapper around a [FixedTweener], which makes it so that
// /// every time the tweener *would* end, we move onto the next tweener in the sequence.
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct FixedChain<I, T>
// where
//     I: Iterator<Item = FixedTweener<T>>,
//     T: Tween,
// {
//     chain: I,
//     current: Option<FixedTweener<T>>,
// }

// impl<I, T> FixedChain<I, T>
// where
//     I: Iterator<Item = FixedTweener<T>>,
//     T: Tween,
// {
//     /// Creates a new Chain around an iterator of [Tweener]s.
//     pub fn new<It>(chain: It) -> Self
//     where
//         It: IntoIterator<Item = FixedTweener<T>, IntoIter = I>,
//     {
//         Self {
//             chain: chain.into_iter(),
//             current: None,
//         }
//     }
// }

// impl<I, T> Iterator for FixedChain<I, T>
// where
//     I: Iterator<Item = FixedTweener<T>>,
//     T: Tween,
// {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         use crate::TweenTime;

//         match self.current.as_mut() {
//             Some(current) => {
//                 let current_time = current.last_time;
//                 let output = current.next().unwrap();

//                 if current.fused {
//                     let new_delta = current_time.add(delta).sub(current.last_time);
//                     self.current = self.chain.next();

//                     // we have to check if delta is zero here because we could have
// non-contiguous                     // tweens, in which case you'd get to see the final tween
//                     if self.current.is_some() && new_delta != T::Time::ZERO {
//                         self.update(new_delta)
//                     } else {
//                         Some(output)
//                     }
//                 } else {
//                     Some(output)
//                 }
//             }
//             None => {
//                 self.current = Some(self.chain.next()?);
//                 println!("trying one more time with current: {:#?}", self.current);
//                 self.update(delta)
//             }
//         }
//     }
// }

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Linear;

    #[test]
    fn normal() {
        let mut looper = Chain::new([
            Tweener::new(Linear::new(0, 2, 2)),
            Tweener::new(Linear::new(2, 4, 2)),
            Tweener::new(Linear::new(6, 8, 2)),
        ]);

        assert_eq!(looper.update(0).unwrap(), 0);
        assert_eq!(looper.update(1).unwrap(), 1);
        assert_eq!(looper.update(1).unwrap(), 2);
        assert_eq!(looper.chain.len(), 1);
        assert_eq!(looper.current.as_ref().unwrap().last_time, 0);
        assert_eq!(looper.update(1).unwrap(), 3);
        assert_eq!(looper.current.as_ref().unwrap().last_time, 1);
        assert_eq!(looper.chain.len(), 1);
        assert_eq!(looper.update(1).unwrap(), 4);
        assert_eq!(looper.update(1).unwrap(), 7);
        assert_eq!(looper.update(1).unwrap(), 8);
        assert_eq!(looper.update(1), None);
    }

    #[test]
    fn too_fast() {
        let mut looper = Chain::new([
            Tweener::new(Linear::new(0, 2, 2)),
            Tweener::new(Linear::new(2, 4, 2)),
            Tweener::new(Linear::new(6, 8, 2)),
        ]);

        assert_eq!(looper.update(3).unwrap(), 3);
        assert_eq!(looper.update(3).unwrap(), 8);
        assert_eq!(looper.update(1), None);
    }

    #[test]
    fn unique() {
        // extremely funky 0 length array!
        let mut looper = Chain::new([Tweener::new(Linear::new(0, 2, 2)); 0]);

        assert_eq!(looper.update(3), None);
    }

    // #[test]
    // fn fixed() {
    //     let mut looper = Tweener::new(Linear::new(0, 2, 2)).looper();

    //     assert_eq!(looper.next().unwrap(), 1);
    //     assert_eq!(looper.next().unwrap(), 2);
    //     assert_eq!(looper.next().unwrap(), 1);
    //     assert_eq!(looper.next().unwrap(), 2);
    // }
}
