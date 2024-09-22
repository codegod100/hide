use hide::render::render_to_mp4;
use std::{fs, path::PathBuf};
use tracing_subscriber;

fn get_files_as_paths(folder_path: &str) -> Vec<String> {
    fs::read_dir(folder_path)
        .expect("Failed to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .map(|path| path.to_str().unwrap().to_string())
        .collect()
}

fn main() {
    tracing_subscriber::fmt::init();
    let paths = get_files_as_paths("out");
    render_to_mp4(paths);
}
