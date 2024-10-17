use image::imageops::resize;
use image_compare::rgba_hybrid_compare;
use std::io;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut folder_path = String::new();
    let mut similarity_input = String::new();

    println!("Enter the folder path:");
    io::stdin()
        .read_line(&mut folder_path)
        .expect("Failed to read line");

    println!("Enter the similarity threshold (as a floating point number):");
    io::stdin()
        .read_line(&mut similarity_input)
        .expect("Failed to read line");

    let folder_path = folder_path.trim();
    let similarity_threshold: f64 = similarity_input
        .trim()
        .parse()
        .expect("Please enter a valid floating point number");

    let mut images = vec![];

    for entry in WalkDir::new(folder_path).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Ok(img) = image::open(path) {
                images.push((path.to_path_buf(), img.into_rgba8()));
            }
        }
    }

    for i in 0..images.len() {
        for j in (i + 1)..images.len() {
            let (ref path_one, ref image_one) = images[i];
            let (ref path_two, ref image_two) = images[j];

            let resized_image_two = resize(
                image_two,
                image_one.width(),
                image_one.height(),
                image::imageops::FilterType::Nearest,
            );
            if let Ok(result) = rgba_hybrid_compare(image_one, &resized_image_two) {
                if result.score > similarity_threshold {
                    println!(
                        "Images {:?} and {:?} are {:.2}% similar",
                        path_one,
                        path_two,
                        result.score * 100.0
                    );
                }
            }
        }
    }

    Ok(())
}
