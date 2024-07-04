use transformer::Size;

mod batch_runners;
mod transformations;
mod transformer;

fn main() {
    let start_time = std::time::Instant::now();

    let source_image = image::open("source.png").unwrap();
    let batch_size = Size {width: 1000, height: 1000};
    batch_runners::run_all_circle_to_square(source_image.clone(), Some(batch_size));
    batch_runners::run_all_square_to_circle(source_image.clone(), Some(batch_size));

    println!("Generated in: {}ms", start_time.elapsed().as_millis());
}
