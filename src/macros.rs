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
    ($name:ident, $update:item) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub struct $name<TValue, TTime> {
            range: RangeInclusive<TValue>,
            value_delta: TValue,
            duration: TTime,
        }

        impl<TValue, TTime> $name<TValue, TTime>
        where
            TValue: TweenValue,
            TTime: TweenTime,
        {
            pub fn new(range: RangeInclusive<TValue>, duration: TTime) -> Self {
                let delta = TValue::calculate_delta(*range.end(), *range.start());
                Self {
                    range,
                    value_delta: delta,
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

            fn range(&self) -> &RangeInclusive<V> {
                &self.range
            }

            fn duration(&self) -> T {
                self.duration
            }
        }
    };
}

/// This is internal to the library, but allows for creating simple ease-style
/// tweens.
macro_rules! declare_in_out_tween {
    ($name:ident, $update:item) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub struct $name<TValue, TTime> {
            range: RangeInclusive<TValue>,
            half_delta: TValue,
            duration: TTime,
        }

        impl<TValue, TTime> $name<TValue, TTime>
        where
            TValue: TweenValue,
            TTime: TweenTime,
        {
            pub fn new(range: RangeInclusive<TValue>, duration: TTime) -> Self {
                let value_delta = TValue::calculate_delta(*range.end(), *range.start());
                let half_delta = TValue::scale(value_delta, 0.5);
                Self {
                    range,
                    half_delta,
                    duration,
                }
            }
        }

        impl<TValue, TTime> Tween for $name<TValue, TTime>
        where
            TValue: TweenValue,
            TTime: TweenTime,
        {
            type Value = TValue;
            type Time = TTime;

            $update

            fn range(&self) -> &RangeInclusive<TValue> {
                &self.range
            }

            fn duration(&self) -> TTime {
                self.duration
            }
        }
    };
}
