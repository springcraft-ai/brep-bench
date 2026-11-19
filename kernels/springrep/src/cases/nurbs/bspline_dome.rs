//! A block on the square 0 ≤ x, y ≤ 20 under a clamped bicubic B-spline of 6 × 5 control points
//! with a double knot, its control points not at its knots' averages.
use springrep::geom::NurbsSurface;
use springrep::math::Vec3;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::nurbs::prism;

const XS: [f64; 6] = [0., 3., 7., 11., 16., 20.];
const YS: [f64; 5] = [0., 5., 10., 15., 20.];
const HEIGHTS: [[f64; 5]; 6] = [
    [3., 5., 6., 5., 4.],
    [5., 9., 11., 8., 5.],
    [6., 12., 15., 10., 6.],
    [4., 10., 18., 9., 5.],
    [5., 8., 9., 7., 4.],
    [2., 4., 5., 4., 3.],
];

pub fn build() -> Result<Brep, String> {
    let ctrl = (0..6)
        .flat_map(|i| (0..5).map(move |j| Vec3::new(XS[i], YS[j], HEIGHTS[i][j])))
        .collect();
    let knots = (
        vec![0., 0., 0., 0., 0.4, 0.4, 1., 1., 1., 1.],
        vec![0., 0., 0., 0., 1. / 3., 1., 1., 1., 1.],
    );
    let top = NurbsSurface::new((3, 3), knots, (6, 5), ctrl, None).map_err(e)?;
    prism(top, 0., "dome")
}
