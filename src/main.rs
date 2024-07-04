fn main() {
    let source_image = image::open("source.png").unwrap().as_rgb8().unwrap().to_owned();

    let (source_width, source_height) = (source_image.width(), source_image.height());

    let mut output_image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(source_width, source_height);

    for (x, y, rgb) in source_image.enumerate_pixels() {
        let (unit_x, unit_y) = ((x + (source_width / 2)) as f32 / source_width as f32, (y + (source_height / 2)) as f32 / source_height as f32);
        let (transform_x, transform_y) = (unit_x / 2.0, unit_y / 2.0);
        let (real_x, real_y) = ((transform_x * source_width as f32) as u32, (transform_y * source_height as f32) as u32);

        let output_pixel = output_image.get_pixel_mut(real_x, real_y);
        *output_pixel = image::Rgb([*rgb.0.get(0).unwrap(), *rgb.0.get(1).unwrap(), *rgb.0.get(2).unwrap()]);
    }

    output_image.save("output.png").unwrap();
}
