mod batch_runners;
mod transformations;
mod transformer;

const EPSILON: f64 = 0.00001;

fn main() {
    let source_image = image::open("source.png").unwrap().into_rgba8();

    run_batches(
        &source_image,
        transformer::Size {
            width: 2000,
            height: 2000,
        },
    );
}

fn run_batches<P, Container>(
    source_image: &image::ImageBuffer<P, Container>,
    batch_size: transformer::Size,
) where
    P: image::Pixel + image::PixelWithColorType,
    [P::Subpixel]: image::EncodableLayout,
    Container: std::ops::Deref<Target = [P::Subpixel]>,
{
    let start_time = std::time::Instant::now();
    batch_runners::run_all_circle_to_square(source_image, Some(batch_size));
    println!(
        "Generated circle to square batch in: {}ms",
        start_time.elapsed().as_millis()
    );

    let start_time = std::time::Instant::now();
    batch_runners::run_all_square_to_circle(source_image, Some(batch_size));
    println!(
        "Generated square to circle batch in: {}ms",
        start_time.elapsed().as_millis()
    );

    let start_time = std::time::Instant::now();
    batch_runners::run_all_half_face_superellipse(source_image, Some(batch_size));
    println!(
        "Generated half face superellipse batch in: {}ms",
        start_time.elapsed().as_millis()
    );
}
