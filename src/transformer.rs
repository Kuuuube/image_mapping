pub fn transform_image<F>(
    image: image::DynamicImage,
    output_size: Size,
    mut transformation: F,
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
where
    F: FnMut(Point) -> Point,
{
    let source_image = image.into_rgba8();
    let source_size = Size {
        width: source_image.width(),
        height: source_image.height(),
    };

    let mut output_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::new(output_size.clone().width, output_size.clone().height);

    for (x, y, rgba) in source_image.enumerate_pixels() {
        let unit_point = to_unit(
            Point {
                x: x.into(),
                y: y.into(),
            },
            source_size.clone(),
        );

        let transformed_point = transformation(unit_point);

        if transformed_point.x > 1.0
            || transformed_point.x < -1.0
            || transformed_point.y > 1.0
            || transformed_point.y < -1.0
        {
            continue;
        }

        let real_point = from_unit(transformed_point, output_size.clone());

        let output_pixel = output_image.get_pixel_mut(real_point.x, real_point.y);
        *output_pixel = rgba.clone();
    }

    return output_image;
}

fn to_unit(point: Point, input_size: Size) -> Point {
    return Point {
        x: point.x as f64 / input_size.width as f64 * 2.0 - 1.0,
        y: point.y as f64 / input_size.height as f64 * 2.0 - 1.0,
    };
}

fn from_unit(point: Point, output_size: Size) -> UPoint {
    return UPoint {
        x: ((point.x + 1.0) / 2.0 * output_size.width as f64) as u32,
        y: ((point.y + 1.0) / 2.0 * output_size.height as f64) as u32,
    };
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

struct UPoint {
    x: u32,
    y: u32,
}

#[derive(Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}
