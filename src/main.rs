use image_hasher::{HashAlg, HasherConfig};
use std::collections::HashMap;
use std::io;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut folder_path = String::new();
    io::stdin()
        .read_line(&mut folder_path)
        .expect("Failed to read line");
    folder_path = folder_path.trim().to_string();
    let mut hash_map: HashMap<String, Vec<std::path::PathBuf>> = HashMap::new();

    for entry in WalkDir::new(folder_path).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Ok(img) = image::open(path) {
                let hash = hash_image(&img);
                hash_map.entry(hash).or_default().push(path.to_path_buf());
            }
        }
    }

    for (hash, paths) in hash_map {
        if paths.len() > 1 {
            println!("Duplicate images found:");
            for path in paths {
                println!("{:?}", path);
            }
        }
    }

    Ok(())
}

fn hash_image(image: &image::DynamicImage) -> String {
    let hasher = HasherConfig::new().to_hasher();
    let hash = hasher.hash_image(image);
    hash.to_base64()
}
