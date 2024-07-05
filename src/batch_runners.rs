use crate::{
    transformations,
    transformer::{self, Point, Size},
};

fn basic_runner_wrapper<F>(
    mapping_name: &str,
    source_image: image::DynamicImage,
    output_size_option: Option<Size>,
    mut transformation: F,
) where
    F: FnMut(Point) -> Point,
{
    transformer::transform_image(source_image, output_size_option, |point| {
        transformation(point)
    })
    .save(&format!("output/{}.png", mapping_name))
    .expect(&format!("Failed to generate {}", mapping_name));
}

fn tertiary_runner_wrapper<F>(
    mapping_name: &str,
    source_image: image::DynamicImage,
    output_size_option: Option<Size>,
    mut transformation: F,
    tertiary_value: f64,
) where
    F: FnMut(Point, f64) -> Point,
{
    transformer::transform_image(source_image, output_size_option, |point| {
        transformation(point, tertiary_value)
    })
    .save(&format!("output/{}.png", mapping_name))
    .expect(&format!("Failed to generate {}", mapping_name));
}

fn quaternary_runner_wrapper<F>(
    mapping_name: &str,
    source_image: image::DynamicImage,
    output_size_option: Option<Size>,
    mut transformation: F,
    tertiary_value: f64,
    quaternary_value: f64,
) where
    F: FnMut(Point, f64, f64) -> Point,
{
    transformer::transform_image(source_image, output_size_option, |point| {
        transformation(point, tertiary_value, quaternary_value)
    })
    .save(&format!("output/{}.png", mapping_name))
    .expect(&format!("Failed to generate {}", mapping_name));
}

pub fn run_all_square_to_circle(
    source_image: image::DynamicImage,
    output_size_option: Option<Size>,
) {
    //FG-Squircular

    //FG-Squircular Secondary

    //FG-Squircular Tertiary
    //Power2
    {
        tertiary_runner_wrapper(
            "circle_power2_B0.01_mapping",
            source_image.clone(),
            output_size_option,
            transformations::square_to_circle::power2,
            0.01,
        );
        tertiary_runner_wrapper(
            "circle_power2_B0.25_mapping",
            source_image.clone(),
            output_size_option,
            transformations::square_to_circle::power2,
            0.25,
        );
        tertiary_runner_wrapper(
            "circle_power2_B0.33_mapping",
            source_image.clone(),
            output_size_option,
            transformations::square_to_circle::power2,
            0.33,
        );
        tertiary_runner_wrapper(
            "circle_power2_B0.50_mapping",
            source_image.clone(),
            output_size_option,
            transformations::square_to_circle::power2,
            0.50,
        );
        tertiary_runner_wrapper(
            "circle_power2_B0.66_mapping",
            source_image.clone(),
            output_size_option,
            transformations::square_to_circle::power2,
            0.66,
        );
        tertiary_runner_wrapper(
            "circle_power2_B0.75_mapping",
            source_image.clone(),
            output_size_option,
            transformations::square_to_circle::power2,
            0.75,
        );
        tertiary_runner_wrapper(
            "circle_power2_B0.99_mapping",
            source_image.clone(),
            output_size_option,
            transformations::square_to_circle::power2,
            0.99,
        );
    }

    //FG-Squircular Quaternary

    //Elliptical Grid
    basic_runner_wrapper(
        "circle_elliptical_grid_mapping",
        source_image.clone(),
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
        source_image.clone(),
        output_size_option,
        transformations::square_to_circle::lame,
    );

    //p-Norm Squircular
}

pub fn run_all_circle_to_square(
    source_image: image::DynamicImage,
    output_size_option: Option<Size>,
) {
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
        source_image.clone(),
        output_size_option,
        transformations::circle_to_square::lame,
    );

    //p-Norm Squircular
}

pub fn run_all_half_face_superellipse(
    source_image: image::DynamicImage,
    output_size_option: Option<Size>,
) {
    //FG-Squircular

    //FG-Squircular Secondary

    //FG-Squircular Tertiary

    //FG-Squircular Quaternary

    //Elliptical Grid
    basic_runner_wrapper(
        "superellipse_elliptical_grid",
        source_image.clone(),
        output_size_option,
        transformations::half_face_superellipse::elliptical_grid,
    );

    //Elliptical Grid Secondary

    //Elliptical Grid Tertiary

    //Simple Stretch

    //Simple Stretch Secondary

    //Lamé-based

    //p-Norm Squircular
}
