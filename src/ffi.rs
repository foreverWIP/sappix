#![cfg(test)]
#![allow(unused)]

use core::ffi::{c_double, c_int, c_long, c_uchar, c_uint};

pub type Bitmap = *mut ();
pub type BMColor = c_uint;

unsafe extern "C" {
    pub fn bm_create(w: c_int, h: c_int) -> Bitmap;
    pub fn bm_set_color(bm: Bitmap, col: BMColor) -> ();
    pub fn bm_puts(b: Bitmap, x: c_int, y: c_int, text: *const c_uchar) -> c_int;
    pub fn bm_save(b: Bitmap, fname: *const c_uchar) -> c_int;
    pub fn bm_free(b: Bitmap) -> ();
    pub fn bm_atoi(text: *const c_uchar) -> BMColor;
    pub fn bm_line(b: Bitmap, x0: c_int, y0: c_int, x1: c_int, y1: c_int) -> ();
    pub fn bm_load(filename: *const c_uchar) -> Bitmap;
    pub fn bm_rotate_blit(
        dst: Bitmap,
        ox: c_int,
        oy: c_int,
        src: Bitmap,
        px: c_int,
        py: c_int,
        angle: c_double,
        scale: c_double,
    ) -> ();
    pub fn bm_width(b: Bitmap) -> c_int;
    pub fn bm_height(b: Bitmap) -> c_int;
    pub fn bm_load_mem(buffer: *const u8, len: c_long) -> Bitmap;
}
