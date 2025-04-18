use alloc::{sync::Arc, vec::Vec};
use glam::{IVec2, U8Vec2, Vec2};

use crate::{BlendMode, ColorMode, Drawable, FBColor, I16Vec2, Renderer, bilinear_4_colors};

#[derive(Clone, Copy)]
pub struct SpriteFrame {
    offset: usize,
    size: U8Vec2,
}
impl SpriteFrame {
    pub fn new(offset: usize, width: u8, height: u8) -> Self {
        Self {
            offset,
            size: U8Vec2::new(width, height),
        }
    }
}

#[derive(Clone)]
pub enum SpriteFrameMode {
    StillImage(u8, u8),
    MultipleFrames(Vec<SpriteFrame>),
}

pub struct Sprite {
    pixels: Arc<Vec<FBColor>>,
    pub position: I16Vec2,
    pub rotation: i16,
    pub scale: u16,
    pub blend_mode: BlendMode,
    pub modulate: ColorMode<4>,
    frame_mode: SpriteFrameMode,
    cur_frame: usize,
}
impl Sprite {
    pub fn new(
        pixels: Arc<Vec<FBColor>>,
        position: I16Vec2,
        rotation: i16,
        scale: u16,
        blend_mode: BlendMode,
        modulate: ColorMode<4>,
        frame_mode: SpriteFrameMode,
    ) -> Self {
        Self {
            pixels,
            position,
            rotation,
            scale,
            blend_mode,
            modulate,
            frame_mode,
            cur_frame: 0,
        }
    }

    pub fn set_frame(&mut self, frame: usize) {
        self.cur_frame = frame;
    }

    fn size(&self) -> U8Vec2 {
        match &self.frame_mode {
            SpriteFrameMode::StillImage(w, h) => U8Vec2::new(*w, *h),
            SpriteFrameMode::MultipleFrames(sprite_frames) => sprite_frames[self.cur_frame].size,
        }
    }

    fn draw_rotozoom<const NO_ROTO: bool, const NO_ZOOM: bool>(
        &self,
        renderer: &mut Renderer,
        modulate_colors: &[FBColor; 4],
        single_color: bool,
    ) {
        if self.scale == 0 {
            return;
        }
        let angle = if NO_ROTO {
            0.0
        } else {
            ((self.rotation as f32 / 256.0) * 360.0).to_radians()
        };
        let scale = if NO_ZOOM {
            1.0
        } else {
            self.scale as f32 / 256.0
        };

        let size = self.size();
        let pixels_offset = match &self.frame_mode {
            SpriteFrameMode::StillImage(_, _) => 0,
            SpriteFrameMode::MultipleFrames(sprite_frames) => sprite_frames[self.cur_frame].offset,
        };

        let center = size.as_vec2() / 2.0;

        // crude approximation of drawing bounds
        // faster than scanning the entire framebuffer at least
        let diagonal_radius = ((size.x as f32).powi(2) + (size.y as f32).powi(2)).sqrt() / 2.0;

        let min_x = ((self.position.x as f32 - (diagonal_radius * scale)) as i16).max(0);
        let max_x =
            ((self.position.x as f32 + (diagonal_radius * scale)) as i16).min(renderer.width());
        let min_y = ((self.position.y as f32 - (diagonal_radius * scale)) as i16).max(0);
        let max_y =
            ((self.position.y as f32 + (diagonal_radius * scale)) as i16).min(renderer.height());

        let delta_col = Vec2::new(angle.sin(), angle.cos()) / scale;
        let delta_row = Vec2::new(delta_col.y, -delta_col.x);
        let delta_col_fixed = (delta_col * 65536.0).as_ivec2();
        let delta_row_fixed = (delta_row * 65536.0).as_ivec2();

        let start = Vec2::new(
            center.x
                - (self.position.x as f32 * delta_col.y + self.position.y as f32 * delta_col.x),
            center.y
                - (self.position.x as f32 * delta_row.y + self.position.y as f32 * delta_row.x),
        );

        let size_x_fixed = ((size.x as i32) << 16) as u32;
        let size_y_fixed = ((size.y as i32) << 16) as u32;

        /*
        we store uv as a fixed point vector instead of floating point
        the benefit of this is that instead of checking if 0 <= x and x < size,
        we can take advantage of two's complement negative being in the upper half of
        unsigned values. so we reduce four checks for every pixel to two casts + two checks,
        which runs faster than the c library this is benchmarked against
        (the casts are practically free since it's from signed -> unsigned int of same width)
        */
        {
            let mut row: IVec2 = ((start + delta_col * min_y as f32 + delta_row * min_x as f32)
                * 65536.0)
                .as_ivec2();

            for y in min_y..max_y {
                let mut uv = row;

                let mut x = min_x;
                while x < max_x {
                    if (uv.x as u32) < size_x_fixed && (uv.y as u32) < size_y_fixed {
                        break;
                    }
                    uv += delta_row_fixed;
                    x += 1;
                }
                loop {
                    if x == max_x {
                        break;
                    }

                    if (uv.x as u32) < size_x_fixed && (uv.y as u32) < size_y_fixed {
                        let c = self.pixels[pixels_offset
                            + ((uv.y >> 16) as usize * size.x as usize + (uv.x >> 16) as usize)]
                            * if single_color {
                                modulate_colors[0]
                            } else {
                                let x = (uv.x / size.x as i32) as f32 / 65536.0;
                                let y = (uv.y / size.y as i32) as f32 / 65536.0;
                                bilinear_4_colors(
                                    x,
                                    y,
                                    modulate_colors[0],
                                    modulate_colors[1],
                                    modulate_colors[2],
                                    modulate_colors[3],
                                )
                            };
                        renderer.set_unchecked(x, y, c, self.blend_mode);
                    } else {
                        break;
                        // renderer.set(x, y, FBColor::MAGENTA_RGBA8, BlendMode::Opaque);
                    }

                    uv += delta_row_fixed;

                    x += 1;
                }

                row += delta_col_fixed;
            }
        }
    }
}
impl Drawable for Sprite {
    fn draw(&self, renderer: &mut Renderer) {
        let (modulate_colors, single_color) = match self.modulate {
            ColorMode::Solid(c) => ([c, c, c, c], true),
            ColorMode::PerPoint(cs) => (cs, false),
        };
        match (self.rotation & 0xff, self.scale) {
            (_, 0) => {
                return;
            }
            (0, 0x100) => {
                self.draw_rotozoom::<true, true>(renderer, &modulate_colors, single_color);
            }
            (0, _) => {
                self.draw_rotozoom::<true, false>(renderer, &modulate_colors, single_color);
            }
            (_, 0x100) => {
                self.draw_rotozoom::<false, true>(renderer, &modulate_colors, single_color);
            }
            (_, _) => {
                self.draw_rotozoom::<false, false>(renderer, &modulate_colors, single_color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ColorMode, Drawable, FBColor};
    extern crate std;

    use ::test::Bencher;
    use alloc::{sync::Arc, vec::Vec};
    use core::ffi::*;
    use std::random::Random;

    use crate::{BlendMode, I16Vec2, Renderer, ffi::*};

    use super::{Sprite, SpriteFrameMode};

    const TEST_SPRITE_FILE: &[u8] = include_bytes!("../testimgs/test.gif");

    #[bench]
    fn bmp_rotozoom(bencher: &mut Bencher) {
        let src = test::black_box(unsafe {
            bm_load_mem(TEST_SPRITE_FILE.as_ptr(), TEST_SPRITE_FILE.len() as c_long)
        });
        let dst = test::black_box(unsafe { bm_create(128, 128) });
        let mut rand = std::random::DefaultRandomSource;
        bencher.iter(|| unsafe {
            let scale_whole = u8::random(&mut rand) as f64;
            let scale_frac = (u8::random(&mut rand) as f64 / (256.0 / 8.0)).fract();
            let mut scale = scale_whole + scale_frac;
            if scale == 0.0 {
                scale = 1.0;
            }
            bm_rotate_blit(
                dst,
                c_short::random(&mut rand) as i32 % 128,
                c_short::random(&mut rand) as i32 % 128,
                src,
                32,
                32,
                ((c_int::random(&mut rand) % 360) as f64).to_radians(),
                scale,
            )
        });
        unsafe {
            bm_free(src);
            bm_free(dst);
        }
    }

    #[bench]
    fn our_rotozoom(bencher: &mut Bencher) {
        let mut test_img = gif::Decoder::new(TEST_SPRITE_FILE).unwrap();
        let frame = test_img.read_next_frame().unwrap().unwrap().clone();
        let palette = test_img.palette().unwrap();
        let sprite_buf: Vec<_> = frame
            .buffer
            .iter()
            .map(|i| {
                FBColor::from_rgba8(
                    palette[*i as usize * 3 + 0],
                    palette[*i as usize * 3 + 1],
                    palette[*i as usize * 3 + 2],
                    0xff,
                )
            })
            .collect();

        let mut sprite = Sprite::new(
            Arc::new(sprite_buf),
            I16Vec2::splat(64),
            0,
            0x100,
            BlendMode::Opaque,
            ColorMode::Solid(FBColor::WHITE),
            SpriteFrameMode::StillImage(test_img.width() as u8, test_img.height() as u8),
        );

        let mut renderer = Renderer::new(128, 128);
        let mut rand = std::random::DefaultRandomSource;
        bencher.iter(|| {
            sprite.position.x = c_short::random(&mut rand) % renderer.width();
            sprite.position.y = c_short::random(&mut rand) % renderer.width();
            sprite.rotation = c_short::random(&mut rand);
            sprite.scale = c_ushort::random(&mut rand) & 0x7ff;
            sprite.draw(&mut renderer)
        })
    }
}
