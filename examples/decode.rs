use tracing_subscriber;
fn main() {
    tracing_subscriber::fmt::init();
    hide::decode("frames", "compiled.mp4").unwrap();
}
