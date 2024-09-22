use kantera::{
    image::Image,
    pixel::Rgba,
    render::{Render, RenderOpt},
    renders::image_render::ImageRender,
};
use tracing::info;

use crate::exporter::Exporter;
const FRAMERATE: usize = 100;

#[tracing::instrument]
fn pixel_buffer(path: &str, i: i32) -> Vec<kantera::pixel::Rgba> {
    let i = i + 1;
    let buffer_frame_num = 1;
    let img = image::open(path).unwrap();
    let height = img.height() as usize;
    let width = img.width() as usize;
    let mut buffer = vec![Rgba::default(); width * height * buffer_frame_num];

    if let None = img.as_rgb8() {
        info!("missing rgb8 data");
    }
    let buf = img.as_rgb8().unwrap();
    let img = Image {
        width: buf.width() as usize,
        height: buf.height() as usize,
        vec: buf
            .pixels()
            .map(|image::Rgb([r, g, b])| {
                Rgba(
                    *r as f64 / std::u8::MAX as f64,
                    *g as f64 / std::u8::MAX as f64,
                    *b as f64 / std::u8::MAX as f64,
                    1.0,
                )
            })
            .collect(),
    };

    let render = &ImageRender {
        image: Box::new(img),
    };
    render.render(
        &RenderOpt {
            u_range: 0.0..1.0,
            u_res: width,
            v_range: 0.0..1.0,
            v_res: height,
            frame_range: (i..i + 1),
            framerate: FRAMERATE,
        },
        buffer.as_mut_slice(),
    );
    buffer
}
// https://github.com/carrotflakes/kantera/blob/main/examples/from_image_file.rs
pub fn render_to_mp4(paths: Vec<String>) {
    let width = 1400;
    let height = 1400;
    let mut exporter = Exporter::new(width, height, FRAMERATE, "automate.mp4");
    for (i, path) in paths.iter().enumerate() {
        let buffer = pixel_buffer(&path, i as i32);
        exporter.push(&buffer);
    }

    exporter.close();
}
