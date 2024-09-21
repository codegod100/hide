use hide::image_to_string;

fn main() {
    let s = image_to_string("frames/frame-001.png").unwrap();
    let s2 = image_to_string("out/qrcode-0.png").unwrap();
    // println!("{s}");
    println!("{}", s == s2);
}
