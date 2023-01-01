impl<T> crate::TweenValue for nalgebra::Vector2<T>
where
    T: crate::TweenValue + nalgebra::ClosedAdd + nalgebra::ClosedSub + nalgebra::Scalar,
{
    fn scale(self, scale: f32) -> Self {
        nalgebra::Vector2::new(self[0].scale(scale), self[1].scale(scale))
    }
}

impl<T> crate::TweenValue for nalgebra::Vector3<T>
where
    T: crate::TweenValue + nalgebra::ClosedAdd + nalgebra::ClosedSub + nalgebra::Scalar,
{
    fn scale(self, scale: f32) -> Self {
        nalgebra::Vector3::new(self[0].scale(scale), self[1].scale(scale), self[2].scale(scale))
    }
}

impl<T> crate::TweenValue for nalgebra::Vector4<T>
where
    T: crate::TweenValue + nalgebra::ClosedAdd + nalgebra::ClosedSub + nalgebra::Scalar,
{
    fn scale(self, scale: f32) -> Self {
        nalgebra::Vector4::new(
            self[0].scale(scale),
            self[1].scale(scale),
            self[2].scale(scale),
            self[3].scale(scale),
        )
    }
}
