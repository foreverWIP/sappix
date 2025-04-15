use alloc::vec;
use alloc::vec::Vec;

use crate::{BlendMode, Drawable, FBColor, FBCoord, blend_alpha, blend_none};

macro_rules! fb_idx {
    ($renderer:expr, $x:expr, $y:expr) => {
        ($y as u32 * $renderer.width() as u32 + $x as u32) as usize
    };
}

type BlendFunc = fn(src: FBColor, dst: &mut FBColor);

const fn get_blend_func(blend_mode: BlendMode) -> BlendFunc {
    match blend_mode {
        BlendMode::Opaque => blend_none,
        BlendMode::Alpha => blend_alpha,
    }
}

#[derive(Clone)]
pub struct Renderer {
    fb: Vec<FBColor>,
    width: u16,
    height: u16,
}
impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            fb: vec![FBColor::EMPTY_RGBA8; width as usize * height as usize],
            width,
            height,
        }
    }

    pub fn width(&self) -> FBCoord {
        self.width as FBCoord
    }

    pub fn height(&self) -> FBCoord {
        self.height as FBCoord
    }

    pub fn fb(&self) -> &Vec<FBColor> {
        &self.fb
    }

    pub fn fb_rgba8(&self) -> Vec<u8> {
        self.fb.iter().flat_map(|fbc| fbc.to_rgba8()).collect()
    }

    pub fn fill(&mut self, color: FBColor, blend_mode: BlendMode) {
        if blend_mode == BlendMode::Opaque {
            self.fb.fill(color);
            return;
        }
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x as FBCoord, y as FBCoord, color, blend_mode);
            }
        }
    }

    pub fn set(&mut self, x: FBCoord, y: FBCoord, color: FBColor, blend_mode: BlendMode) {
        if x < 0 || x >= self.width as FBCoord || y < 0 || y >= self.height as FBCoord {
            return;
        }
        let blend_func = get_blend_func(blend_mode);
        let idx = fb_idx!(self, x, y);
        blend_func(color, &mut self.fb[idx]);
    }

    pub fn set_unchecked(&mut self, x: FBCoord, y: FBCoord, color: FBColor, blend_mode: BlendMode) {
        let blend_func = get_blend_func(blend_mode);
        let idx = fb_idx!(self, x, y);
        blend_func(color, &mut self.fb[idx]);
    }

    pub fn draw(&mut self, drawable: &dyn Drawable) {
        drawable.draw(self);
    }

    pub fn draw_multiple(&mut self, drawables: &[&dyn Drawable]) {
        for drawable in drawables {
            self.draw(*drawable);
        }
    }
}
