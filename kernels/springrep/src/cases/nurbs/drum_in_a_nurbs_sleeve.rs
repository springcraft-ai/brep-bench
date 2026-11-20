//! A drum of radius 5 on the z axis from z = 0 to 10, united with a sleeve from radius 5 to 7 and
//! z = 2 to 8 whose bore is the drum's cylinder written as a rational B-spline surface, sewn from
//! that surface, the cylinder of radius 7 and its two flat ends.
use std::f64::consts::{FRAC_1_SQRT_2, TAU};

use springrep::boolean::union;
use springrep::geom::{Circle, Curve, Cylinder, NurbsSurface, Surface};
use springrep::math::Vec3;
use springrep::primitive::cylinder;
use springrep::sew::{Piece, Span, sew};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let drum = cylinder(&at([0., 0., 0.]), 5., 10., "drum").map_err(e)?;
    // the bore: degree 2 round the axis on nine weighted points, 1 along it from z = 2 to 8
    let ring = [
        (0., 5.),
        (-5., 5.),
        (-5., 0.),
        (-5., -5.),
        (0., -5.),
        (5., -5.),
        (5., 0.),
        (5., 5.),
        (0., 5.),
    ];
    let h = FRAC_1_SQRT_2;
    let ring_w = [1., h, 1., h, 1., h, 1., h, 1.];
    let mut ctrl = Vec::new();
    let mut weights = Vec::new();
    for (&(x, y), &w) in ring.iter().zip(&ring_w) {
        ctrl.extend([Vec3::new(x, y, 2.), Vec3::new(x, y, 8.)]);
        weights.extend([w, w]);
    }
    let knots = vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.];
    let bore = NurbsSurface::new(
        (2, 1),
        (knots, vec![0., 0., 1., 1.]),
        (9, 2),
        ctrl,
        Some(weights),
    )
    .map_err(e)?;
    let rim = |z: f64| {
        let circle = Circle {
            frame: at([0., 0., z]),
            radius: 7.,
        };
        Span::new(Curve::Circle(circle), 0., TAU)
    };
    // the bore's own boundary circles, its parameter lines v = 0 (z = 2) and v = 1 (z = 8)
    let bore_rim = |v: f64| {
        let line = bore.iso_curve(false, v);
        Ok::<_, String>(Span::new(Curve::Nurbs(Box::new(line)), 0., 1.))
    };
    // the outer side's seam, where its circles start
    let seam = |z: f64| Vec3::new(7., 0., z);
    let outside = Piece::bounded(
        Surface::Cylinder(Cylinder {
            frame: at([0., 0., 2.]),
            radius: 7.,
        }),
        vec![vec![
            rim(2.),
            Span::line(seam(2.), seam(8.)).map_err(e)?,
            Span::new(rim(8.).curve, TAU, 0.),
            Span::line(seam(8.), seam(2.)).map_err(e)?,
        ]],
        "outside",
    );
    let bottom = Piece::planar(vec![vec![rim(2.)], vec![bore_rim(0.)?]], "bottom").map_err(e)?;
    let top = Piece::planar(vec![vec![rim(8.)], vec![bore_rim(1.)?]], "top").map_err(e)?;
    let bore = Piece::whole(bore, "bore").map_err(e)?;
    let sleeve = sew(&[bore, outside, bottom, top], 0., "sleeve").map_err(e)?;
    union(&drum, &sleeve).map_err(e)
}
