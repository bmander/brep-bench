//! Vectors, Gauss's quadrature rules and the B-spline basis.
use crate::py::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

pub type V3 = [f64; 3];
pub const TAU: f64 = 2.0 * std::f64::consts::PI;
pub const PI: f64 = std::f64::consts::PI;

#[inline(always)]
pub fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline(always)]
pub fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline(always)]
pub fn mul(a: V3, s: f64) -> V3 {
    [a[0] * s, a[1] * s, a[2] * s]
}

#[inline(always)]
pub fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline(always)]
pub fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline(always)]
pub fn norm(a: V3) -> f64 {
    dot(a, a).sqrt()
}

pub fn unit(a: V3) -> R<V3> {
    let n = norm(a);
    if n == 0.0 {
        return unsupported("a direction of length zero");
    }
    Ok(mul(a, 1.0 / n))
}

/// Σ s·v over two (s, v) pairs, summed from zero in order.
#[inline(always)]
pub fn lin2(s1: f64, v1: V3, s2: f64, v2: V3) -> V3 {
    [
        (0.0 + s1 * v1[0]) + s2 * v2[0],
        (0.0 + s1 * v1[1]) + s2 * v2[1],
        (0.0 + s1 * v1[2]) + s2 * v2[2],
    ]
}

/// Σ s·v over three (s, v) pairs.
#[inline(always)]
pub fn lin3(s1: f64, v1: V3, s2: f64, v2: V3, s3: f64, v3: V3) -> V3 {
    [
        ((0.0 + s1 * v1[0]) + s2 * v2[0]) + s3 * v3[0],
        ((0.0 + s1 * v1[1]) + s2 * v2[1]) + s3 * v3[1],
        ((0.0 + s1 * v1[2]) + s2 * v2[2]) + s3 * v3[2],
    ]
}

// ---------------------------------------------------------------------------------------------
// quadrature

/// Gauss-Legendre nodes and weights on [-1, 1].
fn legendre(n: usize) -> (Vec<f64>, Vec<f64>) {
    let (mut xs, mut ws) = (Vec::new(), Vec::new());
    let nf = n as f64;
    for i in 1..=n {
        let mut x = (PI * (i as f64 - 0.25) / (nf + 0.5)).cos();
        let mut dp = 0.0;
        for _ in 0..100 {
            let (mut p0, mut p1) = (1.0, x);
            for k in 2..=n {
                let kf = k as f64;
                let p2 = ((2.0 * kf - 1.0) * x * p1 - (kf - 1.0) * p0) / kf;
                p0 = p1;
                p1 = p2;
            }
            dp = nf * (x * p1 - p0) / (x * x - 1.0);
            let dx = p1 / dp;
            x -= dx;
            if dx.abs() < 1e-16 {
                break;
            }
        }
        xs.push(x);
        ws.push(2.0 / ((1.0 - x * x) * dp * dp));
    }
    (xs, ws)
}

/// Gauss-Legendre's n nodes and weights on [-1, 1], kept.
pub fn gauss(n: usize) -> Arc<(Vec<f64>, Vec<f64>)> {
    static KEPT: OnceLock<Mutex<HashMap<usize, Arc<(Vec<f64>, Vec<f64>)>>>> = OnceLock::new();
    let mut kept = KEPT
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    kept.entry(n)
        .or_insert_with(|| Arc::new(legendre(n)))
        .clone()
}

/// How many Gauss points integrate the measures exactly along a polynomial parameter of
/// degree p: x³ (degree 3p) times the normal (2p - 1), 5p - 1 in all.
pub fn exact(p: usize) -> usize {
    ((5 * p + 1) / 2).max(2)
}

/// As `exact`, along a parameter in which the surface curves: twelve points a piece at least.
pub fn curved(p: usize) -> usize {
    exact(p).max(12)
}

// Gauss-Kronrod 7-15: the Kronrod nodes (the Gauss ones at odd places) and both weights
const XK: [f64; 8] = [
    0.991455371120812639206854697526329,
    0.949107912342758524526189684047851,
    0.864864423359769072789712788640926,
    0.741531185599394439863864773280788,
    0.586087235467691130294144845693013,
    0.405845151377397166906606412076961,
    0.207784955007898467600689403773245,
    0.0,
];
const WK: [f64; 8] = [
    0.022935322010529224963732008058970,
    0.063092092629978553290700663189204,
    0.104790010322250183839876322541518,
    0.140653259715525918745189590510238,
    0.169004726639267902826583426598550,
    0.190350578064785409913256402421014,
    0.204432940075298892414161999234649,
    0.209482141084727828012999174891714,
];
const WG: [f64; 4] = [
    0.129484966168869693270611432679082,
    0.279705391489276667901467771423780,
    0.381830050505118944950369775488975,
    0.417959183673469387755102040816327,
];

/// (node, Kronrod weight, Gauss weight) on [-1, 1], in the order the sums are taken (which the answer's last bits rest on).
pub fn kronrod() -> &'static [(f64, f64, f64); 15] {
    static K: OnceLock<[(f64, f64, f64); 15]> = OnceLock::new();
    K.get_or_init(|| {
        let mut out = [(0.0, 0.0, 0.0); 15];
        let mut n = 0;
        for (i, &x) in XK.iter().enumerate() {
            let g = if i % 2 == 1 { WG[i / 2] } else { 0.0 };
            out[n] = (-x, WK[i], g);
            n += 1;
            if x != 0.0 {
                out[n] = (x, WK[i], g);
                n += 1;
            }
        }
        out
    })
}

/// How near each panel of the integral round a loop is taken, as a fraction of the solid's size's
/// power (its size cubed, for the volume).
pub const TOLERANCE: f64 = 1e-11;
/// The most points the integral along one piece of a loop may take, before it is given up.
pub const BUDGET: usize = 3000000;
/// How near together a surface's two rates S_u and S_v may run, as the square of the sine of the
/// angle between them, before the integral round a loop is taken by chords there (face.rs,
/// `chords`): a hundredth of a radian. A pole, where one rate is nothing, is not this: the sine
/// is of the angle, whatever the rates' lengths.
pub const FOLDED: f64 = 1e-4;
/// How far a path's rate, integrated over a panel, may come from how far the path went over it,
/// as a fraction of that, and be a writer's tolerance (face.rs, `adaptive`). A curve standing a
/// tolerance off a surface that bends is that fraction longer or shorter than its image on the
/// surface, and neither is the edge more truly than the other; past this the two have parted for
/// another reason, and the loop does not close.
pub const PARTED: f64 = 1e-6;
/// The measures integrated, in order: the face's area in (u, v), its area, the solid's volume, its
/// first moments ∫x, ∫y, ∫z, its second moments ∫x², ∫y², ∫z², ∫xy, ∫yz, ∫zx.
pub const NM: usize = 12;

// ---------------------------------------------------------------------------------------------
// B-splines

/// The knot span of `t`: i with U[i] <= t < U[i+1], p <= i <= n (the NURBS Book, A2.1).
#[inline]
pub fn span(n: usize, p: usize, t: f64, u: &[f64]) -> usize {
    // at the domain's ends, the last (or first) span of non-zero length: knots need not be
    // clamped, and an end knot may be repeated past the domain
    if t >= u[n + 1] {
        let mut i = n;
        while i > p && u[i] >= u[i + 1] {
            i -= 1;
        }
        return i;
    }
    if t <= u[p] {
        let mut i = p;
        while i < n && u[i] >= u[i + 1] {
            i += 1;
        }
        return i;
    }
    let (mut lo, mut hi) = (p, n + 1);
    let mut mid = (lo + hi) / 2;
    while t < u[mid] || t >= u[mid + 1] {
        if t < u[mid] {
            hi = mid;
        } else {
            lo = mid;
        }
        mid = (lo + hi) / 2;
    }
    mid
}

/// The most a B-spline's degree may be for its basis to be kept on the stack.
pub const DEG: usize = 15;

/// The non-zero basis functions at `t` and their first derivatives, into `nn` and `dd` (each
/// p + 1 long): Cox-de Boor's triangle (the NURBS Book, A2.2), the derivatives from the degree
/// below's values (its eq. 2.7).
#[inline]
pub fn basis(i: usize, t: f64, p: usize, u: &[f64], nn: &mut [f64], dd: &mut [f64]) {
    if p == 1 {
        let h = u[i + 1] - u[i];
        if h != 0.0 {
            let x = (t - u[i]) / h;
            nn[0] = 1.0 - x;
            nn[1] = x;
            dd[0] = -1.0 / h;
            dd[1] = 1.0 / h;
        } else {
            nn[0] = 1.0;
            nn[1] = 0.0;
            dd[0] = 0.0;
            dd[1] = 0.0;
        }
        return;
    }
    if p == 0 {
        nn[0] = 1.0;
        dd[0] = 0.0;
        return;
    }
    let mut stack = [0.0f64; 3 * (DEG + 1)];
    let mut heap;
    let work: &mut [f64] = if p <= DEG {
        &mut stack[..3 * (p + 1)]
    } else {
        heap = vec![0.0f64; 3 * (p + 1)];
        &mut heap
    };
    let (left, rest) = work.split_at_mut(p + 1);
    let (right, below) = rest.split_at_mut(p + 1);
    nn[0] = 1.0;
    for x in nn[1..=p].iter_mut() {
        *x = 0.0;
    }
    for j in 1..=p {
        left[j] = t - u[i + 1 - j];
        right[j] = u[i + j] - t;
        if j == p {
            below[..p].copy_from_slice(&nn[..p]);
        }
        let mut saved = 0.0;
        for r in 0..j {
            let den = right[r + 1] + left[j - r];
            let tmp = if den != 0.0 { nn[r] / den } else { 0.0 };
            nn[r] = saved + right[r + 1] * tmp;
            saved = left[j - r] * tmp;
        }
        nn[j] = saved;
    }
    let pf = p as f64;
    for r in 0..=p {
        let a = i - p + r; // the function's index
        let mut d = 0.0;
        if r >= 1 {
            let den = u[a + p] - u[a];
            d += if den != 0.0 { below[r - 1] / den } else { 0.0 };
        }
        if r <= p - 1 {
            let den = u[a + p + 1] - u[a + 1];
            d -= if den != 0.0 { below[r] / den } else { 0.0 };
        }
        dd[r] = pf * d;
    }
}

/// The knots of a B-spline whose kind implies them: Bézier, uniform, quasi-uniform.
pub fn implicit_knots(name: &str, n: i64, p: i64) -> R<Vec<f64>> {
    let rep = |x: f64, k: i64| std::iter::repeat(x).take(k.max(0) as usize);
    if name.starts_with("BEZIER") {
        return Ok(rep(0.0, p + 1).chain(rep(1.0, p + 1)).collect());
    }
    if name.starts_with("UNIFORM") {
        return Ok((0..(n + p + 2).max(0)).map(|i| (i - p) as f64).collect());
    }
    if name.starts_with("QUASI_UNIFORM") {
        let inner = n - p;
        return Ok(rep(0.0, p + 1)
            .chain((1..=inner).map(|i| i as f64))
            .chain(rep((inner + 1) as f64, p + 1))
            .collect());
    }
    unsupported(format!("a B-spline without knots: {}", name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gauss_integrates_polynomials() {
        let g = gauss(4);
        let s: f64 = g.0.iter().zip(&g.1).map(|(x, w)| w * x.powi(6)).sum();
        assert!((s - 2.0 / 7.0).abs() < 1e-14);
        assert_eq!(kronrod().len(), 15);
        assert_eq!(kronrod()[14].0, 0.0);
    }

    #[test]
    fn the_basis_sums_to_one() {
        let u = [0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0];
        let i = span(4, 3, 1.5, &u);
        assert_eq!(i, 4);
        let (mut n, mut d) = ([0.0; 4], [0.0; 4]);
        basis(i, 1.5, 3, &u, &mut n, &mut d);
        assert!((n.iter().sum::<f64>() - 1.0).abs() < 1e-15);
        assert!(d.iter().sum::<f64>().abs() < 1e-15);
    }
}
