use crate::FBVec2;
use crate::{BlendMode, ColorMode, Drawable, bilinear_4_colors};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub position: FBVec2,
    pub size: FBVec2,
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
        if self.rect.size.x <= 0 || self.rect.size.y <= 0 {
            return;
        }
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

        for y in 0..self.rect.size.y {
            for x in 0..self.rect.size.x {
                renderer.set(
                    self.rect.position.x + x,
                    self.rect.position.y + y,
                    if single_color {
                        colors[0]
                    } else {
                        let x = x as f32 / self.rect.size.x as f32;
                        let y = y as f32 / self.rect.size.y as f32;
                        bilinear_4_colors(x, y, colors[0], colors[1], colors[2], colors[3])
                    },
                    self.blend_mode,
                );
            }
        }
    }
}
