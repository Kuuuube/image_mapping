// use num::{self, complex::ComplexFloat};

// const MIN_ERROR: f64 = 0.0000001;

// fn rf(input_x: num::Complex<f64>, input_y: num::Complex<f64>, input_z: num::Complex<f64>) -> num::Complex<f64> {
//     let mut x = input_x;
//     let mut y = input_y;
//     let mut z = input_z;

//     let mut a: num::Complex<f64>;
//     let mut dx: num::Complex<f64>;
//     let mut dy: num::Complex<f64>;
//     let mut dz: num::Complex<f64>;

//     loop {
//         let lambda = num::Complex::sqrt(x * y)
//             + num::Complex::sqrt(y * z)
//             + num::Complex::sqrt(z * x);

//         x = (x + lambda) / 4.0;
//         y = (y + lambda) / 4.0;
//         z = (z + lambda) / 4.0;

//         a = (x + y + z) / 3.0;

//         dx = 1.0 - x / a;
//         dy = 1.0 - y / a;
//         dz = 1.0 - z / a;
//         if !(((num::Complex::abs(dx).max(num::Complex::abs(dy))).max(num::Complex::abs(dz))) > MIN_ERROR) {
//             break;
//         }
//     }

//     let e2;
//     let e3;
//     e2 = dx * dy + dy * dz + dz * dx;
//     e3 = dy * dx * dz;

//     //http://dlmf.nist.gov/19.36#E1
//     let e2_2 = num::Complex::powi(&e2, 2);
//     let mut result = 1.0
//         - (1.0 / 10.0 * e2)
//         + (1.0 / 14.0 * e3)
//         + (1.0 / 24.0 * e2_2)
//         - (3.0 / 44.0 * e2 * e3)
//         - (5.0 / 208.0 * num::Complex::powi(&e2, 3))
//         + (3.0 / 104.0 * num::Complex::powi(&e3, 2))
//         + (1.0 / 16.0 * (e2_2 * e3));

//     result *= 1.0 / num::Complex::sqrt(a);
//     return result;
// }

// pub fn f(angle: num::Complex<f64>, k: f64) -> num::Complex<f64> {
//     return num::Complex::sin(angle) * rf(num::Complex::powi(&num::Complex::cos(angle), 2), 1.0 - k * num::Complex::powi(&num::Complex::sin(angle), 2), num::Complex { re: 1.0, im: 0.0 });
// }

pub fn landen_elliptic_f(input_phi: f64) -> f64 {
    let mut phi = input_phi;
    let mut a: f64 = 1.0;
    let mut g: f64 = (1.0/2.0_f64).sqrt();
    let mut last_a: f64;
    let mut last_g: f64;
    let mut tan_2n_phi: f64;

    const MAX_ITER: i128 = 63;
    let mut i: i128 = 0;
    loop {
        tan_2n_phi = f64::tan(phi);
        i += 1;
        phi += phi - f64::atan((a - g) * tan_2n_phi / (a + g * tan_2n_phi * tan_2n_phi));
        last_a = a;
        last_g = g;
        a = 0.5 * (last_a + last_g);
        g = (last_a * last_g).sqrt();
        if !(i < MAX_ITER && f64::abs(a - g) > std::f64::EPSILON) {
            break;
        }
    }

    if i == MAX_ITER {
        dbg!("WARNING: max iterations in landen_elliptic_F()");
    }

    let longlong1: i128 = 1;
    phi /= (longlong1 << i) as f64;
    return phi / g;
}
