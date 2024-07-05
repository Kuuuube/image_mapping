mod batch_runners;
mod transformations;
mod transformer;

fn main() {
    let start_time = std::time::Instant::now();

    let source_image = image::open("source.png").unwrap();

    run_batches(
        source_image,
        transformer::Size {
            width: 2000,
            height: 2000,
        },
    );

    println!("Generated in: {}ms", start_time.elapsed().as_millis());
}

fn run_batches(source_image: image::DynamicImage, batch_size: transformer::Size) {
    batch_runners::run_all_circle_to_square(source_image.clone(), Some(batch_size));
    batch_runners::run_all_square_to_circle(source_image.clone(), Some(batch_size));
    batch_runners::run_all_half_face_superellipse(source_image.clone(), Some(batch_size));
}
