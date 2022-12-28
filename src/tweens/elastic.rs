use crate::{Tween, TweenTime, TweenValue};
use core::{f32::consts::PI, marker::PhantomData};

/// An elastic tween in. Go [here](https://easings.net/#easeInElastic) for a visual demonstration.
#[derive(Debug, PartialEq, Clone)]
pub struct ElasticIn<Value, Time> {
    duration: Time,
    three_tenths: f32,
    s: f32,
    _value: PhantomData<Value>,
}

impl<Value, Time> ElasticIn<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    /// Creates a new tween out of a range with a duration.
    pub fn new(duration: Time) -> Self {
        let three_tenths = duration.to_f32() * 0.3;
        Self {
            duration,
            three_tenths,
            s: three_tenths * 0.25,
            _value: PhantomData,
        }
    }
}

impl<Value, Time> Tween<Value> for ElasticIn<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    fn tween(&mut self, value_delta: Value, mut percent: f32) -> Value {
        if percent == 0.0 {
            return value_delta.scale(0.0);
        }

        if percent == 1.0 {
            return value_delta;
        }

        percent -= 1.0;

        #[cfg(feature = "libm")]
        let scalar = libm::powf(2.0, percent * 10.0);

        #[cfg(feature = "std")]
        let scalar = 2f32.powf(percent * 10.0);

        let post_fix = value_delta.scale(scalar);
        let temp = (self.duration.to_f32() * percent - self.s) * (2.0 * PI) / self.three_tenths;

        #[cfg(feature = "libm")]
        let scalar = -libm::sinf(temp);

        #[cfg(feature = "std")]
        let scalar = -temp.sin();

        post_fix.scale(scalar)
    }
}

impl<Value, Time> crate::Tweener<Value, Time, ElasticIn<Value, Time>>
where
    Time: TweenTime,
    Value: TweenValue,
{
    /// Creates a new [BounceInOut] tween.
    pub fn elastic_in(start: Value, end: Value, duration: Time) -> crate::Tweener<Value, Time, ElasticIn<Value, Time>> {
        crate::Tweener::new(start, end, duration, ElasticIn::new(duration))
    }
}

/// An elastic tween out. Go [here](https://easings.net/#easeOutElastic) for a visual demonstration.
#[derive(Debug, PartialEq, Clone)]
pub struct ElasticOut<Value, Time> {
    duration: Time,
    three_tenths: f32,
    s: f32,
    _value: PhantomData<Value>,
}

impl<Value, Time> ElasticOut<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    /// Creates a new tween out of a range with a duration.
    pub fn new(duration: Time) -> Self {
        let three_tenths = duration.to_f32() * 0.3;
        Self {
            duration,
            three_tenths,
            s: three_tenths * 0.25,
            _value: PhantomData,
        }
    }
}

impl<Value, Time> Tween<Value> for ElasticOut<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    fn tween(&mut self, value_delta: Value, percent: f32) -> Value {
        if percent == 0.0 {
            return value_delta.scale(0.0);
        }

        if percent == 1.0 {
            return value_delta;
        }

        let temp = (percent * self.duration.to_f32() - self.s) * (2.0 * PI) / self.three_tenths;

        #[cfg(feature = "libm")]
        let scalar = libm::powf(2.0, -10.0 * percent) * libm::sinf(temp);

        #[cfg(feature = "std")]
        let scalar = 2f32.powf(-10.0 * percent) * temp.sin();

        value_delta.scale(scalar) + value_delta
    }
}

impl<Value, Time> crate::Tweener<Value, Time, ElasticOut<Value, Time>>
where
    Time: TweenTime,
    Value: TweenValue,
{
    /// Creates a new [BounceInOut] tween.
    pub fn elastic_out(
        start: Value,
        end: Value,
        duration: Time,
    ) -> crate::Tweener<Value, Time, ElasticOut<Value, Time>> {
        crate::Tweener::new(start, end, duration, ElasticOut::new(duration))
    }
}

/// An elastic tween in and out. Go [here](https://easings.net/#easeInOutElastic) for a visual demonstration.
#[derive(Debug, PartialEq, Clone)]
pub struct ElasticInOut<Value, Time> {
    duration: Time,
    p: f32,
    s: f32,
    _value: PhantomData<Value>,
}

impl<Value, Time> ElasticInOut<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    /// Creates a new tween out of a range with a duration.
    pub fn new(duration: Time) -> Self {
        let p = duration.to_f32() * 0.45;

        Self {
            duration,
            s: p * 0.25,
            p,
            _value: PhantomData,
        }
    }
}

impl<Value, Time> Tween<Value> for ElasticInOut<Value, Time>
where
    Value: TweenValue,
    Time: TweenTime,
{
    fn tween(&mut self, value_delta: Value, mut percent: f32) -> Value {
        if percent == 0.0 {
            return value_delta.scale(0.0);
        }

        if percent == 1.0 {
            return value_delta;
        }

        percent = (percent * 2.0) - 1.0;

        if percent < 0.0 {
            #[cfg(feature = "libm")]
            let scalar = libm::powf(2.0, percent * 10.0);

            #[cfg(feature = "std")]
            let scalar = 2f32.powf(percent * 10.0);

            let post_fix = value_delta.scale(scalar);
            let temp = (self.duration.to_f32() * percent - self.s) * (2.0 * PI) / self.p;

            #[cfg(feature = "libm")]
            let temp_sin = libm::sinf(temp);

            #[cfg(feature = "std")]
            let temp_sin = temp.sin();

            post_fix.scale(-0.5 * temp_sin)
        } else {
            #[cfg(feature = "libm")]
            let scalar = libm::powf(2.0, percent * -10.0);

            #[cfg(feature = "std")]
            let scalar = 2f32.powf(-10.0 * percent);

            let post_fix = value_delta.scale(scalar);
            let temp = (self.duration.to_f32() * percent - self.s) * (2.0 * PI) / self.p;

            #[cfg(feature = "libm")]
            let temp_sin = libm::sinf(temp);

            #[cfg(feature = "std")]
            let temp_sin = temp.sin();

            post_fix.scale(temp_sin * 0.5) + value_delta
        }
    }
}

impl<Value, Time> crate::Tweener<Value, Time, ElasticInOut<Value, Time>>
where
    Time: TweenTime,
    Value: TweenValue,
{
    /// Creates a new [BounceInOut] tween.
    pub fn elastic_in_out(
        start: Value,
        end: Value,
        duration: Time,
    ) -> crate::Tweener<Value, Time, ElasticInOut<Value, Time>> {
        crate::Tweener::new(start, end, duration, ElasticInOut::new(duration))
    }
}

#[cfg(test)]
mod tests {
    use crate::Tweener;

    use super::*;
    use approx::assert_relative_eq;
    use easer::functions::{Easing, Elastic};

    #[test]
    fn t_in() {
        let mut tweener = Tweener::new(0.0, 100.0, 10.0, ElasticIn::new(10.0));

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.move_to(time);
            let o = Elastic::ease_in(time, 0.0, 100.0, 10.0);

            assert_relative_eq!(v, o, max_relative = 0.000001);
        }
    }

    #[test]
    fn t_in_rev() {
        let mut tweener = Tweener::new(100.0, 0.0, 10.0, ElasticIn::new(10.0));

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.move_to(time);
            let o = Elastic::ease_in(time, 100.0, -100.0, 10.0);

            assert_relative_eq!(v, o, max_relative = 0.000001);
        }
    }

    #[test]
    fn t_out() {
        let mut tweener = Tweener::new(0.0, 100.0, 10.0, ElasticOut::new(10.0));

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.move_to(time);
            let o = Elastic::ease_out(time, 0.0, 100.0, 10.0);

            assert_relative_eq!(v, o, max_relative = 0.000001);
        }
    }

    #[test]
    fn t_out_rev() {
        let mut tweener = Tweener::new(100.0, 0.0, 10.0, ElasticOut::new(10.0));

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.move_to(time);
            let o = Elastic::ease_out(time, 100.0, -100.0, 10.0);

            assert_relative_eq!(v, o, max_relative = 0.000001);
        }
    }

    #[test]
    fn t_in_out() {
        let mut tweener = Tweener::new(0.0, 100.0, 10.0, ElasticInOut::new(10.0));

        for time in 0..=10 {
            let time = time as f32;

            let our_value = tweener.move_to(time);
            let easer = Elastic::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_relative_eq!(our_value, easer, max_relative = 0.000001);
        }
    }

    #[test]
    fn t_in_out_rev() {
        let mut tweener = Tweener::new(100.0, 0.0, 10.0, ElasticInOut::new(10.0));

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.move_to(time);
            let o = Elastic::ease_in_out(time, 100.0, -100.0, 10.0);

            assert_relative_eq!(v, o, max_relative = 0.000001);
        }
    }
}
