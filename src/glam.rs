impl crate::TweenValue for glam::Vec2 {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl crate::TweenValue for glam::UVec2 {
    fn scale(self, scale: f32) -> Self {
        (self.as_vec2() * scale).as_uvec2()
    }
}

impl crate::TweenValue for glam::DVec2 {
    fn scale(self, scale: f32) -> Self {
        (self.as_vec2() * scale).as_dvec2()
    }
}
