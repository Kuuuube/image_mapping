use crate::transformer::{self, Point};

//FG-Squircular

//FG-Squircular Secondary

//FG-Squircular Tertiary

//FG-Squircular Quaternary

//Elliptical Grid
pub fn elliptical_grid(point: transformer::Point) -> transformer::Point {
    let transformed_point = super::square_to_circle::elliptical_grid(point);
    let expanded_point = Point {
        x: transformed_point.x * 1.41421356237,
        y: transformed_point.y * 1.41421356237,
    };
    if expanded_point.x > 1.0
        || expanded_point.x < -1.0
        || expanded_point.y > 1.0
        || expanded_point.y < -1.0
    {
        return transformer::Point { x: 200.0, y: 200.0 };
    }
    return transformer::Point {
        x: point.x,
        y: point.y,
    };
}

//Elliptical Grid Secondary

//Elliptical Grid Tertiary

//Simple Stretch

//Simple Stretch Secondary

//Lamé-based

//p-Norm Squircular
