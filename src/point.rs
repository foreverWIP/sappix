use crate::{BlendMode, Drawable, FBColor, I16Vec2};

pub struct ColorVec2 {
    pub position: I16Vec2,
    pub color: FBColor,
    pub blend_mode: BlendMode,
}
impl ColorVec2 {
    pub const fn new(position: I16Vec2, color: FBColor, blend_mode: BlendMode) -> Self {
        Self {
            position,
            color,
            blend_mode,
        }
    }
}
impl Drawable for ColorVec2 {
    fn draw(&self, renderer: &mut crate::Renderer) {
        renderer.set(
            self.position.x,
            self.position.y,
            self.color,
            self.blend_mode,
        );
    }
}
