use crate::{BlendMode, ColorMode, Drawable, FBCoord, FBVec2, Renderer};

pub struct Line {
    pub a: FBVec2,
    pub b: FBVec2,
    pub color: ColorMode<2>,
    pub blend_mode: BlendMode,
}
impl Line {
    pub const fn new(
        a_x: FBCoord,
        a_y: FBCoord,
        b_x: FBCoord,
        b_y: FBCoord,
        color: ColorMode<2>,
        blend_mode: BlendMode,
    ) -> Self {
        Self {
            a: FBVec2::new(a_x, a_y),
            b: FBVec2::new(b_x, b_y),
            color,
            blend_mode,
        }
    }
}
impl Drawable for Line {
    fn draw(&self, renderer: &mut crate::Renderer) {
        fn plot_line_common<const HIGH: bool>(
            renderer: &mut Renderer,
            x1: FBCoord,
            y1: FBCoord,
            x2: FBCoord,
            y2: FBCoord,
            color: ColorMode<2>,
            blend_mode: BlendMode,
        ) {
            if y1 == y2 {
                match color {
                    ColorMode::Solid(color) => {
                        for x in x1..x2 {
                            renderer.set(x, y1, color, blend_mode);
                        }
                    }
                    ColorMode::PerPoint([color_a, color_b]) => {
                        for x in x1..x2 {
                            let by = x as f32 / ((x2 - x1) as f32);
                            renderer.set(x, y1, color_a.lerp(color_b, by), blend_mode);
                        }
                    }
                }
            }

            if x1 == x2 {
                match color {
                    ColorMode::Solid(color) => {
                        for y in y1..y2 {
                            renderer.set(x1, y, color, blend_mode);
                        }
                    }
                    ColorMode::PerPoint([color_a, color_b]) => {
                        for y in y1..y2 {
                            let by = y as f32 / ((y2 - y1) as f32);
                            renderer.set(x1, y, color_a.lerp(color_b, by), blend_mode);
                        }
                    }
                }
            }

            let mut d_x = x2 - x1;
            let mut d_y = y2 - y1;
            let mut i = 1;
            if HIGH {
                if d_x < 0 {
                    i = -1;
                    d_x = -d_x;
                }
            } else {
                if d_x < 0 {
                    i = -1;
                    d_y = -d_y;
                }
            }
            let mut d = (if HIGH {
                (2 * d_x) - d_y
            } else {
                (2 * d_y) - d_x
            }) as i32;
            let mut x_or_y = if HIGH { x1 } else { y1 };
            if HIGH {
                match color {
                    ColorMode::Solid(color) => {
                        for y in y1..y2 {
                            renderer.set(x_or_y, y, color, blend_mode);
                            if d > 0 {
                                x_or_y += i;
                                d += (2 * (d_x - d_y)) as i32;
                            } else {
                                d += (2 * d_x) as i32;
                            }
                        }
                    }
                    ColorMode::PerPoint([color_a, color_b]) => {
                        for y in y1..y2 {
                            let by = y as f32 / ((y2 - y1) as f32);
                            renderer.set(x_or_y, y, color_a.lerp(color_b, by), blend_mode);
                            if d > 0 {
                                x_or_y += i;
                                d += (2 * (d_x - d_y)) as i32;
                            } else {
                                d += (2 * d_x) as i32;
                            }
                        }
                    }
                }
            } else {
                match color {
                    ColorMode::Solid(color) => {
                        for x in x1..x2 {
                            renderer.set(x, x_or_y, color, blend_mode);
                            if d > 0 {
                                x_or_y += i;
                                d += (2 * (d_y - d_x)) as i32;
                            } else {
                                d += (2 * d_y) as i32;
                            }
                        }
                    }
                    ColorMode::PerPoint([color_a, color_b]) => {
                        for x in x1..x2 {
                            let by = x as f32 / ((x2 - x1) as f32);
                            renderer.set(x, x_or_y, color_a.lerp(color_b, by), blend_mode);
                            if d > 0 {
                                x_or_y += i;
                                d += (2 * (d_y - d_x)) as i32;
                            } else {
                                d += (2 * d_y) as i32;
                            }
                        }
                    }
                }
            }
        }

        if (self.a.x < 0 && self.b.x < 0)
            || (self.a.y < 0 && self.b.y < 0)
            || (self.a.x < renderer.width() && self.b.x < renderer.width())
            || (self.a.y < renderer.height() && self.b.y < renderer.height())
        {
            return;
        }

        if (self.a.y - self.b.y).abs() < (self.a.x - self.b.x).abs() {
            if self.a.x > self.b.x {
                plot_line_common::<false>(
                    renderer,
                    self.b.x,
                    self.b.y,
                    self.a.x,
                    self.a.y,
                    self.color,
                    self.blend_mode,
                );
            } else {
                plot_line_common::<false>(
                    renderer,
                    self.a.x,
                    self.a.y,
                    self.b.x,
                    self.b.y,
                    self.color,
                    self.blend_mode,
                );
            }
        } else {
            if self.a.y > self.b.y {
                plot_line_common::<true>(
                    renderer,
                    self.b.x,
                    self.b.y,
                    self.a.x,
                    self.a.y,
                    self.color,
                    self.blend_mode,
                );
            } else {
                plot_line_common::<true>(
                    renderer,
                    self.a.x,
                    self.a.y,
                    self.b.x,
                    self.b.y,
                    self.color,
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
