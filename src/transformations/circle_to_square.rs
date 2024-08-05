use crate::{transformer, EPSILON};

//FG-Squircular
pub fn fg_squircular(point: transformer::Point) -> transformer::Point {
    if f64::abs(point.x) < EPSILON || f64::abs(point.y) < EPSILON {
        return point;
    }

    let u = point.x;
    let v = point.y;

    let u2 = f64::powi(u, 2);
    let v2 = f64::powi(v, 2);

    let sgnuv = f64::signum(u * v);

    let usqrttwo = u * f64::sqrt(2.0);
    let vsqrttwo = v * f64::sqrt(2.0);

    return transformer::Point {
        x: sgnuv / vsqrttwo * f64::sqrt(u2 + v2 - f64::sqrt((u2 + v2) * (u2 + v2 - 4.0 * u2 * v2))),
        y: sgnuv / usqrttwo * f64::sqrt(u2 + v2 - f64::sqrt((u2 + v2) * (u2 + v2 - 4.0 * u2 * v2))),
    };
}

//FG-Squircular Secondary
pub fn tapered_1_5(point: transformer::Point) -> transformer::Point {
    if f64::abs(point.x) < EPSILON || f64::abs(point.y) < EPSILON {
        return point;
    }

    let u = point.x;
    let v = point.y;

    let u2 = f64::powi(u, 2);
    let v2 = f64::powi(v, 2);

    let sgnu = f64::signum(u);
    let sgnv = f64::signum(v);

    return transformer::Point {
        x: (sgnu
            * f64::sqrt(
                ((1.0 + (u2 / v2)) / (2.0 * (3.0 - 2.0 * f64::sqrt(u2 + v2))))
                    - f64::sqrt(
                        f64::powi(
                            (1.0 + (u2 / v2)) / (2.0 * (3.0 - 2.0 * f64::sqrt(u2 + v2))),
                            2,
                        ) - (u2 / v2)
                            * (f64::powi(f64::sqrt(u2 + v2), 2) / (3.0 - 2.0 * f64::sqrt(u2 + v2))),
                    ),
            )),
        y: (sgnv
            * f64::sqrt(
                ((1.0 + (1.0 / (u2 / v2))) / (2.0 * (3.0 - 2.0 * f64::sqrt(u2 + v2))))
                    - f64::sqrt(
                        f64::powi(
                            (1.0 + (1.0 / (u2 / v2))) / (2.0 * (3.0 - 2.0 * f64::sqrt(u2 + v2))),
                            2,
                        ) - (1.0 / (u2 / v2))
                            * (f64::powi(f64::sqrt(u2 + v2), 2) / (3.0 - 2.0 * f64::sqrt(u2 + v2))),
                    ),
            )),
    };
}

//FG-Squircular Tertiary

//FG-Squircular Quaternary

//Elliptical Grid

//Elliptical Grid Secondary

//Elliptical Grid Tertiary

//Simple Stretch

//Simple Stretch Secondary

//Lamé-based
pub fn lame(point: transformer::Point) -> transformer::Point {
    let u = point.x;
    let v = point.y;

    let u2 = f64::powi(u, 2);
    let v2 = f64::powi(v, 2);

    let absu = f64::abs(u);
    let absv = f64::abs(v);

    let sgnu = f64::signum(u);
    let sgnv = f64::signum(v);

    return transformer::Point {
        x: (sgnu * f64::powf(absu, 1.0 - u2 - v2)),
        y: (sgnv * f64::powf(absv, 1.0 - u2 - v2)),
    };
}

//p-Norm Squircular
