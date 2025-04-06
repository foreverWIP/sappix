# Sappix

A 2D software rendering library in Rust. Currently incomplete and not fully tested! Don't use this yet.

Planned features include:
- Transparency and multiple color blending options for everything drawable
- Drawing of 2D shapes and primitives
- Drawing of sprites with rotozoom
- Data-based rendering setup, designed for persistent state outside the library and easy rendering to multiple surfaces
- Support for multiple pixel formats (besides RGBA8)
- Support for changing internal float type used in subpixel calculations (f32? f64? fixed point?)