use super::{FixedTweener, Tweener};
use crate::{Tween, TweenTime};

/// A [Looper] is a wrapper around a [Tweener], which makes it so that
/// every time the tweener *would* fuse (end), it loops from the start.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Looper<T: Tween>(Tweener<T>);

impl<T> Looper<T>
where
    T: Tween,
{
    /// Creates a new Looper around a [Tweener].
    ///
    /// If the [Tweener] is *already* fused, this will reset it to starting
    /// values.
    pub fn new(mut delta_tweener: Tweener<T>) -> Self {
        // unfuse it...
        if delta_tweener.fused {
            delta_tweener.last_time = T::Time::ZERO;
            delta_tweener.fused = false;
        }

        Self(delta_tweener)
    }

    /// Drives the inner [Tweener] forward X steps in time, looping if required.
    ///
    /// If the delta given is great enough, you may loop around several times.
    pub fn update(&mut self, delta: T::Time) -> Option<T::Value> {
        let output = self.0.update(delta).unwrap(); // we make sure this ALWAYS returns `some`.

        // catch the fused here...
        if self.0.fused {
            self.0.last_time = T::Time::ZERO;
            self.0.fused = false;
        }

        Some(output)
    }
}

/// A [FixedLooper] is a wrapper around a [FixedTweener], which makes it so that
/// every time the tweener *would* fuse (end), it instead loops.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FixedLooper<T: Tween>(FixedTweener<T>);

impl<T> FixedLooper<T>
where
    T: Tween,
{
    /// Creates a new FixedLooper. If the tweener is already complete, then it will
    /// reset it.
    pub fn new(mut tweener: FixedTweener<T>) -> Self {
        // unfuse it...
        if tweener.fused {
            tweener.last_time = T::Time::ZERO;
            tweener.fused = false;
        }

        Self(tweener)
    }
}

impl<T> Iterator for FixedLooper<T>
where
    T: Tween,
{
    type Item = T::Value;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.0.next().unwrap(); // we make sure this ALWAYS returns `some`.

        // catch the fused here...
        if self.0.fused {
            self.0.last_time = T::Time::ZERO;
            self.0.fused = false;
        }

        Some(output)
    }
}
