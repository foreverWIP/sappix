use alloc::{sync::Arc, vec::Vec};
use glam::{U16Vec2, Vec2};

use crate::{
    BlendMode, ColorMode, Drawable, FBAngle, FBColor, FBCoord, FBVec2, Line, Rect,
    bilinear_4_colors,
};

pub struct Sprite {
    pixels: Arc<Vec<FBColor>>,
    size: U16Vec2,
    pub position: FBVec2,
    pub rotation: FBAngle,
    pub scale: u16,
    pub blend_mode: BlendMode,
    pub modulate: ColorMode<4>,
    pub clip_rect: Option<Rect>,
}
impl Sprite {
    pub fn new(
        pixels: Arc<Vec<FBColor>>,
        width: u16,
        height: u16,
        x: FBCoord,
        y: FBCoord,
        rotation: FBAngle,
        scale: u16,
        blend_mode: BlendMode,
        modulate: ColorMode<4>,
        clip_rect: Option<Rect>,
    ) -> Self {
        let mut ret = Self {
            pixels,
            size: U16Vec2::new(width, height),
            position: FBVec2::new(x, y),
            rotation,
            scale,
            blend_mode,
            modulate,
            clip_rect,
        };
        ret
    }

    fn calc_bounds(position: FBVec2, rotation: f32, size: U16Vec2, scale: f32) -> Rect {
        let angle = rotation.rem_euclid(360.0);
        let angle_rads = angle.to_radians();

        let width_2 = (size.x as f32) / 2.0;
        let height_2 = (size.y as f32) / 2.0;

        let angle_vec = Vec2::from_angle(angle_rads);
        let p1 = angle_vec.rotate(Vec2::new(-width_2, -height_2));
        let p2 = angle_vec.rotate(Vec2::new(width_2, -height_2));
        let p3 = angle_vec.rotate(Vec2::new(width_2, height_2));
        let p4 = angle_vec.rotate(Vec2::new(-width_2, height_2));

        let min_x = p1.x.min(p2.x).min(p3.x).min(p4.x);
        let min_y = p1.y.min(p2.y).min(p3.y).min(p4.y);
        let max_x = p1.x.max(p2.x).max(p3.x).max(p4.x);
        let max_y = p1.y.max(p2.y).max(p3.y).max(p4.y);

        let result_width = (max_x - min_x).abs().ceil() * scale;
        let result_height = (max_y - min_y).abs().ceil() * scale;

        let ret = Rect {
            position: position + FBVec2::new(min_x as FBCoord, min_y as FBCoord),
            size: FBVec2::new(result_width as FBCoord, result_height as FBCoord),
        };
        ret
    }

    fn draw_rotozoom(
        &mut self,
        renderer: &mut Option<&mut crate::Renderer>,
        modulate_colors: &[FBColor; 4],
        single_color: bool,
    ) {
        let angle = ((self.rotation as f32) / 255.0) * 360.0;
        let angle_rads = angle.to_radians();

        let width_float = self.size.x as f32;
        let height_float = self.size.y as f32;

        let sin = angle_rads.sin();
        let cos = angle_rads.cos();

        let bounds =
            Self::calc_bounds(self.position, angle, self.size, (self.scale as f32) / 256.0);
        let result_width = bounds.size.x;
        let result_height = bounds.size.y;
        let width_2 = result_width / 2;
        let height_2 = result_height / 2;

        let min_x = bounds.position.x;
        let min_y = bounds.position.y;
        let max_x = min_x + bounds.size.x;
        let max_y = min_y + bounds.size.y;

        for y in (min_y as usize)..(max_y as usize) {
            for x in (min_x as usize)..(max_x as usize) {
                let x = x as f32/*  - self.position.x*/;
                let y = y as f32/*  - self.position.y*/;
                let src_x = x;
                let src_y = y;
                if src_x >= 0.0 && src_x < width_float && src_y >= 0.0 && src_y < height_float {
                    let dst_x = (x as f32 * cos) - (y as f32 * sin);
                    let dst_y = (x as f32 * sin) + (y as f32 * cos);
                    match renderer {
                        Some(renderer) => {
                            let color =
                                self.pixels[src_y as usize * self.size.x as usize + src_x as usize];
                            renderer.set(
                                dst_x as FBCoord,
                                dst_y as FBCoord,
                                color,
                                self.blend_mode,
                            );
                        }
                        None => {}
                    }
                }
            }
        }

        if let Some(renderer) = renderer {
            Line::new(
                bounds.position.x,
                bounds.position.y,
                bounds.position.x + bounds.size.x,
                bounds.position.y + bounds.size.y,
                ColorMode::Solid(FBColor::GRAY50_RGBA8),
                BlendMode::Opaque,
            )
            .draw(renderer);
            Line::new(
                bounds.position.x + bounds.size.x,
                bounds.position.y,
                bounds.position.x,
                bounds.position.y + bounds.size.y,
                ColorMode::Solid(FBColor::GRAY50_RGBA8),
                BlendMode::Opaque,
            )
            .draw(renderer);
            renderer.set(
                (bounds.position.x + bounds.size.x / 2) as FBCoord,
                (bounds.position.y + bounds.size.y / 2) as FBCoord,
                FBColor::WHITE_RGBA8,
                BlendMode::Opaque,
            );
        }
    }
}
impl Drawable for Sprite {
    fn draw(&mut self, renderer: &mut crate::Renderer) {
        let (modulate_colors, single_color) = match self.modulate {
            ColorMode::Solid(c) => ([c, c, c, c], true),
            ColorMode::PerPoint(cs) => (cs, false),
        };
        match (self.rotation, self.scale) {
            (_, ..=0x100) => {
                return;
            }
            /*(0.0, 1.0) => {
                for y in 0..(self.size.y as FBCoordI) {
                    for x in 0..(self.size.x as FBCoordI) {
                        renderer.set(
                            self.position.x as FBCoordI + x,
                            self.position.y as FBCoordI + y,
                            self.pixels[(y as u32 * self.size.x as u32 + x as u32) as usize],
                            self.blend_mode,
                        );
                    }
                }
            }*/
            (_, _) => {
                self.draw_rotozoom(&mut Some(renderer), &modulate_colors, single_color);
            }
        }
    }
}
