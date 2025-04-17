use crate::{BlendMode, ColorMode, Drawable, I16Vec2, bilinear_4_colors};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub position: I16Vec2,
    pub size: I16Vec2,
}

pub struct ColorRect {
    pub rect: Rect,
    pub color: ColorMode<4>,
    pub blend_mode: BlendMode,
}
impl ColorRect {
    pub fn new(rect: Rect, color: ColorMode<4>, blend_mode: BlendMode) -> Self {
        Self {
            rect,
            color,
            blend_mode,
        }
    }
}
impl Drawable for ColorRect {
    fn draw(&self, renderer: &mut crate::Renderer) {
        let single_color;
        let colors = match self.color {
            ColorMode::Solid(c) => {
                single_color = true;
                [c, c, c, c]
            }
            ColorMode::PerPoint(cs) => {
                single_color = false;
                cs
            }
        };

        let min_point = self.rect.position.min(self.rect.position + self.rect.size);
        let max_point = self.rect.position.max(self.rect.position + self.rect.size);

        let min_x = min_point.x.max(0);
        let max_x = max_point.x.min(renderer.width());
        let min_y = min_point.y.max(0);
        let max_y = max_point.y.min(renderer.height());

        let size = self.rect.size.abs();

        for y in min_y..max_y {
            for x in min_x..max_x {
                renderer.set(
                    x,
                    y,
                    if single_color {
                        colors[0]
                    } else {
                        let x = x as f32 / size.x as f32;
                        let y = y as f32 / size.y as f32;
                        bilinear_4_colors(x, y, colors[0], colors[1], colors[2], colors[3])
                    },
                    self.blend_mode,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Drawable, FBColor};
    extern crate std;

    use ::test::Bencher;
    use core::ffi::*;
    use std::random::Random;

    use crate::{BlendMode, I16Vec2, Renderer, ffi::*};

    use super::{ColorRect, Rect};

    #[bench]
    fn bmp_rect(bencher: &mut Bencher) {
        let b = test::black_box(unsafe { bm_create(128, 128) });
        let mut rand = std::random::DefaultRandomSource;
        bencher.iter(|| unsafe {
            bm_fillrect(
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
    fn our_rect(bencher: &mut Bencher) {
        let mut renderer = test::black_box(Renderer::new(128, 128));
        let mut rand = std::random::DefaultRandomSource;
        let mut rect = ColorRect::new(
            Rect {
                position: I16Vec2::ZERO,
                size: I16Vec2::ZERO,
            },
            crate::ColorMode::Solid(FBColor::WHITE),
            BlendMode::Opaque,
        );
        bencher.iter(|| {
            rect.rect.position.x = c_short::random(&mut rand);
            rect.rect.position.y = c_short::random(&mut rand);
            rect.rect.size.x = c_short::random(&mut rand);
            rect.rect.size.y = c_short::random(&mut rand);
            rect.draw(&mut renderer)
        })
    }
}
