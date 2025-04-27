use crate::{
    transformations,
    transformer::{self, Point, Size},
};

pub fn basic_runner_wrapper<P, Container, F>(
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

pub fn tertiary_runner_wrapper<P, Container, F>(
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

pub fn quaternary_runner_wrapper<P, Container, F>(
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
    let tertiary_steps = step_maker(5);

    //FG-Squircular
    basic_runner_wrapper(
        "circle_fg_squircular",
        source_image,
        output_size_option,
        transformations::square_to_circle::fg_squircular,
    );

    //FG-Squircular Secondary
    //Tapered 1.5
    basic_runner_wrapper(
        "circle_tapered_1_5",
        source_image,
        output_size_option,
        transformations::square_to_circle::tapered_1_5,
    );

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

    //Schwarz Christoffel
    basic_runner_wrapper(
        "circle_schwarz-christoffel",
        source_image,
        output_size_option,
        transformations::square_to_circle::schwarz_christoffel,
    );
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
    basic_runner_wrapper(
        "square_fg_squircular",
        source_image,
        output_size_option,
        transformations::circle_to_square::fg_squircular,
    );

    //FG-Squircular Secondary
    //Tapered 1.5
    basic_runner_wrapper(
        "square_tapered_1_5",
        source_image,
        output_size_option,
        transformations::circle_to_square::tapered_1_5,
    );

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
    let p_norm_steps = vec![0.01, 0.1, 0.5, 1.0, 5.0, 10.0]; // p-norm accepts all values >0, these steps give a decent preview
    for i in p_norm_steps {
        tertiary_runner_wrapper(
            &format!("circle_p-norm_squircular_{:.2}_mapping", i),
            &source_image,
            None,
            transformations::circle_to_square::p_norm_squircular,
            i,
        );
    }

    //Schwarz Christoffel
    basic_runner_wrapper(
        "square_schwarz-christoffel",
        source_image,
        output_size_option,
        transformations::circle_to_square::schwarz_christoffel,
    );
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
    basic_runner_wrapper(
        "superellipse_fg_squircular",
        source_image,
        output_size_option,
        transformations::half_face_superellipse::fg_squircular,
    );

    //FG-Squircular Secondary
    //Tapered 1.5
    basic_runner_wrapper(
        "tapered_1_5",
        source_image,
        output_size_option,
        transformations::half_face_superellipse::tapered_1_5,
    );

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

    //Schwarz Christoffel
    basic_runner_wrapper(
        "superellipse_schwarz-christoffel",
        source_image,
        output_size_option,
        transformations::half_face_superellipse::schwarz_christoffel,
    );
}

pub fn step_maker(step_count: u32) -> Vec<f64> {
    let step_distance = 1.0 / step_count as f64;
    let mut steps: Vec<f64> = vec![];
    for i in 0..=step_count {
        steps.push(i as f64 * step_distance);
    }
    return steps;
}
