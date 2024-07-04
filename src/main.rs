fn main() {
    let source_image = image::open("source.png").unwrap();
    let output_image = process_image(source_image);

    output_image.save("output.png").unwrap();
}

fn transform(pixel_coords: Point) -> Point {
    return pixel_coords;
}

fn process_image(image: image::DynamicImage) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let source_image = image.into_rgba8();
    let source_size = Size {
        width: source_image.width(),
        height: source_image.height(),
    };

    let output_size = Size {
        width: 1000,
        height: 1000,
    };
    let mut output_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::new(output_size.width, output_size.height);

    for (x, y, rgba) in source_image.enumerate_pixels() {
        let unit_point = Point {
            x: x as f64 / source_size.width as f64 * 2.0 - 1.0,
            y: y as f64 / source_size.height as f64 * 2.0 - 1.0,
        };
        let transformed_point = transform(unit_point);

        if transformed_point.x > 1.0
            || transformed_point.x < -1.0
            || transformed_point.y > 1.0
            || transformed_point.y < -1.0
        {
            continue;
        }

        let (real_x, real_y) = (
            ((transformed_point.x + 1.0) / 2.0 * output_size.width as f64) as u32,
            ((transformed_point.x + 1.0) / 2.0 * output_size.height as f64) as u32,
        );

        let output_pixel = output_image.get_pixel_mut(real_x, real_y);
        *output_pixel = rgba.clone();
    }
    return output_image;
}

struct Point {
    x: f64,
    y: f64,
}

struct Size {
    width: u32,
    height: u32,
}
