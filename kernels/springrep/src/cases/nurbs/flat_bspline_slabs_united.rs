//! Two boxes united on the plane z = 4, where each has a flat B-spline face: (0, 0, 0)–(10, 10, 4)
//! with a bicubic top and (5, 3, 4)–(15, 13, 7) with a biquadratic bottom.
use springrep::boolean::union;
use springrep::geom::NurbsSurface;
use springrep::math::Vec3;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::nurbs::prism;

/// The Bézier patch in the plane z = 4 on the grid `xs` × `ys`, its control points unevenly
/// spaced.
fn flat(xs: &[f64], ys: &[f64]) -> Result<NurbsSurface, String> {
    let (nu, nv) = (xs.len(), ys.len());
    let ctrl = xs
        .iter()
        .flat_map(|&x| ys.iter().map(move |&y| Vec3::new(x, y, 4.)))
        .collect();
    let knots = |n: usize| [vec![0.; n], vec![1.; n]].concat();
    NurbsSurface::new(
        (nu - 1, nv - 1),
        (knots(nu), knots(nv)),
        (nu, nv),
        ctrl,
        None,
    )
    .map_err(e)
}

pub fn build() -> Result<Brep, String> {
    let lower = prism(flat(&[0., 2., 7., 10.], &[0., 4., 5., 10.])?, 0., "a")?;
    // the upper box reaches from its B-spline face at z = 4 up to the plane z = 7
    let upper = prism(flat(&[5., 12., 15.], &[3., 4., 13.])?, 7., "b")?;
    union(&lower, &upper).map_err(e)
}
