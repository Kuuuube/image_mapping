mod transformer;
mod transformations;

fn main() {
    let source_image = image::open("source.png").unwrap();
    let output_image = transformer::transform_image(
        source_image,
        None,
        |point| {
            transformations::square_to_circle::elliptical_grid(point)
        }
    );

    output_image.save("output.png").unwrap();
}
