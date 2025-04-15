use std::fs::File;
use std::sync::Arc;

use gif::ExtensionData;
use sappix::BlendMode;
use sappix::ColorMode;
use sappix::Drawable;
use sappix::FBColor;
use sappix::Renderer;
use sappix::Sprite;

const TEST_SPRITE_FILE: &[u8] = include_bytes!("../testimgs/space.gif");

fn main() {
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
        test_img.width() as u16,
        test_img.height() as u16,
        256,
        256,
        0,
        0x100,
        BlendMode::Opaque,
        // ColorMode::Solid(FBColor::WHITE_RGBA8),
        ColorMode::PerPoint([
            FBColor::MAGENTA_RGBA8,
            FBColor::WHITE_RGBA8,
            FBColor::GRAY50_RGBA8,
            FBColor::YELLOW_RGBA8,
        ]),
        None,
    );

    let mut renderer = Renderer::new(512, 512);

    let image = File::create("test.gif").unwrap();
    let mut encoder = gif::Encoder::new(
        image,
        renderer.width() as u16,
        renderer.height() as u16,
        test_img.global_palette().unwrap(),
    )
    .unwrap();
    encoder
        .write_extension(ExtensionData::new_control_ext(
            1,
            gif::DisposalMethod::Background,
            false,
            Some(0),
        ))
        .unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();
    const FRAMECOUNT: usize = 60;
    for i in 0..FRAMECOUNT {
        println!("rendering frame {}", i);
        renderer.fill(FBColor::GRAY50_RGBA8, BlendMode::Opaque);
        sprite.rotation = ((i as f32 / FRAMECOUNT as f32) * 256.0) as i16;
        sprite.scale = (0x200 as f32
            + 0x100 as f32 * ((i as f32 / FRAMECOUNT as f32) * 360.0).to_radians().sin())
            as u16;
        sprite.draw(&mut renderer);

        let mut fb: Vec<_> = renderer.fb_rgba8();
        encoder
            .write_frame(&gif::Frame::from_rgba(
                renderer.width() as u16,
                renderer.height() as u16,
                &mut fb,
            ))
            .unwrap();
    }
}
