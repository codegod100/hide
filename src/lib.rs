use bardecoder::{
    decode::{Decode, QRDecoder},
    util::qr::{QRData, QRError},
    Decoder,
};
use base64::prelude::*;
use base64::Engine;
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma};
use qrcode::{render::Pixel, EcLevel, QrCode};
use rxing::{
    common::{AdaptiveThresholdBinarizer, CharacterSet, GlobalHistogramBinarizer, HybridBinarizer},
    helpers::detect_in_file,
    qrcode::{QRCodeReader, QRCodeWriter},
    BarcodeFormat, BinaryBitmap, BufferedImageLuminanceSource, Luma8LuminanceSource,
    RGBLuminanceSource, Reader, Writer,
};
use std::cmp::Ordering;
use std::{
    error::Error,
    fs::{self, DirEntry, File},
    io::Read,
};
use threadpool::ThreadPool;
use tracing::{debug, error, info, span, warn, Level};
// pub mod ffmpeg;
pub mod render;

pub fn encode_with_qr(data: &str) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Box<dyn Error>> {
    // Encode some data into bits.
    // let code = QrCode::new(data)?;
    let code = QrCode::with_error_correction_level(data, EcLevel::L).unwrap();
    // Render the bits into an image.
    Ok(code.render::<Luma<u8>>().build())
}

pub fn encode(filename: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_dir_all("out").ok(); // The .ok() ignores errors if the directory doesn't exist
    fs::create_dir_all("out")?;

    // let (tx, rx) = flume::unbounded();
    let mut buf: Vec<u8> = Vec::new();
    File::open(filename).unwrap().read_to_end(&mut buf)?;
    // Encode some data into bits.
    let based = BASE64_STANDARD.encode(buf);
    let chunks: Vec<Vec<u8>> = based
        .as_bytes()
        .chunks(2900)
        .map(|chunk| chunk.to_vec())
        .collect();
    let pool = ThreadPool::new(8);
    // info!("{}", chunks.len());
    let len = chunks.len();
    for (i, chunk) in chunks.into_iter().enumerate() {
        pool.execute(move || {
            info!("{}/{}", i + 1, len);
            // let cs = CharacterSet::get_character_set_by_name("Binary").unwrap();
            info!("encoding");
            let encoded = String::from_utf8(chunk.clone()).unwrap();
            info!("writing file sum");
            fs::write(format!("sums/{i}.sum"), &encoded).unwrap();
            let writer = QRCodeWriter::default();
            let matrix = writer
                .encode(&encoded, &BarcodeFormat::QR_CODE, 200, 200)
                .unwrap();
            let image: DynamicImage = matrix.into();
            // let image = encode_with_qr(&encoded).unwrap();
            let p = format!("out/qrcode-{:04}.png", i);
            info!("writing: {p}");
            image.save(p).unwrap();
            // let tx = tx.clone();

            // tx.send(1).unwrap();
            // Ok(())
        });
    }
    pool.join();
    // drop(tx);
    // rx.iter().count();

    Ok(())
}

#[tracing::instrument]
pub fn image_to_string(path: &str) -> Result<String, Box<dyn Error>> {
    info!("opening file");
    // let img = image::open(path)?.to_rgb8();
    let result = detect_in_file(path, Some(BarcodeFormat::QR_CODE))?;

    info!("getting raw bytes");
    // let result = decoded.getRawBytes();

    Ok(result.getText().to_string())
}

pub fn decode(frames_path: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut entries: Vec<_> = fs::read_dir(frames_path)?.collect::<Result<_, _>>()?;
    entries.sort_by(|a, b| {
        let extract_number = |entry: &fs::DirEntry| -> u32 {
            entry
                .file_name()
                .to_str()
                .and_then(|s| s.strip_prefix("qrcode-"))
                .and_then(|s| s.strip_suffix(".png"))
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap()
        };

        let a_num = extract_number(a);
        let b_num = extract_number(b);

        a_num.cmp(&b_num)
    });

    let len = entries.len();

    let mut base = "".to_string();

    for (i, entry) in entries.into_iter().enumerate() {
        info!("{:?} {}/{}", entry, i + 1, len);
        // let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("png") {
            // let mut buf: Vec<u8> = vec![];
            info!("image to string");
            let s = image_to_string(path.to_str().unwrap())?;
            info!("writing string");
            base.push_str(&s);
            info!("checking sum");
            let sum = fs::read_to_string(format!("sums/{i}.sum"))?;
            if sum != s {
                panic!("sums don't match for {i}:\n\n{}\n\n{}", sum, s)
            }
        }
    }
    // info!("base64 string: {}", String::from_utf8(bytes.clone())?);
    info!("decoding base64");
    let decoded = BASE64_STANDARD.decode(&base)?;
    info!("writing file");
    fs::write(filename, decoded)?;

    Ok(())
}
