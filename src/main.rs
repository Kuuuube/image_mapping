mod transformer;

fn main() {
    let source_image = image::open("source.png").unwrap();
    let output_image = transformer::transform_image(
        source_image,
        Some(transformer::Size {
            width: 1000,
            height: 1000,
        }),
        |point| transformer::Point {
            x: point.x / 2.0,
            y: point.y / 2.0,
        },
    );

    output_image.save("output.png").unwrap();
}
