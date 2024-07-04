mod transformations;
mod transformer;

fn main() {
    let start_time = std::time::Instant::now();

    let source_image = image::open("source.png").unwrap();
    let output_image = transformer::transform_image(source_image, None, |point| {
        transformations::square_to_circle::elliptical_grid(point)
    });

    output_image.save("output.png").unwrap();

    println!("Generated in: {}ms", start_time.elapsed().as_millis());
}
