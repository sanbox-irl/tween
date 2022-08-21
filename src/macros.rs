/// This is internal to the library, but allows for simple numeric
/// types to be made into a time value.
macro_rules! declare_time {
    ($t:ty) => {
        impl TweenTime for $t {
            const ZERO: Self = 0;

            fn percent(duration: Self, current_time: Self) -> f64 {
                current_time as f64 / duration as f64
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

            fn percent(duration: Self, current_time: Self) -> f64 {
                current_time as f64 / duration as f64
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
            TValue: TweenValue,
            TTime: TweenTime,
        {
            /// Creates a new tween out of a range with a duration.
            pub fn new(initial_value: TValue, final_value: TValue, duration: TTime) -> Self {
                let delta = TValue::calculate_delta(final_value, initial_value);
                Self {
                    initial_value,
                    final_value,
                    value_delta: delta,
                    duration,
                }
            }

            /// Run the given Tween with a new time.
            pub fn run(&mut self, new_time: <Self as Tween>::Time) -> <Self as Tween>::Value {
                // we pass this through so that we don't require users to (annoyingly) import
                // a trait. Inherent methods in traits pls!
                <Self as Tween>::run(self, new_time)
            }
        }

        impl<V, T> Tween for $name<V, T>
        where
            V: TweenValue,
            T: TweenTime,
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
            TValue: TweenValue,
            TTime: TweenTime,
        {
            /// Creates a new tween out of a range with a duration.
            pub fn new(initial_value: TValue, final_value: TValue, duration: TTime) -> Self {
                let value_delta = TValue::calculate_delta(final_value, initial_value);
                let half_delta = TValue::scale(value_delta, 0.5);
                Self {
                    initial_value,
                    final_value,
                    half_delta,
                    duration,
                }
            }
        }

        impl<V, T> Tween for $name<V, T>
        where
            V: TweenValue,
            T: TweenTime,
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
    };
}
