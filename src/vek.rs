impl<T> crate::TweenValue for vek::Vec2<T>
where
    T: crate::TweenValue,
{
    fn scale(self, scale: f32) -> Self {
        vek::Vec2::new(self.x.scale(scale), self.y.scale(scale))
    }
}

impl<T> crate::TweenValue for vek::Vec3<T>
where
    T: crate::TweenValue,
{
    fn scale(self, scale: f32) -> Self {
        vek::Vec3::new(self.x.scale(scale), self.y.scale(scale), self.z.scale(scale))
    }
}

impl<T> crate::TweenValue for vek::Vec4<T>
where
    T: crate::TweenValue,
{
    fn scale(self, scale: f32) -> Self {
        vek::Vec4::new(
            self.x.scale(scale),
            self.y.scale(scale),
            self.z.scale(scale),
            self.w.scale(scale),
        )
    }
}
