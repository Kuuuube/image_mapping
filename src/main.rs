mod transformer;

fn main() {
    let source_image = image::open("source.png").unwrap();
    let output_image = transformer::transform_image(
        source_image,
        None,
        |point| {
            // Elliptical Grid Mapping Inverse (Square -> Circle)
            transformer::Point {
                x: point.x * f64::sqrt(1.0 - (f64::powi(point.y, 2) / 2.0)),
                y: point.y * f64::sqrt(1.0 - (f64::powi(point.x, 2) / 2.0)),
            }
        }
    );

    output_image.save("output.png").unwrap();
}
