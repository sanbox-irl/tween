impl crate::TweenValue for ultraviolet::Vec2 {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl crate::TweenValue for ultraviolet::Vec3 {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl crate::TweenValue for ultraviolet::Vec4 {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl crate::TweenValue for ultraviolet::DVec2 {
    fn scale(self, scale: f32) -> Self {
        self * scale as f64
    }
}

impl crate::TweenValue for ultraviolet::DVec3 {
    fn scale(self, scale: f32) -> Self {
        self * scale as f64
    }
}

impl crate::TweenValue for ultraviolet::DVec4 {
    fn scale(self, scale: f32) -> Self {
        self * scale as f64
    }
}
