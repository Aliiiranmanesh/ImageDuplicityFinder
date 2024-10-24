use image::imageops::resize;
use image_compare::rgba_hybrid_compare;
use rayon::prelude::*;
use std::io;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut folder_path = String::new();
    let mut similarity_input = String::new();

    println!("Enter the folder path:");
    io::stdin()
        .read_line(&mut folder_path)
        .expect("Failed to read line");

    println!("Enter the similarity threshold (as a percentage):");
    io::stdin()
        .read_line(&mut similarity_input)
        .expect("Failed to read line");

    let folder_path = folder_path.trim();
    let mut similarity_threshold: f64 = similarity_input
        .trim()
        .parse()
        .expect("Please enter a valid floating point number");
    similarity_threshold /= 100.0;

    let mut images = vec![];

    for entry in WalkDir::new(folder_path).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Ok(img) = image::open(path) {
                images.push((path.to_path_buf(), img.into_rgba8()));
            }
        }
    }

    images
        .par_iter()
        .enumerate()
        .for_each(|(i, (path_one, image_one))| {
            images.iter().skip(i + 1).for_each(|(path_two, image_two)| {
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
            });
        });

    Ok(())
}