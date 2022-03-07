use std::marker::PhantomData;

use crate::{Tween, TweenTime, TweenValue};

pub struct DeltaTweener<Tween, TValue, TTime> {
    tween: Tween,
    last_time: TTime,
    fused: bool,
    _value: PhantomData<fn(&mut Self, TTime) -> TValue>,
}

impl<T, TValue, TTime> DeltaTweener<T, TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
    T: Tween<Time = TTime, Value = TValue>,
{
    pub fn new(tween: T) -> Self {
        Self {
            tween,
            last_time: TTime::ZERO,
            fused: false,

            _value: PhantomData,
        }
    }

    pub fn update(&mut self, delta: TTime) -> Option<TValue> {
        if !self.fused {
            self.last_time = self.last_time.add(delta);

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

    pub fn looper(self) -> Looper<T, TValue, TTime> {
        Looper::new(self)
    }
}

pub struct Looper<T, TValue, TTime>(DeltaTweener<T, TValue, TTime>);

impl<T, TValue, TTime> Looper<T, TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
    T: Tween<Time = TTime, Value = TValue>,
{
    pub fn new(delta_tweener: DeltaTweener<T, TValue, TTime>) -> Self {
        Self(delta_tweener)
    }

    pub fn update(&mut self, delta: TTime) -> Option<TValue> {
        let output = self.0.update(delta).unwrap(); // we make sure this ALWAYS returns `some`.

        // catch the fused here...
        if self.0.fused {
            self.0.last_time = TTime::ZERO;
            self.0.fused = false;
        }

        Some(output)
    }
}

pub struct FixedDeltaTweener<Tween, TValue, TTime> {
    tween: Tween,
    last_time: TTime,
    delta: TTime,
    fused: bool,
    _value: PhantomData<fn(&mut Self, TTime) -> TValue>,
}

impl<T, TValue, TTime> FixedDeltaTweener<T, TValue, TTime>
where
    T: Tween,
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

    pub fn looper(self) -> FixedLooper<T, TValue, TTime> {
        FixedLooper::new(self)
    }
}

impl<T, TValue, TTime> Iterator for FixedDeltaTweener<T, TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
    T: Tween<Time = TTime, Value = TValue>,
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

pub struct FixedLooper<T, TValue, TTime>(FixedDeltaTweener<T, TValue, TTime>);

impl<T, TValue, TTime> FixedLooper<T, TValue, TTime>
where
    T: Tween,
    TValue: TweenValue,
    TTime: TweenTime,
{
    pub fn new(fixed_delta_tweener: FixedDeltaTweener<T, TValue, TTime>) -> Self {
        Self(fixed_delta_tweener)
    }
}

impl<T, TValue, TTime> Iterator for FixedLooper<T, TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
    T: Tween<Time = TTime, Value = TValue>,
{
    type Item = TValue;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.0.next().unwrap(); // we make sure this ALWAYS returns `some`.

        // catch the fused here...
        if self.0.fused {
            self.0.last_time = TTime::ZERO;
            self.0.fused = false;
        }

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Linear;

    #[test]
    fn tweener() {
        let tweener = FixedDeltaTweener::new(Linear::new(0..=100, 10), 1);
        let values: Vec<_> = tweener.collect();

        assert_eq!(*values, [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
    }

    #[test]
    fn fixed_tweener_loop() {
        let mut looper = FixedDeltaTweener::new(Linear::new(0..=2, 2), 1).looper();

        assert_eq!(looper.next().unwrap(), 1);
        assert_eq!(looper.next().unwrap(), 2);
        assert_eq!(looper.next().unwrap(), 1);
        assert_eq!(looper.next().unwrap(), 2);
    }

    #[test]
    fn tweener_loop() {
        let mut looper = DeltaTweener::new(Linear::new(0..=2, 2)).looper();

        assert_eq!(looper.update(1).unwrap(), 1);
        assert_eq!(looper.update(1).unwrap(), 2);
        assert_eq!(looper.update(1).unwrap(), 1);
        assert_eq!(looper.update(1).unwrap(), 2);
    }
}
