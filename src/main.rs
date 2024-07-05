mod batch_runners;
mod transformations;
mod transformer;

fn main() {
    let source_image = image::open("source.png").unwrap();

    run_batches(
        source_image,
        transformer::Size {
            width: 2000,
            height: 2000,
        },
    );
}

fn run_batches(source_image: image::DynamicImage, batch_size: transformer::Size) {
    let start_time = std::time::Instant::now();
    batch_runners::run_all_circle_to_square(source_image.clone(), Some(batch_size));
    println!(
        "Generated circle to square batch in: {}ms",
        start_time.elapsed().as_millis()
    );

    let start_time = std::time::Instant::now();
    batch_runners::run_all_square_to_circle(source_image.clone(), Some(batch_size));
    println!(
        "Generated square to circle batch in: {}ms",
        start_time.elapsed().as_millis()
    );

    let start_time = std::time::Instant::now();
    batch_runners::run_all_half_face_superellipse(source_image.clone(), Some(batch_size));
    println!(
        "Generated half face superellipse batch in: {}ms",
        start_time.elapsed().as_millis()
    );
}
