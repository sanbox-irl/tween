impl<T> crate::TweenValue for cgmath::Vector2<T>
where
    T: crate::TweenValue + cgmath::BaseNum,
{
    fn scale(self, scale: f32) -> Self {
        cgmath::Vector2::new(self.x.scale(scale), self.y.scale(scale))
    }
}

impl<T> crate::TweenValue for cgmath::Vector3<T>
where
    T: crate::TweenValue + cgmath::BaseNum + cgmath::num_traits::NumCast + cgmath::num_traits::cast::NumCast,
{
    fn scale(self, scale: f32) -> Self {
        cgmath::Vector3::new(self.x.scale(scale), self.y.scale(scale), self.z.scale(scale))
    }
}

impl<T> crate::TweenValue for cgmath::Vector4<T>
where
    T: crate::TweenValue + cgmath::BaseNum,
{
    fn scale(self, scale: f32) -> Self {
        cgmath::Vector4::new(
            self.x.scale(scale),
            self.y.scale(scale),
            self.z.scale(scale),
            self.w.scale(scale),
        )
    }
}
