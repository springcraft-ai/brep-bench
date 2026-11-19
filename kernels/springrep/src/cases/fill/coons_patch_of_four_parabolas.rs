//! The solid over the square from (0, 0) to (10, 10) under the bilinear Coons patch of four
//! quadratic Bézier arcs, with upright walls and a flat floor.
use springrep::geom::{Curve, NurbsCurve};
use springrep::math::Vec3;
use springrep::sew::{Span, coons};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::nurbs::prism;

/// The quadratic Bézier arc from `a` through the control point `m` to `b`.
fn arc(a: [f64; 3], m: [f64; 3], b: [f64; 3]) -> Result<Span, String> {
    let ctrl = [a, m, b].map(Vec3::from_array).to_vec();
    let c = NurbsCurve::new(2, vec![0., 0., 0., 1., 1., 1.], ctrl, None).map_err(e)?;
    Ok(Span::new(Curve::Nurbs(Box::new(c)), 0., 1.))
}

pub fn build() -> Result<Brep, String> {
    let rim = [
        arc([0., 0., 10.], [5., 0., 14.], [10., 0., 10.])?,
        arc([10., 0., 10.], [10., 5., 6.], [10., 10., 10.])?,
        arc([10., 10., 10.], [5., 10., 6.], [0., 10., 10.])?,
        arc([0., 10., 10.], [0., 5., 14.], [0., 0., 10.])?,
    ];
    prism(coons(&rim).map_err(e)?, 0., "block")
}
