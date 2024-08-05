use crate::transformer::{self, Point};

fn superellipse_wrapper<F>(point: transformer::Point, mut transformation: F) -> Point
where
    F: FnMut(Point) -> Point,
{
    let transformed_point = transformation(point);
    let expanded_point = Point {
        x: transformed_point.x * 1.41421356237,
        y: transformed_point.y * 1.41421356237,
    };
    if expanded_point.x > 1.0
        || expanded_point.x < -1.0
        || expanded_point.y > 1.0
        || expanded_point.y < -1.0
    {
        return transformer::Point {
            x: f64::INFINITY,
            y: f64::INFINITY,
        };
    }
    return transformer::Point {
        x: point.x,
        y: point.y,
    };
}

//FG-Squircular
pub fn fg_squircular(point: transformer::Point) -> transformer::Point {
    return superellipse_wrapper(point, super::square_to_circle::fg_squircular);
}

//FG-Squircular Secondary
pub fn tapered_1_5(point: transformer::Point) -> transformer::Point {
    return superellipse_wrapper(point, super::square_to_circle::tapered_1_5);
}

//FG-Squircular Tertiary

//FG-Squircular Quaternary

//Elliptical Grid
pub fn elliptical_grid(point: transformer::Point) -> transformer::Point {
    return superellipse_wrapper(point, super::square_to_circle::elliptical_grid);
}

//Elliptical Grid Secondary

//Elliptical Grid Tertiary

//Simple Stretch

//Simple Stretch Secondary

//Lamé-based
pub fn lame(point: transformer::Point) -> transformer::Point {
    return superellipse_wrapper(point, super::square_to_circle::lame);
}

//p-Norm Squircular
