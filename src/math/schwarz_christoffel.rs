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
    let mut g: f64 = (1.0 / 2.0_f64).sqrt();
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
        dbg!("WARNING: max iterations in landen_elliptic_f()");
    }

    let longlong1: i128 = 1;
    phi /= (longlong1 << i) as f64;
    return phi / g;
}

fn agm_jacobi_sn_cn_dn(u: f64) -> (f64, f64, f64) {
    const MAX_ITER: usize = 64;
    let mut a: [f64; MAX_ITER + 1] = [0.0; MAX_ITER + 1];
    let mut g: [f64; MAX_ITER + 1] = [0.0; MAX_ITER + 1];
    let mut c: [f64; MAX_ITER + 1] = [0.0; MAX_ITER + 1];
    a[0] = 1.0;
    g[0] = (1.0 / 2.0_f64).sqrt();
    c[0] = (1.0 / 2.0_f64).sqrt();
    let mut i: usize = 0;
    loop {
        a[i + 1] = 0.5 * (a[i] + g[i]);
        g[i + 1] = (a[i] * g[i]).sqrt();
        c[i + 1] = 0.5 * (a[i] - g[i]);
        i += 1;
        if !(i < MAX_ITER && f64::abs(a[i] - g[i]) > std::f64::EPSILON) {
            break;
        }
    }

    if i == MAX_ITER {
        dbg!("WARNING: max iterations in agm_jacobi_sn_cn_dn()");
    }

    let longlong1: i128 = 1;
    let mut phi: f64 = (longlong1 << i) as f64 * a[i] * u;
    while i > 0 {
        phi = 0.5 * (phi + f64::asin(c[i] * f64::sin(phi) / a[i]));
        i -= 1;
    }

    let sn = f64::sin(phi);
    let cn = f64::cos(phi);
    let dn = (1.0 - 0.5 * (sn * sn)).sqrt();
    return (sn, cn, dn);
}

pub fn ccn(re: f64, im: f64) -> (f64, f64) {
    let (sn_re, cn_re, dn_re) = agm_jacobi_sn_cn_dn(re);
    let (sn_im, cn_im, dn_im) = agm_jacobi_sn_cn_dn(im);
    let t = 1.0 - dn_re * dn_re * sn_im * sn_im;
    let ret_re = cn_re * cn_im / t;
    let ret_im = -sn_re * dn_re * sn_im * dn_im / t;
    return (ret_re, ret_im);
}
