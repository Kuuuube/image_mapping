use crate::transformer;

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
pub fn lame(point: transformer::Point) -> transformer::Point {
    let u = point.x;
    let v = point.y;

    let u2 = f64::powi(u, 2);
    let v2 = f64::powi(v, 2);

    let absu = f64::abs(u);
    let absv = f64::abs(v);

    let sgnu = absu / u;
    let sgnv = absv / v;

    return transformer::Point {
        x: (sgnu * f64::powf(absu, 1.0 - u2 - v2)),
        y: (sgnv * f64::powf(absv, 1.0 - u2 - v2)),
    };
}

//p-Norm Squircular
