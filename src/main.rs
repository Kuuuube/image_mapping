mod transformer;

fn main() {
    let source_image = image::open("source.png").unwrap();
    let output_image = transformer::transform_image(source_image, |point| point);

    output_image.save("output.png").unwrap();
}
