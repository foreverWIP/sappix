#![no_std]
#![feature(test, random)]
extern crate test;

extern crate alloc;

#[cfg(debug_assertions)]
extern crate std;

mod color;
mod ffi;
mod line;
mod point;
mod rect;
mod renderer;
mod sprite;

use glam::I16Vec2;

pub use self::color::ColorMode;
pub use self::color::FBColor;
pub use self::line::Line;
pub use self::point::ColorVec2;
pub use self::rect::ColorRect;
pub use self::renderer::Renderer;
pub use self::sprite::Sprite;

pub trait Drawable {
    fn draw(&self, renderer: &mut Renderer);
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BlendMode {
    Opaque,
    Alpha,
}

pub type FBCoord = i16;
pub type FBAngle = i16;
pub type FBVec2 = I16Vec2;
pub type FBRange = u8;

pub(crate) fn bilinear_4_colors(
    x: f32,
    y: f32,
    top_left: FBColor,
    top_right: FBColor,
    bottom_left: FBColor,
    bottom_right: FBColor,
) -> FBColor {
    let top_linear = top_left.lerp(top_right, x);
    let bottom_linear = bottom_left.lerp(bottom_right, x);
    top_linear.lerp(bottom_linear, y)
}

pub(crate) fn blend_none(src: FBColor, dst: &mut FBColor) {
    *dst = src;
}

pub(crate) fn blend_alpha(src: FBColor, dst: &mut FBColor) {
    if src.a() >= 1.0 {
        *dst = src;
        return;
    }
    if src.a() <= 0.0 {
        return;
    }

    let src_r = src.r();
    let src_g = src.g();
    let src_b = src.b();
    let src_a = src.a();
    let dst_r = dst.r();
    let dst_g = dst.g();
    let dst_b = dst.b();
    let dst_a = dst.a();
    let r = src_r + dst_r * (1.0 - src_a);
    let g = src_g + dst_g * (1.0 - src_a);
    let b = src_b + dst_b * (1.0 - src_a);
    let a = src_a + dst_a * (1.0 - src_a);
    dst.set_r(r);
    dst.set_g(g);
    dst.set_b(b);
    dst.set_a(a);
}
