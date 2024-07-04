mod transformer;

fn main() {
    let source_image = image::open("source.png").unwrap();
    let output_image = transformer::transform_image(
        source_image,
        transformer::Size {
            width: 1000,
            height: 1000,
        },
        |point| point,
    );

    output_image.save("output.png").unwrap();
}
