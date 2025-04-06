use core::ops::BitAnd;

use glam::FloatExt;
use num::{Float, Num, PrimInt};

#[derive(Clone, Copy)]
pub enum ColorMode<const N: usize> {
    Solid(FBColor),
    PerPoint([FBColor; N]),
}

#[derive(Clone, Copy)]
pub enum FBColor {
    Rgba8 { r: u8, g: u8, b: u8, a: u8 },
}
impl FBColor {
    pub const EMPTY_RGBA8: Self = Self::Rgba8 {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    pub const GRAY50_RGBA8: Self = Self::Rgba8 {
        r: 0x7f,
        g: 0x7f,
        b: 0x7f,
        a: 0xff,
    };

    pub const WHITE_RGBA8: Self = Self::Rgba8 {
        r: 0xff,
        g: 0xff,
        b: 0xff,
        a: 0xff,
    };

    pub fn r(&self) -> u8 {
        match self {
            FBColor::Rgba8 { r, .. } => *r,
        }
    }

    pub fn g(&self) -> u8 {
        match self {
            FBColor::Rgba8 { g, .. } => *g,
        }
    }

    pub fn b(&self) -> u8 {
        match self {
            FBColor::Rgba8 { b, .. } => *b,
        }
    }

    pub fn a(&self) -> u8 {
        match self {
            FBColor::Rgba8 { a, .. } => *a,
        }
    }

    pub fn set_r(&mut self, value: u8) {
        match self {
            FBColor::Rgba8 { r, .. } => {
                *r = value;
            }
        }
    }

    pub fn set_g(&mut self, value: u8) {
        match self {
            FBColor::Rgba8 { g, .. } => {
                *g = value;
            }
        }
    }

    pub fn set_b(&mut self, value: u8) {
        match self {
            FBColor::Rgba8 { b, .. } => {
                *b = value;
            }
        }
    }

    pub fn set_a(&mut self, value: u8) {
        match self {
            FBColor::Rgba8 { a, .. } => {
                *a = value;
            }
        }
    }

    pub fn uses_a(&self) -> bool {
        match self {
            FBColor::Rgba8 { .. } => true,
        }
    }

    pub fn lerp(&self, rhs: Self, by: u8) -> Self {
        if by == u8::MIN {
            return *self;
        }
        if by == u8::MAX {
            return match self {
                Self::Rgba8 { .. } => match rhs {
                    Self::Rgba8 { .. } => rhs,
                },
            };
        }
        let by = by as u16;
        let (src_r, src_g, src_b, src_a) = match self {
            Self::Rgba8 { r, g, b, a } => ((*r as u16), (*g as u16), (*b as u16), (*a as u16)),
        };
        let (dst_r, dst_g, dst_b, dst_a) = match self {
            Self::Rgba8 { r, g, b, a } => ((*r as u16), (*g as u16), (*b as u16), (*a as u16)),
        };
        let final_r = (u8::MAX as u16 - by) * src_r + (by * dst_r);
        let final_g = (u8::MAX as u16 - by) * src_g + (by * dst_g);
        let final_b = (u8::MAX as u16 - by) * src_b + (by * dst_b);
        let final_a = (u8::MAX as u16 - by) * src_a + (by * dst_a);
        match self {
            Self::Rgba8 { .. } => Self::Rgba8 {
                r: (final_r >> 8) as u8,
                g: (final_g >> 8) as u8,
                b: (final_b >> 8) as u8,
                a: (final_a >> 8) as u8,
            },
        }
    }
}
