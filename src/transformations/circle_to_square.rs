use crate::{math, transformer, EPSILON};

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

//Schwarz Christoffel
pub fn schwarz_christoffel(point: transformer::Point) -> transformer::Point {
    let k = 1.854074677301371918433850347195260046217598823521766905586;
    let u = point.x;
    let v = point.y;

    let ru: f64 = (u - v) * (1.0 / 2.0_f64).sqrt();
    let rv: f64 = (u + v) * (1.0 / 2.0_f64).sqrt();
    let a_big: f64 = ru * ru + rv * rv;
    let b_big: f64 = ru * ru - rv * rv;
    let u_big: f64 = 1.0 + 2.0 * b_big - a_big * a_big;
    let t_big: f64 = ((1.0 + a_big * a_big) * (1.0 + a_big * a_big) - 4.0 * b_big * b_big).sqrt();
    let cos_a: f64 = (2.0 * a_big - t_big) / u_big;
    let cos_b: f64 = u_big / (2.0 * a_big + t_big);
    let a: f64 = f64::acos(f64::min(f64::max(cos_a, -1.0), 1.0));
    let b: f64 = f64::acos(f64::min(f64::max(cos_b, -1.0), 1.0));
    let rx: f64 =
        f64::signum(ru) * (1.0 - math::schwarz_christoffel::landen_elliptic_f(a) / (2.0 * k));
    let ry: f64 = f64::signum(rv) * (math::schwarz_christoffel::landen_elliptic_f(b) / (2.0 * k));
    return transformer::Point {
        x: rx + ry,
        y: ry - rx,
    };
}
