//! A cone of radius 5 and height 10 whose side is one rational B-spline face, quadratic around and
//! linear up, collapsing to a pole at the apex, closed by a flat disk.
use springrep::geom::{Curve, NurbsSurface};
use springrep::math::Vec3;
use springrep::sew::{Piece, Span, sew};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::nurbs::{CIRCLE, CIRCLE_KNOTS, CIRCLE_W};

pub fn build() -> Result<Brep, String> {
    let mut ctrl = Vec::new();
    let mut weights = Vec::new();
    for (&(x, y), &w) in CIRCLE.iter().zip(&CIRCLE_W) {
        ctrl.extend([Vec3::new(5. * x, 5. * y, 0.), Vec3::new(0., 0., 10.)]);
        weights.extend([w, w]);
    }
    let knots = (CIRCLE_KNOTS.to_vec(), vec![0., 0., 1., 1.]);
    let side = NurbsSurface::new((2, 1), knots, (9, 2), ctrl, Some(weights)).map_err(e)?;
    let rim = Curve::Nurbs(Box::new(side.iso_curve(false, 0.).map_err(e)?));
    let base = Piece::planar(vec![vec![Span::new(rim, 0., 1.)]], "base").map_err(e)?;
    let side = Piece::whole(side, "side").map_err(e)?;
    sew(&[side, base], 0., "cone").map_err(e)
}
