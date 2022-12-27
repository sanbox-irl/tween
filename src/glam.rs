impl crate::TweenValue for glam::Vec2 {
    const ZERO: Self = Self::ZERO;

    fn scale(self, scale: f64) -> Self {
        (self.as_dvec2() * scale).as_vec2()
    }
}

impl crate::TweenValue for glam::UVec2 {
    const ZERO: Self = Self::ZERO;

    fn scale(self, scale: f64) -> Self {
        (self.as_dvec2() * scale).as_uvec2()
    }
}

impl crate::TweenValue for glam::DVec2 {
    const ZERO: Self = Self::ZERO;

    fn scale(self, scale: f64) -> Self {
        self * scale
    }
}
