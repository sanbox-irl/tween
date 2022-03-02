use std::marker::PhantomData;

use crate::{Tween, TweenTime, TweenValue};

pub struct FixedDeltaTweener<Tween, TValue, TTime> {
    tween: Tween,
    last_time: TTime,
    delta: TTime,
    fused: bool,
    _value: PhantomData<fn(&mut Self, TTime) -> TValue>,
}

impl<T, TValue, TTime> FixedDeltaTweener<T, TValue, TTime>
where
    T: Tween<TValue, TTime>,
    TValue: TweenValue,
    TTime: TweenTime,
{
    pub fn new(tween: T, delta: TTime) -> Self {
        Self {
            tween,
            last_time: TTime::ZERO,
            delta,
            fused: false,

            _value: PhantomData,
        }
    }
}

impl<T, TValue, TTime> Iterator for FixedDeltaTweener<T, TValue, TTime>
where
    T: Tween<TValue, TTime>,
    TValue: TweenValue,
    TTime: TweenTime,
{
    type Item = TValue;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.fused {
            self.last_time = self.last_time.add(self.delta);

            if self.last_time.is_complete(self.tween.duration()) {
                self.fused = true;
                Some(*self.tween.range().end())
            } else {
                Some(self.tween.update(self.last_time))
            }
        } else {
            None
        }
    }
}
