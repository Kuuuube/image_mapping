#[cfg(test)]
#[test]
fn test() {
    let source_image = image::open("source.png").unwrap().into_rgba8();
    crate::run_batches(
        &source_image,
        crate::transformer::Size {
            width: 500,
            height: 500,
        },
    );
}
