# Image Mapping

Transform images using arbitrary functions.

## Usage

Note: This is very much a developer tool. There are no options. You may edit the code to change functionality.

- In the directory you are running the tool, create a directory named `output` and place an image named `source.png`.

- Run `cargo run --release`. You may want to always build release for this project due how much computation happens. Debug builds will be significantly slower.

## Dependencies

Rust: https://www.rust-lang.org/tools/install

## Notes

- By default, it will try to run every transformation on `source.png`. Here is an example of how to set it to run a single transformation instead:

    ```rust
    fn main() {
        let source_image = image::open("source.png").unwrap().into_rgba8();
        let output_size = transformer::Size { width: 2000, height: 2000, };
        batch_runners::basic_runner_wrapper(
            "square_fg_squircular", // This controls the output filename. You can set this to anything you want.
            &source_image,
            Some(output_size), // Set to `None` to use the source image's size as the output size.
            transformations::circle_to_square::fg_squircular,
        );
    }
    ```

- With some mappings can end up with "holes" leftover from the stretching. Scaling the output size of the image down a bit can help alleviate this.
