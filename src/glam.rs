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

impl crate::TweenValue for glam::IVec2 {
    fn scale(self, scale: f32) -> Self {
        (self.as_vec2() * scale).as_ivec2()
    }
}

impl crate::TweenValue for glam::DVec2 {
    fn scale(self, scale: f32) -> Self {
        (self.as_vec2() * scale).as_dvec2()
    }
}

impl crate::TweenValue for glam::Vec3 {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl crate::TweenValue for glam::UVec3 {
    fn scale(self, scale: f32) -> Self {
        (self.as_vec3() * scale).as_uvec3()
    }
}

impl crate::TweenValue for glam::IVec3 {
    fn scale(self, scale: f32) -> Self {
        (self.as_vec3() * scale).as_ivec3()
    }
}

impl crate::TweenValue for glam::DVec3 {
    fn scale(self, scale: f32) -> Self {
        (self.as_vec3() * scale).as_dvec3()
    }
}

impl crate::TweenValue for glam::Vec4 {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl crate::TweenValue for glam::UVec4 {
    fn scale(self, scale: f32) -> Self {
        (self.as_vec4() * scale).as_uvec4()
    }
}

impl crate::TweenValue for glam::IVec4 {
    fn scale(self, scale: f32) -> Self {
        (self.as_vec4() * scale).as_ivec4()
    }
}

impl crate::TweenValue for glam::DVec4 {
    fn scale(self, scale: f32) -> Self {
        (self.as_vec4() * scale).as_dvec4()
    }
}

impl crate::TweenValue for glam::Mat2 {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl crate::TweenValue for glam::Mat3 {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl crate::TweenValue for glam::Mat3A {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl crate::TweenValue for glam::Mat4 {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl crate::TweenValue for glam::DMat2 {
    fn scale(self, scale: f32) -> Self {
        self * scale as f64
    }
}

impl crate::TweenValue for glam::DMat3 {
    fn scale(self, scale: f32) -> Self {
        self * scale as f64
    }
}

impl crate::TweenValue for glam::DMat4 {
    fn scale(self, scale: f32) -> Self {
        self * scale as f64
    }
}

impl crate::TweenValue for glam::Affine2 {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl crate::TweenValue for glam::Affine3A {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl crate::TweenValue for glam::DAffine2 {
    fn scale(self, scale: f32) -> Self {
        self * scale as f64
    }
}

impl crate::TweenValue for glam::DAffine3 {
    fn scale(self, scale: f32) -> Self {
        self * scale as f64
    }
}
