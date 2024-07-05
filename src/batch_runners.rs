use crate::{
    transformations,
    transformer::{self, Point, Size},
};

fn basic_runner_wrapper<P, Container, F>(
    mapping_name: &str,
    source_image: &image::ImageBuffer<P, Container>,
    output_size_option: Option<Size>,
    mut transformation: F,
) where
    P: image::Pixel + image::PixelWithColorType,
    [P::Subpixel]: image::EncodableLayout,
    Container: std::ops::Deref<Target = [P::Subpixel]>,
    F: FnMut(Point) -> Point,
{
    transformer::transform_image(source_image, output_size_option, |point| {
        transformation(point)
    })
    .save(&format!("output/{}.png", mapping_name))
    .expect(&format!("Failed to generate {}", mapping_name));
}

fn tertiary_runner_wrapper<P, Container, F>(
    mapping_name: &str,
    source_image: &image::ImageBuffer<P, Container>,
    output_size_option: Option<Size>,
    mut transformation: F,
    tertiary_value: f64,
) where
    P: image::Pixel + image::PixelWithColorType,
    [P::Subpixel]: image::EncodableLayout,
    Container: std::ops::Deref<Target = [P::Subpixel]>,
    F: FnMut(Point, f64) -> Point,
{
    transformer::transform_image(source_image, output_size_option, |point| {
        transformation(point, tertiary_value)
    })
    .save(&format!("output/{}.png", mapping_name))
    .expect(&format!("Failed to generate {}", mapping_name));
}

fn quaternary_runner_wrapper<P, Container, F>(
    mapping_name: &str,
    source_image: &image::ImageBuffer<P, Container>,
    output_size_option: Option<Size>,
    mut transformation: F,
    tertiary_value: f64,
    quaternary_value: f64,
) where
    P: image::Pixel + image::PixelWithColorType,
    [P::Subpixel]: image::EncodableLayout,
    Container: std::ops::Deref<Target = [P::Subpixel]>,
    F: FnMut(Point, f64, f64) -> Point,
{
    transformer::transform_image(source_image, output_size_option, |point| {
        transformation(point, tertiary_value, quaternary_value)
    })
    .save(&format!("output/{}.png", mapping_name))
    .expect(&format!("Failed to generate {}", mapping_name));
}

pub fn run_all_square_to_circle<P, Container>(
    source_image: &image::ImageBuffer<P, Container>,
    output_size_option: Option<Size>,
) where
    P: image::Pixel + image::PixelWithColorType,
    [P::Subpixel]: image::EncodableLayout,
    Container: std::ops::Deref<Target = [P::Subpixel]>,
{
    let tertiary_steps = step_maker(100);

    //FG-Squircular

    //FG-Squircular Secondary

    //FG-Squircular Tertiary
    //Power2
    for i in tertiary_steps {
        tertiary_runner_wrapper(
            &format!("circle_power2_{:.2}_mapping", i),
            source_image,
            output_size_option,
            transformations::square_to_circle::power2,
            i,
        );
    }

    //FG-Squircular Quaternary

    //Elliptical Grid
    basic_runner_wrapper(
        "circle_elliptical_grid_mapping",
        source_image,
        output_size_option,
        transformations::square_to_circle::elliptical_grid,
    );

    //Elliptical Grid Secondary

    //Elliptical Grid Tertiary

    //Simple Stretch

    //Simple Stretch Secondary

    //Lamé-based
    basic_runner_wrapper(
        "circle_lame-based",
        source_image,
        output_size_option,
        transformations::square_to_circle::lame,
    );

    //p-Norm Squircular
}

pub fn run_all_circle_to_square<P, Container>(
    source_image: &image::ImageBuffer<P, Container>,
    output_size_option: Option<Size>,
) where
    P: image::Pixel + image::PixelWithColorType,
    [P::Subpixel]: image::EncodableLayout,
    Container: std::ops::Deref<Target = [P::Subpixel]>,
{
    //FG-Squircular

    //FG-Squircular Secondary

    //FG-Squircular Tertiary

    //FG-Squircular Quaternary

    //Elliptical Grid

    //Elliptical Grid Secondary

    //Elliptical Grid Tertiary

    //Simple Stretch

    //Simple Stretch Secondary

    //Lamé-based
    basic_runner_wrapper(
        "square_lame-based",
        source_image,
        output_size_option,
        transformations::circle_to_square::lame,
    );

    //p-Norm Squircular
}

pub fn run_all_half_face_superellipse<P, Container>(
    source_image: &image::ImageBuffer<P, Container>,
    output_size_option: Option<Size>,
) where
    P: image::Pixel + image::PixelWithColorType,
    [P::Subpixel]: image::EncodableLayout,
    Container: std::ops::Deref<Target = [P::Subpixel]>,
{
    //FG-Squircular

    //FG-Squircular Secondary

    //FG-Squircular Tertiary

    //FG-Squircular Quaternary

    //Elliptical Grid
    basic_runner_wrapper(
        "superellipse_elliptical_grid",
        source_image,
        output_size_option,
        transformations::half_face_superellipse::elliptical_grid,
    );

    //Elliptical Grid Secondary

    //Elliptical Grid Tertiary

    //Simple Stretch

    //Simple Stretch Secondary

    //Lamé-based
    basic_runner_wrapper(
        "superellipse_lame-based",
        source_image,
        output_size_option,
        transformations::half_face_superellipse::lame,
    );

    //p-Norm Squircular
}

fn step_maker(step_count: u32) -> Vec<f64>{
    let step_distance = 1.0 / step_count as f64;
    let mut steps: Vec<f64> = vec![];
    for i in 0..=step_count {
        steps.push(i as f64 * step_distance);
    }
    return steps;
}
