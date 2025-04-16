use crate::{BlendMode, Drawable, FBColor, FBVec2};

pub struct Circle {
    pub position: FBVec2,
    pub radius: u16,
    pub fill: bool,
    pub color: FBColor,
    pub blend_mode: BlendMode,
}
impl Circle {
    pub fn new(
        x: i16,
        y: i16,
        radius: u16,
        fill: bool,
        color: FBColor,
        blend_mode: BlendMode,
    ) -> Self {
        Self {
            position: FBVec2::new(x, y),
            radius,
            fill,
            color,
            blend_mode,
        }
    }

    fn draw_internal<const FILL: bool>(&self, renderer: &mut crate::Renderer) {
        if self.radius == 0 {
            return;
        }

        let render_width = renderer.width() as u16;
        let render_height = renderer.height() as u16;

        let center_x = self.position.x;
        let center_y = self.position.y;
        let radius = self.radius as i16;

        let mut f = 1 - radius;
        let mut delta_x = 0;
        let mut delta_y = -2 * radius;
        let mut x = 0;
        let mut y = radius;

        let edge_buf = renderer.edge_buffer_mut();

        let mut min_y = i16::MAX;
        let mut max_y = i16::MIN;

        if (self.position.y as u16) < render_height {
            edge_buf[self.position.y as usize] =
                (self.position.x - radius)..(self.position.x + radius);
        }

        while x < y {
            if f >= 0 {
                y -= 1;
                delta_y += 2;
                f += delta_y;
            }
            x += 1;
            delta_x += 2;
            f += delta_x + 1;

            if ((center_y - y) as u16) < render_width {
                min_y = min_y.min(center_y - y);
                max_y = max_y.max(center_y - y);
                edge_buf[(center_y - y) as usize] = (center_x - x)..(center_x + x)
            }
            if ((center_y + y) as u16) < render_width {
                min_y = min_y.min(center_y + y);
                max_y = max_y.max(center_y + y);
                edge_buf[(center_y + y) as usize] = (center_x - x)..(center_x + x);
            }
            if ((center_y - x) as u16) < render_width {
                min_y = min_y.min(center_y - x);
                max_y = max_y.max(center_y - x);
                edge_buf[(center_y - x) as usize] = (center_x - y)..(center_x + y);
            }
            if ((center_y + x) as u16) < render_width {
                min_y = min_y.min(center_y + x);
                max_y = max_y.max(center_y + x);
                edge_buf[(center_y + x) as usize] = (center_x - y)..(center_x + y);
            }
        }

        if FILL {
            for y in min_y..max_y {
                let mut edge = renderer.edge_buffer()[y as usize].clone();
                edge.start = edge.start.clamp(0, renderer.width() - 1);
                edge.end = edge.end.clamp(0, renderer.width() - 1);

                for x in edge {
                    renderer.set(x, y, self.color, self.blend_mode);
                }
            }
        } else {
            for y in min_y..max_y {
                let edge = renderer.edge_buffer()[y as usize].clone();
                renderer.set(edge.start, y, self.color, self.blend_mode);
                renderer.set(edge.end, y, self.color, self.blend_mode);
            }
        }
    }
}
impl Drawable for Circle {
    fn draw(&self, renderer: &mut crate::Renderer) {
        if self.fill {
            self.draw_internal::<true>(renderer);
        } else {
            self.draw_internal::<false>(renderer);
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

    use crate::{BlendMode, Renderer, ffi::*};

    use super::Circle;

    #[bench]
    fn bmp_circle(bencher: &mut Bencher) {
        let b = test::black_box(unsafe { bm_create(128, 128) });
        let mut rand = std::random::DefaultRandomSource;
        bencher.iter(|| unsafe {
            bm_circle(
                b,
                c_short::random(&mut rand) as i32,
                c_short::random(&mut rand) as i32,
                (c_short::random(&mut rand) as i32).clamp(0, 512),
            );
        });
        unsafe {
            bm_free(b);
        }
    }

    #[bench]
    fn our_circle(bencher: &mut Bencher) {
        let mut renderer = test::black_box(Renderer::new(128, 128));
        let mut rand = std::random::DefaultRandomSource;
        let mut circle = Circle::new(0, 0, 0, false, FBColor::MAGENTA, BlendMode::Opaque);
        bencher.iter(|| {
            circle.position.x = c_short::random(&mut rand);
            circle.position.y = c_short::random(&mut rand);
            circle.radius = c_ushort::random(&mut rand).clamp(0, 512);
            circle.draw(&mut renderer)
        })
    }

    #[bench]
    fn bmp_fillcircle(bencher: &mut Bencher) {
        let b = test::black_box(unsafe { bm_create(128, 128) });
        let mut rand = std::random::DefaultRandomSource;
        bencher.iter(|| unsafe {
            bm_fillcircle(
                b,
                c_short::random(&mut rand) as i32,
                c_short::random(&mut rand) as i32,
                (c_short::random(&mut rand) as i32).clamp(0, 512),
            );
        });
        unsafe {
            bm_free(b);
        }
    }

    #[bench]
    fn our_fillcircle(bencher: &mut Bencher) {
        let mut renderer = test::black_box(Renderer::new(128, 128));
        let mut rand = std::random::DefaultRandomSource;
        let mut circle = Circle::new(0, 0, 0, true, FBColor::MAGENTA, BlendMode::Opaque);
        bencher.iter(|| {
            circle.position.x = c_short::random(&mut rand);
            circle.position.y = c_short::random(&mut rand);
            circle.radius = c_ushort::random(&mut rand);
            circle.draw(&mut renderer)
        })
    }
}
