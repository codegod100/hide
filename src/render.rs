use std::path::PathBuf;

use kantera::{
    ffmpeg::Exporter,
    image::Image,
    pixel::Rgba,
    render::{Render, RenderOpt},
    renders::image_render::ImageRender,
};

pub fn rgbas_to_u8s(block: &[Rgba], u8s: &mut [u8]) {
    for i in 0..block.len() {
        u8s[i * 4 + 2] = (block[i].0.min(1.0).max(0.0) * 255.99).floor() as u8;
        u8s[i * 4 + 1] = (block[i].1.min(1.0).max(0.0) * 255.99).floor() as u8;
        u8s[i * 4 + 0] = (block[i].2.min(1.0).max(0.0) * 255.99).floor() as u8;
        u8s[i * 4 + 3] = (block[i].3.min(1.0).max(0.0) * 255.99).floor() as u8;
    }
}

fn push_to_video(path: &str, len: usize, i: i32) -> Vec<kantera::pixel::Rgba> {
    let i = i + 1;
    let framerate = 20;
    let buffer_frame_num = 1;
    let img = image::open(path).unwrap();
    let height = img.height() as usize;
    let width = img.width() as usize;
    let mut buffer = vec![Rgba::default(); width * height * buffer_frame_num];
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
            framerate,
        },
        buffer.as_mut_slice(),
    );
    buffer
}
// https://github.com/carrotflakes/kantera/blob/main/examples/from_image_file.rs
pub fn render_to_mp4(paths: Vec<String>) {
    let framerate = 20;
    let width = 200;
    let height = 200;
    let len = paths.len();
    let mut exporter = Exporter::new(width, height, framerate, "automate.mp4");
    for (i, path) in paths.iter().enumerate() {
        let buffer = push_to_video(&path, len, i as i32);
        exporter.push(&buffer);
    }

    exporter.close();
}
