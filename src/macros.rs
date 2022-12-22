/// This is internal to the library, but allows for simple numeric
/// types to be made into a time value.
macro_rules! declare_time {
    ($t:ty) => {
        impl TweenTime for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            fn percent(duration: Self, current_time: Self) -> f64 {
                current_time as f64 / duration as f64
            }

            fn modulo(self, other: Self) -> Self {
                self % other
            }

            fn add(self, other: Self) -> Self {
                self + other
            }

            fn sub(self, other: Self) -> Self {
                self - other
            }

            fn as_f64(self) -> f64 {
                self as f64
            }

            fn scale(self, other: f64) -> Self {
                (self as f64 * other) as Self
            }

            fn is_complete(self, duration: Self) -> bool {
                self >= duration
            }
        }
    };
    (float $t:ty) => {
        impl TweenTime for $t {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;

            fn percent(duration: Self, current_time: Self) -> f64 {
                current_time as f64 / duration as f64
            }

            fn modulo(self, other: Self) -> Self {
                self % other
            }

            fn add(self, other: Self) -> Self {
                self + other
            }

            fn sub(self, other: Self) -> Self {
                self - other
            }

            fn scale(self, other: f64) -> Self {
                (self as f64 * other) as Self
            }

            fn as_f64(self) -> f64 {
                self as f64
            }

            fn is_complete(self, duration: Self) -> bool {
                self >= duration
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

            fn add(self, other: Self) -> Self {
                self + other
            }

            fn calculate_delta(destination: Self, start: Self) -> Self {
                destination - start
            }

            fn scale(self, scale: f64) -> Self {
                (self as f64 * scale) as $t
            }
        }
    };

    (float $t:ty) => {
        impl TweenValue for $t {
            const ZERO: Self = 0.0;

            fn add(self, other: Self) -> Self {
                self + other
            }

            fn calculate_delta(destination: Self, start: Self) -> Self {
                destination - start
            }

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

        $update:item
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub struct $name<TValue, TTime> {
            initial_value: TValue,
            final_value: TValue,
            value_delta: TValue,
            duration: TTime,
        }

        impl<TValue, TTime> $name<TValue, TTime>
        where
            TValue: $crate::TweenValue,
            TTime: $crate::TweenTime,
        {
            /// Creates a new tween out of a range with a duration.
            pub fn new(initial_value: TValue, final_value: TValue, duration: TTime) -> Self {
                <Self as $crate::SizedTween>::new(initial_value, final_value, duration)
            }

            /// Run the given Tween with a new time.
            pub fn run(&mut self, new_time: TTime) -> TValue {
                // we pass this through so that we don't require users to (annoyingly) import
                // a trait. Inherent methods in traits pls!
                <Self as $crate::Tween>::run(self, new_time)
            }
        }

        impl<V, T> $crate::Tween for $name<V, T>
        where
            V: $crate::TweenValue,
            T: $crate::TweenTime,
        {
            type Value = V;
            type Time = T;

            $update

            fn duration(&self) -> T {
                self.duration
            }

            fn initial_value(&self) -> V {
                self.initial_value
            }

            fn final_value(&self) -> V {
                self.final_value
            }
        }

        impl<V, T> $crate::SizedTween for $name<V, T>
        where
            V: $crate::TweenValue,
            T: $crate::TweenTime,
        {
            /// Creates a new Tween
            fn new(initial_value: V, final_value: V, duration: T) -> Self {
                let delta = V::calculate_delta(final_value, initial_value);
                Self {
                    initial_value,
                    final_value,
                    value_delta: delta,
                    duration,
                }
            }
        }
    };
}

/// This is internal to the library, but allows for creating simple ease-style
/// tweens.
macro_rules! declare_in_out_tween {
    (
        $(#[$struct_meta:meta])*
        pub struct $name:ident;

        $update:item
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub struct $name<TValue, TTime> {
            initial_value: TValue,
            final_value: TValue,
            half_delta: TValue,
            duration: TTime,
        }

        impl<TValue, TTime> $name<TValue, TTime>
        where
            TValue: $crate::TweenValue,
            TTime: $crate::TweenTime,
        {
            /// Creates a new tween out of a range with a duration.
            pub fn new(initial_value: TValue, final_value: TValue, duration: TTime) -> Self {
                <Self as $crate::SizedTween>::new(initial_value, final_value, duration)
            }

            /// Run the given Tween with a new time.
            pub fn run(&mut self, new_time: TTime) -> TValue {
                // we pass this through so that we don't require users to (annoyingly) import
                // a trait. Inherent methods in traits pls!
                <Self as $crate::Tween>::run(self, new_time)
            }
        }

        impl<V, T> $crate::Tween for $name<V, T>
        where
            V: $crate::TweenValue,
            T: $crate::TweenTime,
        {
            type Value = V;
            type Time = T;

            $update

            fn duration(&self) -> T {
                self.duration
            }

            fn initial_value(&self) -> V {
                self.initial_value
            }

            fn final_value(&self) -> V {
                self.final_value
            }
        }

        impl<V, T> $crate::SizedTween for $name<V, T>
        where
            V: $crate::TweenValue,
            T: $crate::TweenTime,
        {
            /// Creates a new Tween
            fn new(initial_value: V, final_value: V, duration: T) -> Self {
                let value_delta = V::calculate_delta(final_value, initial_value);
                let half_delta = V::scale(value_delta, 0.5);
                Self {
                    initial_value,
                    final_value,
                    half_delta,
                    duration,
                }
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
                    let mut tweener = [<$name In>]::new(0.0, 100.0, 10.0);

                    for time in 0..=10 {
                        let time = time as f64;

                        let v = tweener.run(time);
                        let o = [<Ease $name>]::ease_in(time, 0.0, 100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }

                #[test]
                fn t_in_rev() {
                    let mut tweener = [<$name In>]::new(100.0, 0.0, 10.0);

                    for time in 0..=10 {
                        let time = time as f64;

                        let v = tweener.run(time);
                        let o = [<Ease $name>]::ease_in(time, 100.0, -100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }

                #[test]
                fn t_out() {
                    let mut tweener = [<$name Out>]::new(0.0, 100.0, 10.0);

                    for time in 0..=10 {
                        let time = time as f64;

                        let v = tweener.run(time);
                        let o = [<Ease $name>]::ease_out(time, 0.0, 100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }

                #[test]
                fn t_out_rev() {
                    let mut tweener = [<$name Out>]::new(100.0, 0.0, 10.0);

                    for time in 0..=10 {
                        let time = time as f64;

                        let v = tweener.run(time);
                        let o = [<Ease $name>]::ease_out(time, 100.0, -100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }

                #[test]
                fn t_in_out() {
                    let mut tweener = [<$name InOut>]::new(0.0, 100.0, 10.0);

                    for time in 0..=10 {
                        let time = time as f64;

                        let our_value = tweener.run(time);
                        let easer = [<Ease $name>]::ease_in_out(time, 0.0, 100.0, 10.0);

                        assert_relative_eq!(our_value, easer, max_relative = 0.000001);
                    }
                }


                #[test]
                fn t_in_out_rev() {
                    let mut tweener = [<$name InOut>]::new(100.0, 0.0, 10.0);

                    for time in 0..=10 {
                        let time = time as f64;

                        let v = tweener.run(time);
                        let o = [<Ease $name>]::ease_in_out(time, 100.0, -100.0, 10.0);

                        assert_relative_eq!(v, o, max_relative = 0.000001);
                    }
                }
            }
        }
    };
}
