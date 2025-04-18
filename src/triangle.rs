use glam::Vec2;

use crate::{BlendMode, ColorMode, Drawable, FBColor, I16Vec2};

fn edge(a: I16Vec2, b: I16Vec2, c: I16Vec2) -> i32 {
    (b.x as i32 - a.x as i32) * (c.y as i32 - a.y as i32)
        - (b.y as i32 - a.y as i32) * (c.x as i32 - a.x as i32)
}

pub struct Triangle {
    pub a: I16Vec2,
    pub b: I16Vec2,
    pub c: I16Vec2,
    pub color_mode: ColorMode<3>,
    pub blend_mode: BlendMode,
}
impl Triangle {
    pub fn new(
        a: I16Vec2,
        b: I16Vec2,
        c: I16Vec2,
        color_mode: ColorMode<3>,
        blend_mode: BlendMode,
    ) -> Self {
        Self {
            a,
            b,
            c,
            color_mode,
            blend_mode,
        }
    }
}
impl Drawable for Triangle {
    fn draw(&self, renderer: &mut crate::Renderer) {
        let (colors, single_color) = match self.color_mode {
            ColorMode::Solid(fbcolor) => ([fbcolor; 3], true),
            ColorMode::PerPoint(colors) => (colors, false),
        };

        let a = self.a;
        let b = self.b;
        let c = self.c;
        let self_edge = edge(a, b, c);
        let flip_sign = self_edge.is_negative();
        let self_edge = self_edge.abs() as f32;

        let min_x = a.x.min(b.x).min(c.x).max(0);
        let max_x = a.x.max(b.x).max(c.x).min(renderer.width());
        let min_y = a.y.min(b.y).min(c.y).max(0);
        let max_y = a.y.max(b.y).max(c.y).min(renderer.height());

        'yloop: for y in min_y..max_y {
            let mut start_x = min_x;
            loop {
                if start_x >= max_x {
                    continue 'yloop;
                }

                let mut edge_a = edge(a, b, I16Vec2::new(start_x, y));
                let mut edge_b = edge(b, c, I16Vec2::new(start_x, y));
                let mut edge_c = edge(c, a, I16Vec2::new(start_x, y));

                if flip_sign {
                    edge_a = -edge_a;
                    edge_b = -edge_b;
                    edge_c = -edge_c;
                }

                if edge_a >= 0 && edge_b >= 0 && edge_c >= 0 {
                    break;
                }

                start_x += 1;
            }
            for x in start_x..max_x {
                let mut edge_a = edge(a, b, I16Vec2::new(x, y));
                let mut edge_b = edge(b, c, I16Vec2::new(x, y));
                let mut edge_c = edge(c, a, I16Vec2::new(x, y));

                if flip_sign {
                    edge_a = -edge_a;
                    edge_b = -edge_b;
                    edge_c = -edge_c;
                }

                if edge_a >= 0 && edge_b >= 0 && edge_c >= 0 {
                    if single_color {
                        renderer.set(x, y, colors[0], self.blend_mode);
                    } else {
                        let color = FBColor::lerp3(
                            &colors,
                            &[
                                edge_b as f32 / self_edge as f32,
                                edge_c as f32 / self_edge as f32,
                                edge_a as f32 / self_edge as f32,
                            ],
                        );
                        renderer.set(x, y, color, self.blend_mode);
                    }
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ColorMode, Drawable, FBColor};
    extern crate std;

    use ::test::Bencher;
    use core::ffi::*;
    use std::random::Random;

    use crate::{BlendMode, I16Vec2, Renderer, ffi::*};

    use super::Triangle;

    #[bench]
    fn bmp_triangle(bencher: &mut Bencher) {
        let b = test::black_box(unsafe { bm_create(128, 128) });
        let mut rand = std::random::DefaultRandomSource;
        let points = &mut [BmPoint { x: 0, y: 0 }; 3];
        bencher.iter(|| unsafe {
            points[0].x = c_int::random(&mut rand).clamp(-32, 128 + 32);
            points[0].y = c_int::random(&mut rand).clamp(-32, 128 + 32);
            points[1].x = c_int::random(&mut rand).clamp(-32, 128 + 32);
            points[1].y = c_int::random(&mut rand).clamp(-32, 128 + 32);
            points[2].x = c_int::random(&mut rand).clamp(-32, 128 + 32);
            points[2].y = c_int::random(&mut rand).clamp(-32, 128 + 32);
            bm_fillpoly(b, points.as_ptr(), points.len() as c_uint)
        });
        unsafe {
            bm_free(b);
        }
    }

    #[bench]
    fn our_triangle(bencher: &mut Bencher) {
        let mut renderer = test::black_box(Renderer::new(128, 128));
        let mut rand = std::random::DefaultRandomSource;
        let mut tri = Triangle::new(
            I16Vec2::ZERO,
            I16Vec2::ZERO,
            I16Vec2::ZERO,
            ColorMode::Solid(FBColor::MAGENTA),
            BlendMode::Opaque,
        );
        bencher.iter(|| {
            tri.a.x = c_short::random(&mut rand).clamp(-32, 128 + 32);
            tri.a.y = c_short::random(&mut rand).clamp(-32, 128 + 32);
            tri.b.x = c_short::random(&mut rand).clamp(-32, 128 + 32);
            tri.b.y = c_short::random(&mut rand).clamp(-32, 128 + 32);
            tri.c.x = c_short::random(&mut rand).clamp(-32, 128 + 32);
            tri.c.y = c_short::random(&mut rand).clamp(-32, 128 + 32);
            tri.draw(&mut renderer);
        })
    }
}
