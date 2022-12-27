/// This is internal to the library, but allows for simple numeric
/// types to be made into a time value.
macro_rules! declare_time {
    ($t:ty) => {
        impl TweenTime for $t {
            const ZERO: Self = 0;

            fn percent(duration: Self, current_time: Self) -> f64 {
                current_time as f64 / duration as f64
            }

            fn to_f32(self) -> f32 {
                self as f32
            }

            fn to_f64(self) -> f64 {
                self as f64
            }

            fn scale(self, other: f64) -> Self {
                (self as f64 * other) as Self
            }

            fn div_euclid(self, other: Self) -> Self {
                self.div_euclid(other)
            }
        }
    };
    (float $t:ty) => {
        impl TweenTime for $t {
            const ZERO: Self = 0.0;

            fn percent(duration: Self, current_time: Self) -> f64 {
                current_time as f64 / duration as f64
            }

            fn to_f32(self) -> f32 {
                self as f32
            }

            fn to_f64(self) -> f64 {
                self as f64
            }

            fn scale(self, other: f64) -> Self {
                (self as f64 * other) as Self
            }

            fn div_euclid(self, other: Self) -> Self {
                self.div_euclid(other)
            }
        }
    };
}

/// This is internal to the library, but allows for simple numeric
/// types to be made into a tween_value.
macro_rules! declare_value {
    ($t:ty) => {
        impl TweenValue for $t {
            const ZERO: Self = 0;

            fn scale(self, scale: f64) -> Self {
                (self as f64 * scale) as $t
            }
        }
    };

    (float $t:ty) => {
        impl TweenValue for $t {
            const ZERO: Self = 0.0;

            fn scale(self, scale: f64) -> Self {
                (self as f64 * scale) as $t
            }
        }
    };
}

/// This is internal to the library, but allows for creating simple ease-style
/// tweens.
macro_rules! declare_tween {
    (
        $(#[$struct_meta:meta])*
        pub struct $name:ident;

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
                <Self as $crate::SizedTween<Value, Time>>::new()
            }

             /// Calculate what a percent into the Tween based on time. For almost all Tweens,
            /// this is simply `current_time / duration` (`Bounce` and `Elastic` are the exceptions).
            pub fn percent<Value, Time>(&mut self, current_time: Time, duration: Time) -> f64
            where
                Value: $crate::TweenValue,
                Time: $crate::TweenTime,
            {
                <Self as $crate::Tween<Value, Time>>::percent(self, current_time, duration)
            }

            /// Run the given Tween with a new time.
            pub fn tween<Value, Time>(&mut self, value_delta: Value, percent: f64) -> Value
            where
                Value: $crate::TweenValue,
                Time: $crate::TweenTime,
            {
                // we pass this through so that we don't require users to (annoyingly) import
                // a trait. Inherent methods in traits pls!
                <Self as $crate::Tween<Value, Time>>::tween(self, value_delta, percent)
            }
        }

        // impl Default for $name {
        //     fn default() -> Self {
        //         Self::
        //     }
        // }

        impl<Value, Time> $crate::Tween<Value, Time> for $name
        where
            Value: $crate::TweenValue,
            Time: $crate::TweenTime,
        {
            $tween
        }

        impl<Value, Time> $crate::SizedTween<Value, Time> for $name
        where
            Value: $crate::TweenValue,
            Time: $crate::TweenTime,
        {
            /// Creates a new Tween
            fn new() -> Self {
                Self
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
                        let time = time as f64;

                        let v = tweener.move_to(time);
                        let o = [<Ease $name>]::ease_in(time, 0.0, 100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }

                #[test]
                fn t_in_rev() {
                    let mut tweener = $crate::Tweener::new(100.0, 0.0, 10.0, [<$name In>]);

                    for time in 0..=10 {
                        let time = time as f64;

                        let v = tweener.move_to(time);
                        let o = [<Ease $name>]::ease_in(time, 100.0, -100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }

                #[test]
                fn t_out() {
                    let mut tweener = $crate::Tweener::new(0.0, 100.0, 10.0, [<$name Out>]);

                    for time in 0..=10 {
                        let time = time as f64;

                        let v = tweener.move_to(time);
                        let o = [<Ease $name>]::ease_out(time, 0.0, 100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }

                #[test]
                fn t_out_rev() {
                    let mut tweener = $crate::Tweener::new(100.0, 0.0, 10.0, [<$name Out>]);

                    for time in 0..=10 {
                        let time = time as f64;

                        let v = tweener.move_to(time);
                        let o = [<Ease $name>]::ease_out(time, 100.0, -100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }

                #[test]
                fn t_in_out() {
                    let mut tweener = $crate::Tweener::new(0.0, 100.0, 10.0, [<$name InOut>]);

                    for time in 0..=10 {
                        let time = time as f64;

                        let our_value = tweener.move_to(time);
                        let easer = [<Ease $name>]::ease_in_out(time, 0.0, 100.0, 10.0);

                        assert_relative_eq!(our_value, easer, max_relative = 0.000001);
                    }
                }


                #[test]
                fn t_in_out_rev() {
                    let mut tweener = $crate::Tweener::new(100.0, 0.0, 10.0, [<$name InOut>]);

                    for time in 0..=10 {
                        let time = time as f64;

                        let v = tweener.move_to(time);
                        let o = [<Ease $name>]::ease_in_out(time, 100.0, -100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }
            }
        }
    };
}
