use std::{
    error::Error,
    fs::{self, File},
    io::Read,
};

use bardecoder::{
    decode::{Decode, QRDecoder},
    util::qr::{QRData, QRError},
    Decoder,
};
use image::{GenericImageView, Luma};
use qrcode::QrCode;
use rxing::{
    common::{AdaptiveThresholdBinarizer, GlobalHistogramBinarizer, HybridBinarizer},
    qrcode::QRCodeReader,
    BinaryBitmap, BufferedImageLuminanceSource, Luma8LuminanceSource, RGBLuminanceSource, Reader,
};
use threadpool::ThreadPool;

pub fn encode(filename: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_dir_all("out").ok(); // The .ok() ignores errors if the directory doesn't exist
    fs::create_dir_all("out")?;

    let (tx, rx) = flume::unbounded();
    let mut buf: Vec<u8> = Vec::new();
    File::open(filename).unwrap().read_to_end(&mut buf)?;
    // Encode some data into bits.
    let chunks: Vec<Vec<u8>> = buf.chunks(2_000).map(|chunk| chunk.to_vec()).collect();
    let pool = ThreadPool::new(8);
    println!("{}", chunks.len());
    let len = chunks.len();

    for (i, chunk) in chunks.into_iter().enumerate() {
        let tx = tx.clone();
        pool.execute(move || {
            println!("{}/{}", i, len);
            let code =
                QrCode::with_version(&chunk, qrcode::Version::Normal(40), qrcode::EcLevel::M)
                    .unwrap();

            // Render the bits into an image.
            let image = code.render::<Luma<u8>>().build();
            let p = format!("out/qrcode-{}.png", i);
            // Save the image.
            image.save(p).unwrap();
            tx.send(1).unwrap();
        });
    }
    drop(tx);
    rx.iter().count();

    Ok(())
}

pub fn decode(frames_path: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let entries: Vec<_> = fs::read_dir(frames_path)?.collect::<Result<_, _>>()?;
    let len = entries.len();

    let mut bytes: Vec<u8> = vec![];

    for (i, entry) in entries.into_iter().enumerate() {
        println!("{}/{}", i, len);
        // let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("png") {
            // let mut buf: Vec<u8> = vec![];
            println!("opening file");
            let img = image::open(path)?.to_rgb8();

            let pixels: Vec<u32> = img
                .pixels()
                .map(|p| {
                    let [r, g, b] = p.0;
                    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
                })
                .collect();

            println!("lumanizing");
            // let img_gray = img.into_luma8();
            let ls = RGBLuminanceSource::new_with_width_height_pixels(
                img.width() as usize,
                img.height() as usize,
                &pixels,
            );
            println!("binarizing");
            let hb = HybridBinarizer::new(ls);
            let mut bb = BinaryBitmap::new(hb);
            let mut decoder = QRCodeReader::new();
            println!("decoding");
            let decoded = decoder.decode(&mut bb)?;
            println!("getting raw bytes");
            let result = decoded.getRawBytes();

            bytes.extend(result)
        }
    }
    fs::write(filename, bytes)?;

    Ok(())
}
