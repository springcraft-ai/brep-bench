//! Helpers for the nurbs cases: the block under the bicubic Bézier dome, the block under the
//! weighted barrel vault, the blocks under other B-spline surfaces, and the ball bounded by the
//! rational quadratic sphere.
use std::f64::consts::FRAC_1_SQRT_2;

use springrep::geom::NurbsSurface;
use springrep::math::Vec3;
use springrep::primitive::slab;
use springrep::sew::{Piece, Span, sew};
use springrep::topo::Brep;

use super::{at, e};

/// The block on the square 0 ≤ x, y ≤ 20 under the bicubic Bézier dome: a slab, whose top is the
/// B-spline height field with its control points over its knots' Greville abscissae, which for a
/// Bézier are the even grid 0, ⅓, ⅔, 1 the dome's control points stand on.
pub fn bezier_dome() -> Result<Brep, String> {
    let bezier = [0., 0., 0., 0., 1., 1., 1., 1.];
    let heights = vec![
        vec![4., 6., 7., 5.],
        vec![6., 14., 12., 7.],
        vec![5., 13., 16., 8.],
        vec![3., 6., 8., 6.],
    ];
    slab(
        &at([0., 0., 0.]),
        [20., 20.],
        (&bezier, &bezier),
        &heights,
        "dome",
    )
    .map_err(e)
}

/// The solid between `top`, a B-spline surface each of whose boundary lines lies in an upright
/// plane, and the plane z = `z`: `top`'s whole face named `top`, an upright flat wall between each
/// boundary line and that plane, and the flat floor there, sewn into one solid.
pub fn prism(top: NurbsSurface, z: f64, feature: &str) -> Result<Brep, String> {
    let top = Piece::whole(top, "top").map_err(e)?;
    let down = |p: Vec3| Vec3::new(p.x, p.y, z);
    let mut pieces = Vec::new();
    let mut floor = Vec::new();
    for (k, s) in top.loops()[0].iter().enumerate() {
        let (a, b) = (s.curve.eval(s.t0), s.curve.eval(s.t1));
        let wall = vec![
            Span::line(down(a), down(b)).map_err(e)?,
            Span::line(down(b), b).map_err(e)?,
            Span::new(s.curve.clone(), s.t1, s.t0),
            Span::line(a, down(a)).map_err(e)?,
        ];
        pieces.push(Piece::planar(vec![wall], &format!("wall_{k}")).map_err(e)?);
        floor.push(Span::line(down(a), down(b)).map_err(e)?);
    }
    pieces.push(Piece::planar(vec![floor], "floor").map_err(e)?);
    pieces.push(top);
    sew(&pieces, 0., feature).map_err(e)
}

/// The block on −12 ≤ x, y ≤ 12 from z = 0 under the weighted barrel vault that arches across y:
/// degree 1 along x and 2 along y, its rows (x, −12, 10), (x, 0, 26), (x, 12, 10) weighted 1,
/// 3/5, 1, for x = −12 and 12.
pub fn vault() -> Result<Brep, String> {
    let arch = [(-12., 10., 1.), (0., 26., 0.6), (12., 10., 1.)];
    let mut ctrl = Vec::new();
    let mut weights = Vec::new();
    for x in [-12., 12.] {
        for (y, z, w) in arch {
            ctrl.push(Vec3::new(x, y, z));
            weights.push(w);
        }
    }
    let top = NurbsSurface::new(
        (1, 2),
        (vec![0., 0., 1., 1.], vec![0., 0., 0., 1., 1., 1.]),
        (2, 3),
        ctrl,
        Some(weights),
    )
    .map_err(e)?;
    prism(top, 0., "vault")
}

/// The control points of the rational quadratic unit circle about the origin in the xy plane, from
/// (1, 0) round counter-clockwise, with [`CIRCLE_W`] and [`CIRCLE_KNOTS`].
pub const CIRCLE: [(f64, f64); 9] = [
    (1., 0.),
    (1., 1.),
    (0., 1.),
    (-1., 1.),
    (-1., 0.),
    (-1., -1.),
    (0., -1.),
    (1., -1.),
    (1., 0.),
];
const H: f64 = FRAC_1_SQRT_2;
pub const CIRCLE_W: [f64; 9] = [1., H, 1., H, 1., H, 1., H, 1.];
pub const CIRCLE_KNOTS: [f64; 12] = [0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.];

/// The ball of radius 5 about (`cx`, 0, 0) bounded by one face, the biquadratic 9 × 5 rational
/// sphere: the circle's net round, the weighted semicircle's from the south pole to the north.
pub fn nurbs_ball(cx: f64, feature: &str) -> Result<Brep, String> {
    let meridian = [(0., -5.), (5., -5.), (5., 0.), (5., 5.), (0., 5.)];
    let meridian_w = [1., H, 1., H, 1.];
    let mut ctrl = Vec::new();
    let mut weights = Vec::new();
    for (&(x, y), &wi) in CIRCLE.iter().zip(&CIRCLE_W) {
        for (&(rho, z), &wj) in meridian.iter().zip(&meridian_w) {
            ctrl.push(Vec3::new(cx + rho * x, rho * y, z));
            weights.push(wi * wj);
        }
    }
    let skin = NurbsSurface::new(
        (2, 2),
        (
            CIRCLE_KNOTS.to_vec(),
            vec![0., 0., 0., 0.5, 0.5, 1., 1., 1.],
        ),
        (9, 5),
        ctrl,
        Some(weights),
    )
    .map_err(e)?;
    sew(&[Piece::whole(skin, "skin").map_err(e)?], 0., feature).map_err(e)
}
