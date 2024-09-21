use hide::image_to_string;

fn main() {
    let s = image_to_string("frames/qrcode-002.png").unwrap();
    let s2 = image_to_string("out/qrcode-1.png").unwrap();
    // println!("{s}");
    println!("{}", s == s2);
}
