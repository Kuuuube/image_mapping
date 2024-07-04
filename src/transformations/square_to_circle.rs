use crate::transformer;

//FG-Squircular

//FG-Squircular Secondary

//FG-Squircular Tertiary
pub fn power2(point: transformer::Point, b: f64) -> transformer::Point {
    if f64::abs(point.y) < 0.00001 || f64::abs(point.x) < 0.00001 {
        return transformer::Point {
            x: point.x,
            y: point.y,
        }
    }

    let x2 = f64::powi(point.x, 2);
    let y2 = f64::powi(point.y, 2);

    return transformer::Point {
        y: ((f64::sqrt((x2 + y2 - 2.0 * b * x2 * y2) / ((x2 + y2) * (1.0 + x2 * y2 - 2.0 * b * x2 * y2)))) * point.x),
        x: ((f64::sqrt((x2 + y2 - 2.0 * b * x2 * y2) / ((x2 + y2) * (1.0 + x2 * y2 - 2.0 * b * x2 * y2)))) * point.y)
    }
}

//FG-Squircular Quaternary

//Elliptical Grid
pub fn elliptical_grid(point: transformer::Point) -> transformer::Point {
    return transformer::Point {
        x: point.x * f64::sqrt(1.0 - (f64::powi(point.y, 2) / 2.0)),
        y: point.y * f64::sqrt(1.0 - (f64::powi(point.x, 2) / 2.0)),
    }
}

//Elliptical Grid Secondary

//Elliptical Grid Tertiary

//Simple Stretch

//Simple Stretch Secondary

//Lamé-based

//p-Norm Squircular
