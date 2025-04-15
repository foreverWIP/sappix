use crate::{BlendMode, ColorMode, Drawable, FBCoord, FBVec2};

pub struct ColorVec2 {
    pub position: FBVec2,
    pub color: ColorMode<1>,
    pub blend_mode: BlendMode,
}
impl ColorVec2 {
    pub const fn new(color: ColorMode<1>, blend_mode: BlendMode, x: FBCoord, y: FBCoord) -> Self {
        Self {
            color,
            blend_mode,
            position: FBVec2::new(x, y),
        }
    }
}
impl Drawable for ColorVec2 {
    fn draw(&self, renderer: &mut crate::Renderer) {
        if self.position.x < 0
            || self.position.y < 0
            || self.position.x >= renderer.width() as FBCoord
            || self.position.y >= renderer.height() as FBCoord
        {
            return;
        }
        let color = match self.color {
            ColorMode::Solid(c) => c,
            ColorMode::PerPoint([c]) => c,
        };
        renderer.set(self.position.x, self.position.y, color, self.blend_mode);
    }
}
