use core::ops::Range;

use alloc::vec;
use alloc::vec::Vec;

use crate::{BlendMode, Drawable, FBColor, blend_alpha, blend_none};

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
    work_edges: Vec<Range<i16>>,
}
impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            fb: vec![FBColor::EMPTY; width as usize * height as usize],
            width,
            height,
            work_edges: vec![i16::MAX..i16::MIN; height as usize],
        }
    }

    pub fn width(&self) -> i16 {
        self.width as i16
    }

    pub fn height(&self) -> i16 {
        self.height as i16
    }

    pub fn fb(&self) -> &Vec<FBColor> {
        &self.fb
    }

    pub fn fb_rgba8(&self) -> Vec<u8> {
        self.fb.iter().flat_map(|fbc| fbc.to_rgba8()).collect()
    }

    pub(crate) fn edge_buffer(&self) -> &[Range<i16>] {
        &self.work_edges
    }

    pub(crate) fn edge_buffer_mut(&mut self) -> &mut [Range<i16>] {
        &mut self.work_edges
    }

    pub fn fill(&mut self, color: FBColor, blend_mode: BlendMode) {
        if blend_mode == BlendMode::Opaque {
            self.fb.fill(color);
            return;
        }
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x as i16, y as i16, color, blend_mode);
            }
        }
    }

    pub fn set(&mut self, x: i16, y: i16, color: FBColor, blend_mode: BlendMode) {
        if x < 0 || x >= self.width as i16 || y < 0 || y >= self.height as i16 {
            return;
        }
        let blend_func = get_blend_func(blend_mode);
        let idx = fb_idx!(self, x, y);
        blend_func(color, &mut self.fb[idx]);
    }

    pub fn set_unchecked(&mut self, x: i16, y: i16, color: FBColor, blend_mode: BlendMode) {
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
