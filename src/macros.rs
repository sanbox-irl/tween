/// This is internal to the library, but allows for simple numeric
/// types to be made into a time value.
macro_rules! declare_time {
    ($($t:ty),*) => {
        $(
        impl TweenTime for $t {
            const ZERO: Self = 0;

            #[inline(always)]
            fn to_f32(self) -> f32 {
                self as f32
            }
        }
        )*
    };
}

/// This is internal to the library, but allows for simple numeric
/// types to be made into a tween_value.
macro_rules! declare_value {
    ($($t:ident),*) => {
        $(
        impl TweenValue for $t {
            #[inline(always)]
            fn scale(self, scale: f32) -> Self {
                (self as f32 * scale) as $t
            }
        })*
    };
}

/// This is internal to the library, but allows for creating simple ease-style
/// tweens.
macro_rules! declare_tween {
    (
        $(#[$struct_meta:meta])*
        pub struct $name:ident;

        $(#[$method_meta:meta])*
        pub fn $tweener_method_name:ident;

        $(#[$at_method_meta:meta])*
        pub fn $tweener_at_method_name:ident;

        $tween:item
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug, PartialEq, Eq, Clone, Default)]
        pub struct $name;

        impl $name {
            /// Creates a new tween out of a range with a duration.
            pub fn new<Value, Time>() -> Self
            where
                Value: $crate::TweenValue,
                Time: $crate::TweenTime,
            {
                Self
            }

            /// Run the given Tween with a new time.
            #[inline(always)]
            $tween
        }

        impl<Value> $crate::Tween<Value> for $name
        where
            Value: $crate::TweenValue,
        {
            #[inline(always)]
            fn tween(&mut self, value_delta: Value, percent_time: f32) -> Value {
                self.tween(value_delta, percent_time)
            }
        }

        impl<Value, Time> $crate::Tweener<Value, Time, $crate::$name>
        where
            Time: $crate::TweenTime,
            Value: $crate::TweenValue,
        {
            $(#[$method_meta])*
            pub fn $tweener_method_name(start: Value, end: Value, duration: Time) -> $crate::Tweener<Value, Time, $crate::$name> {
                $crate::Tweener::new(start, end, duration, $crate::$name)
            }

            $(#[$at_method_meta])*
            pub fn $tweener_at_method_name(start: Value, end: Value, duration: Time, current_time: Time) -> $crate::Tweener<Value, Time, $crate::$name> {
                $crate::Tweener::new_at(start, end, duration, $crate::$name, current_time)
            }
        }

        impl<Value, Time> $crate::FixedTweener<Value, Time, $crate::$name>
        where
            Time: $crate::TweenTime,
            Value: $crate::TweenValue,
        {
            $(#[$method_meta])*
            pub fn $tweener_method_name(start: Value, end: Value, duration: Time, delta: Time) -> $crate::FixedTweener<Value, Time, $crate::$name> {
                $crate::FixedTweener::new(start, end, duration, $crate::$name, delta)
            }

            $(#[$at_method_meta])*
            pub fn $tweener_at_method_name(start: Value, end: Value, duration: Time, current_time: Time) -> $crate::Tweener<Value, Time, $crate::$name> {
                $crate::Tweener::new_at(start, end, duration, $crate::$name, current_time)
            }
        }
    };
}

macro_rules! impl_tween_for_box {
    ($($trait_bounds:ident),*) => {
        #[cfg(feature = "std")]
        impl<Value: TweenValue> Tween<Value> for std::boxed::Box<dyn Tween<Value> $(+ $trait_bounds)*> {
            #[inline(always)]
            fn tween(&mut self, value_delta: Value, percent: f32) -> Value {
                (**self).tween(value_delta, percent)
            }

            fn is_finite(&self) -> bool {
                (**self).is_finite()
            }
        }
    };
}

macro_rules! test_tween {
    ($name:ident) => {
        #[cfg(test)]
        mod test {
            paste::paste! {
                use super::*;
                use approx::assert_relative_eq;
                use easer::functions::{$name as [<Ease $name>], Easing};

                #[test]
                fn t_in() {
                    let mut tweener = $crate::Tweener::new(0.0, 100.0, 10.0, [<$name In>]);

                    for time in 0..=10 {
                        let time = time as f32;

                        let v = tweener.move_to(time);
                        let o = [<Ease $name>]::ease_in(time, 0.0, 100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001, epsilon = 0.0001);
                    }
                }

                #[test]
                fn t_in_f() {
                    assert_relative_eq!([<$name In>].tween(5.0, 0.0), [<Ease $name>]::ease_in(0.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name In>].tween(5.0, 0.1), [<Ease $name>]::ease_in(0.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name In>].tween(5.0, 0.2), [<Ease $name>]::ease_in(1.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name In>].tween(5.0, 0.3), [<Ease $name>]::ease_in(1.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name In>].tween(5.0, 0.4), [<Ease $name>]::ease_in(2.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name In>].tween(5.0, 0.5), [<Ease $name>]::ease_in(2.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name In>].tween(5.0, 0.6), [<Ease $name>]::ease_in(3.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name In>].tween(5.0, 0.7), [<Ease $name>]::ease_in(3.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name In>].tween(5.0, 0.8), [<Ease $name>]::ease_in(4.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name In>].tween(5.0, 0.9), [<Ease $name>]::ease_in(4.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name In>].tween(5.0, 1.0), [<Ease $name>]::ease_in(5.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                }

                #[test]
                fn t_in_rev() {
                    let mut tweener = $crate::Tweener::new(100.0, 0.0, 10.0, [<$name In>]);

                    for time in 0..=10 {
                        let time = time as f32;

                        let v = tweener.move_to(time);
                        let o = [<Ease $name>]::ease_in(time, 100.0, -100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }

                #[test]
                fn t_out() {
                    let mut tweener = $crate::Tweener::new(0.0, 100.0, 10.0, [<$name Out>]);

                    for time in 0..=10 {
                        let time = time as f32;

                        let v = tweener.move_to(time);
                        let o = [<Ease $name>]::ease_out(time, 0.0, 100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }

                #[test]
                fn t_out_f() {
                    assert_relative_eq!([<$name Out>].tween(5.0, 0.0), [<Ease $name>]::ease_out(0.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name Out>].tween(5.0, 0.1), [<Ease $name>]::ease_out(0.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name Out>].tween(5.0, 0.2), [<Ease $name>]::ease_out(1.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name Out>].tween(5.0, 0.3), [<Ease $name>]::ease_out(1.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name Out>].tween(5.0, 0.4), [<Ease $name>]::ease_out(2.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name Out>].tween(5.0, 0.5), [<Ease $name>]::ease_out(2.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name Out>].tween(5.0, 0.6), [<Ease $name>]::ease_out(3.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name Out>].tween(5.0, 0.7), [<Ease $name>]::ease_out(3.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name Out>].tween(5.0, 0.8), [<Ease $name>]::ease_out(4.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name Out>].tween(5.0, 0.9), [<Ease $name>]::ease_out(4.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name Out>].tween(5.0, 1.0), [<Ease $name>]::ease_out(5.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                }

                #[test]
                fn t_out_rev() {
                    let mut tweener = $crate::Tweener::new(100.0, 0.0, 10.0, [<$name Out>]);

                    for time in 0..=10 {
                        let time = time as f32;

                        let v = tweener.move_to(time);
                        let o = [<Ease $name>]::ease_out(time, 100.0, -100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }

                #[test]
                fn t_in_out() {
                    let mut tweener = $crate::Tweener::new(0.0, 100.0, 10.0, [<$name InOut>]);

                    for time in 0..=10 {
                        let time = time as f32;

                        let our_value = tweener.move_to(time);
                        let easer = [<Ease $name>]::ease_in_out(time, 0.0, 100.0, 10.0);

                        assert_relative_eq!(our_value, easer, max_relative = 0.000001);
                    }
                }

                #[test]
                fn t_in_out_f() {
                    assert_relative_eq!([<$name InOut>].tween(5.0, 0.0), [<Ease $name>]::ease_in_out(0.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name InOut>].tween(5.0, 0.1), [<Ease $name>]::ease_in_out(0.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name InOut>].tween(5.0, 0.2), [<Ease $name>]::ease_in_out(1.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name InOut>].tween(5.0, 0.3), [<Ease $name>]::ease_in_out(1.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name InOut>].tween(5.0, 0.4), [<Ease $name>]::ease_in_out(2.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name InOut>].tween(5.0, 0.5), [<Ease $name>]::ease_in_out(2.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name InOut>].tween(5.0, 0.6), [<Ease $name>]::ease_in_out(3.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name InOut>].tween(5.0, 0.7), [<Ease $name>]::ease_in_out(3.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name InOut>].tween(5.0, 0.8), [<Ease $name>]::ease_in_out(4.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name InOut>].tween(5.0, 0.9), [<Ease $name>]::ease_in_out(4.5, 0.0, 5.0, 5.0), epsilon = 0.00001);
                    assert_relative_eq!([<$name InOut>].tween(5.0, 1.0), [<Ease $name>]::ease_in_out(5.0, 0.0, 5.0, 5.0), epsilon = 0.00001);
                }

                #[test]
                fn t_in_out_rev() {
                    let mut tweener = $crate::Tweener::new(100.0, 0.0, 10.0, [<$name InOut>]);

                    for time in 0..=10 {
                        let time = time as f32;

                        let v = tweener.move_to(time);
                        let o = [<Ease $name>]::ease_in_out(time, 100.0, -100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }
            }
        }
    };
}
