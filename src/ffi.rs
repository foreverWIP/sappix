#![cfg(test)]

use core::ffi::{c_int, c_uchar, c_uint};

pub type Bitmap = *mut ();
pub type bm_color_t = c_uint;

unsafe extern "C" {
    pub fn bm_create(w: c_int, h: c_int) -> Bitmap;
    pub fn bm_set_color(bm: Bitmap, col: bm_color_t) -> ();
    pub fn bm_puts(b: Bitmap, x: c_int, y: c_int, text: *const c_uchar) -> c_int;
    pub fn bm_save(b: Bitmap, fname: *const c_uchar) -> c_int;
    pub fn bm_free(b: Bitmap) -> ();
    pub fn bm_atoi(text: *const c_uchar) -> bm_color_t;
    pub fn bm_line(b: Bitmap, x0: c_int, y0: c_int, x1: c_int, y1: c_int) -> ();
}
