#![no_std]
#![feature(test, random)]
extern crate test;

extern crate alloc;

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
pub use self::rect::Rect;
pub use self::renderer::Renderer;
pub use self::sprite::Sprite;

pub trait Drawable {
    fn draw(&mut self, renderer: &mut Renderer);
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BlendMode {
    Opaque,
    Alpha,
}

pub type FBCoord = i16;
pub type FBAngle = u8;
pub type FBVec2 = I16Vec2;
pub type FBRange = u8;

pub(crate) fn bilinear_4_colors(
    x: FBRange,
    y: FBRange,
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
    if src.a() == u8::MAX {
        *dst = src;
        return;
    }
    if src.a() == u8::MIN {
        return;
    }

    let src_r = src.r() as u16;
    let src_g = src.g() as u16;
    let src_b = src.b() as u16;
    let src_a = src.a() as u16;
    let dst_r = dst.r() as u16;
    let dst_g = dst.g() as u16;
    let dst_b = dst.b() as u16;
    let dst_a = dst.a() as u16;
    let r = src_r + dst_r * (u8::MAX as u16 - src_a);
    let g = src_g + dst_g * (u8::MAX as u16 - src_a);
    let b = src_b + dst_b * (u8::MAX as u16 - src_a);
    let a = src_a + dst_a * (u8::MAX as u16 - src_a);
    dst.set_r((r >> 8) as u8);
    dst.set_g((g >> 8) as u8);
    dst.set_b((b >> 8) as u8);
    dst.set_a((a >> 8) as u8);
}

#[cfg(test)]
mod tests {
    use crate::{Drawable, FBColor};
    extern crate std;

    use ::test::Bencher;
    use core::ffi::*;
    use std::random::Random;

    use crate::{BlendMode, Line, Renderer, ffi::*};

    #[bench]
    fn bmp_draw_line(bencher: &mut Bencher) {
        let b = test::black_box(unsafe { bm_create(128, 128) });
        let mut rand = std::random::DefaultRandomSource;
        bencher.iter(|| unsafe {
            bm_line(
                b,
                c_short::random(&mut rand) as i32,
                c_short::random(&mut rand) as i32,
                c_short::random(&mut rand) as i32,
                c_short::random(&mut rand) as i32,
            );
        });
        unsafe {
            bm_free(b);
        }
    }

    #[bench]
    fn our_draw_line(bencher: &mut Bencher) {
        let mut renderer = test::black_box(Renderer::new(128, 128));
        let mut rand = std::random::DefaultRandomSource;
        let mut line = Line::new(
            0,
            0,
            0,
            0,
            crate::ColorMode::Solid(FBColor::WHITE_RGBA8),
            BlendMode::Opaque,
        );
        bencher.iter(|| {
            line.a.x = c_short::random(&mut rand);
            line.a.y = c_short::random(&mut rand);
            line.b.x = c_short::random(&mut rand);
            line.b.y = c_short::random(&mut rand);
            line.draw(&mut renderer)
        })
    }
}
