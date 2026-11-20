//! A cylinder of radius 5 from z = 0 to 10 whose side is one rational B-spline surface, the rim
//! of its top rounded to radius 1 by a rolling ball.
use springrep::blend::fillet;
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
        ctrl.extend([
            Vec3::new(5. * x, 5. * y, 0.),
            Vec3::new(5. * x, 5. * y, 10.),
        ]);
        weights.extend([w, w]);
    }
    let knots = (CIRCLE_KNOTS.to_vec(), vec![0., 0., 1., 1.]);
    let side = NurbsSurface::new((2, 1), knots, (9, 2), ctrl, Some(weights)).map_err(e)?;
    // the flat ends, each bounded by the side's rim at that end
    let disk = |v: f64, name: &str| -> Result<Piece, String> {
        let rim = Curve::Nurbs(Box::new(side.iso_curve(false, v)));
        Piece::planar(vec![vec![Span::new(rim, 0., 1.)]], name).map_err(e)
    };
    let side_face = Piece::whole(side.clone(), "side").map_err(e)?;
    let cylinder = sew(
        &[disk(0., "start")?, disk(1., "end")?, side_face],
        0.,
        "cyl",
    )
    .map_err(e)?;
    fillet(&cylinder, &[("cyl.end", "cyl.side")], 1., "round").map_err(e)
}
