use crate::{transformer, EPSILON};

//FG-Squircular
pub fn fg_squircular(point: transformer::Point) -> transformer::Point {
    if f64::abs(point.y) < EPSILON || f64::abs(point.x) < EPSILON {
        return point;
    }

    let x = point.x;
    let y = point.y;

    let x2 = f64::powi(x, 2);
    let y2 = f64::powi(y, 2);

    return transformer::Point {
        x: ((x * f64::sqrt(x2 + y2 - x2 * y2)) / (f64::sqrt(x2 + y2))),
        y: ((y * f64::sqrt(x2 + y2 - x2 * y2)) / (f64::sqrt(x2 + y2))),
    };
}

//FG-Squircular Secondary
pub fn tapered_1_5(point: transformer::Point) -> transformer::Point {
    if f64::abs(point.y) < EPSILON || f64::abs(point.x) < EPSILON {
        return point;
    }

    let x = point.x;
    let y = point.y;

    let x2 = f64::powi(point.x, 2);
    let y2 = f64::powi(point.y, 2);

    return transformer::Point {
        y: ((x / f64::sqrt(x2 + y2))
            * ((x2 * y2)
                + f64::sqrt(
                    f64::powi(x2 * y2, 2) - 3.0 * (x2 * y2) + f64::powi(f64::sqrt(x2 + y2), 2),
                ))),
        x: ((y / f64::sqrt(x2 + y2))
            * ((x2 * y2)
                + f64::sqrt(
                    f64::powi(x2 * y2, 2) - 3.0 * (x2 * y2) + f64::powi(f64::sqrt(x2 + y2), 2),
                ))),
    };
}

//FG-Squircular Tertiary
pub fn power2(point: transformer::Point, b: f64) -> transformer::Point {
    if f64::abs(point.y) < EPSILON || f64::abs(point.x) < EPSILON {
        return point;
    }

    let x2 = f64::powi(point.x, 2);
    let y2 = f64::powi(point.y, 2);

    return transformer::Point {
        y: ((f64::sqrt(
            (x2 + y2 - 2.0 * b * x2 * y2) / ((x2 + y2) * (1.0 + x2 * y2 - 2.0 * b * x2 * y2)),
        )) * point.x),
        x: ((f64::sqrt(
            (x2 + y2 - 2.0 * b * x2 * y2) / ((x2 + y2) * (1.0 + x2 * y2 - 2.0 * b * x2 * y2)),
        )) * point.y),
    };
}

//FG-Squircular Quaternary

//Elliptical Grid
pub fn elliptical_grid(point: transformer::Point) -> transformer::Point {
    return transformer::Point {
        x: point.x * f64::sqrt(1.0 - (f64::powi(point.y, 2) / 2.0)),
        y: point.y * f64::sqrt(1.0 - (f64::powi(point.x, 2) / 2.0)),
    };
}

//Elliptical Grid Secondary

//Elliptical Grid Tertiary

//Simple Stretch

//Simple Stretch Secondary

//Lamé-based
pub fn lame(point: transformer::Point) -> transformer::Point {
    let x = point.x;
    let y = point.y;

    let x2 = f64::powi(x, 2);
    let y2 = f64::powi(y, 2);

    let absx: f64 = f64::abs(x);
    let absy = f64::abs(y);

    return transformer::Point {
        x: ((x / f64::sqrt(x2 + y2))
            * f64::powf(
                f64::powf(absx, 2.0 / ((1.0 - absx) * (1.0 - absy)))
                    + f64::powf(absy, 2.0 / ((1.0 - absx) * (1.0 - absy))),
                0.5 * (1.0 - absx) * (1.0 - absy),
            )),
        y: ((y / f64::sqrt(x2 + y2))
            * f64::powf(
                f64::powf(absx, 2.0 / ((1.0 - absx) * (1.0 - absy)))
                    + f64::powf(absy, 2.0 / ((1.0 - absx) * (1.0 - absy))),
                0.5 * (1.0 - absx) * (1.0 - absy),
            )),
    };
}

//p-Norm Squircular
