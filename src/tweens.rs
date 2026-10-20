mod linear;
pub use linear::Linear;

mod cubic;
pub use cubic::{CubicIn, CubicInOut, CubicOut};

mod sine;
pub use sine::{SineIn, SineInOut, SineOut};

mod quint;
pub use quint::{QuintIn, QuintInOut, QuintOut};

mod quad;
pub use quad::{QuadIn, QuadInOut, QuadOut};

mod quart;
pub use quart::{QuartIn, QuartInOut, QuartOut};

mod expo;
pub use expo::{ExpoIn, ExpoInOut, ExpoOut};

mod circ;
pub use circ::{CircIn, CircInOut, CircOut};

mod back;
pub use back::{BackIn, BackInOut, BackOut};

mod elastic;
pub use elastic::{ElasticIn, ElasticInOut, ElasticOut};

mod bounce;
pub use bounce::{BounceIn, BounceInOut, BounceOut};

#[cfg(test)]
mod tests {
    use super::*;

    /// Integer tweens should be the float tween truncated, which only holds if every tween calls
    /// `scale` exactly once.
    #[test]
    fn int_tweens_scale_once() {
        macro_rules! check {
            ($($tween:ident),*) => {$(
                for delta in -50..=50 {
                    for step in 0..=20 {
                        let percent = step as f32 / 20.0;

                        assert_eq!(
                            $tween.tween(delta, percent),
                            $tween.tween(delta as f32, percent) as i32,
                            "{} delta={delta} percent={percent}",
                            stringify!($tween)
                        );
                    }
                }
            )*};
        }

        check!(
            Linear,
            SineIn,
            SineOut,
            SineInOut,
            QuadIn,
            QuadOut,
            QuadInOut,
            CubicIn,
            CubicOut,
            CubicInOut,
            QuartIn,
            QuartOut,
            QuartInOut,
            QuintIn,
            QuintOut,
            QuintInOut,
            ExpoIn,
            ExpoOut,
            ExpoInOut,
            CircIn,
            CircOut,
            CircInOut,
            BackIn,
            BackOut,
            BackInOut,
            ElasticIn,
            ElasticOut,
            ElasticInOut,
            BounceIn,
            BounceOut,
            BounceInOut
        );
    }
}
