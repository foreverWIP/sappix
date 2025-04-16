use core::ops::Mul;

use glam::Vec4;

#[derive(Clone, Copy)]
pub enum ColorMode<const N: usize> {
    Solid(FBColor),
    PerPoint([FBColor; N]),
}

#[derive(Clone, Copy)]
pub struct FBColor {
    internal: Vec4,
}
impl FBColor {
    pub const BLACK: Self = Self {
        internal: Vec4::new(0.0, 0.0, 0.0, 1.0),
    };

    pub const CYAN: Self = Self {
        internal: Vec4::new(0.0, 1.0, 1.0, 1.0),
    };

    pub const EMPTY: Self = Self {
        internal: Vec4::ZERO,
    };

    pub const GRAY50: Self = Self {
        internal: Vec4::new(0.5, 0.5, 0.5, 1.0),
    };

    pub const MAGENTA: Self = Self {
        internal: Vec4::new(1.0, 0.0, 1.0, 1.0),
    };

    pub const WHITE: Self = Self {
        internal: Vec4::ONE,
    };

    pub const YELLOW: Self = Self {
        internal: Vec4::new(1.0, 1.0, 0.0, 1.0),
    };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            internal: Vec4::new(r, g, b, a),
        }
    }

    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    pub fn to_rgba8(&self) -> [u8; 4] {
        [
            (self.r() * 255.0) as u8,
            (self.g() * 255.0) as u8,
            (self.b() * 255.0) as u8,
            (self.a() * 255.0) as u8,
        ]
    }

    pub fn r(&self) -> f32 {
        self.internal.x
    }

    pub fn g(&self) -> f32 {
        self.internal.y
    }

    pub fn b(&self) -> f32 {
        self.internal.z
    }

    pub fn a(&self) -> f32 {
        self.internal.w
    }

    pub fn set_r(&mut self, value: f32) {
        self.internal.x = value;
    }

    pub fn set_g(&mut self, value: f32) {
        self.internal.y = value;
    }

    pub fn set_b(&mut self, value: f32) {
        self.internal.z = value;
    }

    pub fn set_a(&mut self, value: f32) {
        self.internal.z = value;
    }

    pub fn with_a(&self, a: f32) -> Self {
        Self {
            internal: self.internal.with_w(a),
        }
    }

    pub fn lerp(&self, rhs: Self, by: f32) -> Self {
        Self {
            internal: self.internal.lerp(rhs.internal, by),
        }
    }
}
impl Mul for FBColor {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            internal: self.internal * rhs.internal,
        }
    }
}
