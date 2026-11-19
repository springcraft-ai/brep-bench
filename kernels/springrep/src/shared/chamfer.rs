//! Helpers shared by the chamfer cases: solids turned about the z axis from a meridian drawn in
//! (ρ, z), half-spaces, and rims chamfered by a distance and an angle or by two distances.
use std::f64::consts::TAU;

use springrep::blend::{EdgeName, chamfer_distance_angle, chamfer_two_edges};
use springrep::boolean::union;
use springrep::geom::Curve;
use springrep::math::{Frame, Vec2, Vec3};
use springrep::pick::{edges_named, faces_named};
use springrep::primitive::block;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::revolve;
use springrep::topo::{Brep, EdgeId};

use crate::shared::e;

/// The plane y = 0 drawn on in (ρ, z): its x is the world's x and its y the world's z.
pub fn meridian_plane() -> Frame {
    Frame::new(Vec3::ZERO, Vec3::new(0., -1., 0.), Vec3::X).expect("the xz plane")
}

/// The solid `segments`, a closed meridian in (ρ, z), sweeps turned a whole turn about the z axis.
pub fn turned(segments: Vec<NamedSegment>, feature: &str) -> Result<Brep, String> {
    let plane = meridian_plane();
    revolve(
        &Profile::outline(segments),
        &plane,
        Vec3::ZERO,
        Vec3::Z,
        TAU,
        feature,
    )
    .map_err(e)
}

/// The block on the inner side of the plane through `point` with inward normal `inward`, `size`
/// across and deep, centered on `point` along the plane: a half-space as far as the cases reach.
/// Its face on the plane is `<feature>.start`.
pub fn half_space(point: Vec3, inward: Vec3, size: f64, feature: &str) -> Result<Brep, String> {
    let f = Frame::from_axis(point, inward).map_err(e)?;
    let corner = f.origin - f.x * (size / 2.) - f.y * (size / 2.);
    let f = Frame {
        origin: corner,
        ..f
    };
    block(&f, Vec3::new(size, size, size), feature).map_err(e)
}

/// The ball of radius 5 about the origin.
pub fn ball(feature: &str) -> Result<Brep, String> {
    springrep::primitive::sphere(&Frame::WORLD, 5., feature).map_err(e)
}

/// The seat (the cylinder of radius 8 from z = −6 to 5, hollowed above the cone z = 1 + ρ/2, its
/// face `seat.cone`) with the ball of radius 5 about the origin (`ball.skin`) standing in it: they
/// meet in the concave circle of radius 4 at z = 3.
pub fn ball_in_conical_seat() -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let seat = turned(
        vec![
            NamedSegment::line(p(0., -6.), p(8., -6.), "bottom"),
            NamedSegment::line(p(8., -6.), p(8., 5.), "side"),
            NamedSegment::line(p(8., 5.), p(0., 1.), "cone"),
            NamedSegment::line(p(0., 1.), p(0., -6.), "axis"),
        ],
        "seat",
    )?;
    union(&seat, &ball("ball")?).map_err(e)
}

/// The plate (the cylinder of radius 8 from z = −6 to 0, its top `plate.end`) with the ball of
/// radius 5 about the origin (`ball.skin`) standing in it: they meet in the concave circle of
/// radius 5 at z = 0.
pub fn ball_on_plate() -> Result<Brep, String> {
    let at = Frame::WORLD.translated(Vec3::new(0., 0., -6.));
    let plate = springrep::primitive::cylinder(&at, 8., 6., "plate").map_err(e)?;
    union(&plate, &ball("ball")?).map_err(e)
}

/// The frustum from radius 2 at z = 0 to 4 at z = 3, capped by the ball of radius 5 about the
/// origin above z = 3: the cone `cap.side` meets the sphere `cap.dome` in the rim (4, 3).
pub fn capped_frustum() -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let rim = 3f64.atan2(4.);
    turned(
        vec![
            NamedSegment::line(p(0., 0.), p(2., 0.), "bottom"),
            NamedSegment::line(p(2., 0.), p(4., 3.), "side"),
            NamedSegment::arc(p(0., 0.), 5., rim, TAU / 4. - rim, "dome"),
            NamedSegment::line(p(0., 5.), p(0., 0.), "axis"),
        ],
        "cap",
    )
}

/// The upper half of the ball of radius 5 about the origin: its flat face `half.flat` meets its
/// sphere `half.skin` in the rim (5, 0).
pub fn hemisphere() -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    turned(
        vec![
            NamedSegment::line(p(0., 0.), p(5., 0.), "flat"),
            NamedSegment::arc(p(0., 0.), 5., 0., TAU / 4., "skin"),
            NamedSegment::line(p(0., 5.), p(0., 0.), "axis"),
        ],
        "half",
    )
}

/// The spherical sector of the ball of radius 5 about the origin inside the cone about the z axis
/// of half angle α, cos α = `cos_a`: its cone `sector.cone` meets its sphere `sector.skin` in the
/// rim at 5 from the origin.
pub fn sector(cos_a: f64) -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let a = cos_a.acos();
    let rim = p(5. * a.sin(), 5. * cos_a);
    turned(
        vec![
            NamedSegment::line(p(0., 0.), rim, "cone"),
            NamedSegment::arc(p(0., 0.), 5., TAU / 4. - a, a, "skin"),
            NamedSegment::line(p(0., 5.), p(0., 0.), "axis"),
        ],
        "sector",
    )
}

/// An edge of `b` on the circle about the z axis of radius `rho` at height `z`. A rim split in
/// pieces between the same two faces gives the first: a blend takes every edge between its faces.
pub fn rim(b: &Brep, rho: f64, z: f64) -> Result<EdgeId, String> {
    let on = |id: EdgeId| match &b.edge(id).curve {
        Some(Curve::Circle(c)) => {
            let o = c.frame.origin;
            (c.radius - rho).abs() < 1e-6
                && o.x.abs() < 1e-6
                && o.y.abs() < 1e-6
                && (o.z - z).abs() < 1e-6
        }
        _ => false,
    };
    b.edge_ids()
        .find(|&id| on(id))
        .ok_or_else(|| format!("no circle of radius {rho} at z = {z}"))
}

/// `body` with its rim of radius `rho` at height `z` chamfered with two distances: `on_first`
/// across the face named `first`, `on_other` across the other face.
pub fn rim_by_two_distances(
    body: &Brep,
    (rho, z): (f64, f64),
    first: &str,
    on_first: f64,
    on_other: f64,
) -> Result<Brep, String> {
    let face = faces_named(body, first)
        .into_iter()
        .next()
        .ok_or_else(|| format!("no face named {first}"))?;
    let edge = rim(body, rho, z)?;
    chamfer_two_edges(body, edge, face, on_first, on_other, "chamfer").map_err(e)
}

/// `body` with the one edge where its faces named `first` and `other` meet chamfered by
/// `distance` across `first`, the chamfer at `angle` (radians) to `first`.
pub fn by_distance_and_angle(
    body: &Brep,
    first: &str,
    other: &str,
    distance: f64,
    angle: f64,
) -> Result<Brep, String> {
    let pair = EdgeName {
        a: first,
        b: other,
        exact: true,
    };
    let edge = match edges_named(body, &pair).map_err(e)?[..] {
        [edge] => edge,
        ref edges => return Err(format!("{} edges where {first} meets {other}", edges.len())),
    };
    let face = faces_named(body, first)
        .into_iter()
        .next()
        .ok_or_else(|| format!("no face named {first}"))?;
    chamfer_distance_angle(body, edge, face, distance, angle, "chamfer").map_err(e)
}

/// The sector's rim chamfered by `distance` down its cone and the chamfer at `angle` (radians) to
/// the cone.
pub fn sector_rim_by_angle(cos_a: f64, distance: f64, angle: f64) -> Result<Brep, String> {
    by_distance_and_angle(
        &sector(cos_a)?,
        "sector.cone",
        "sector.skin",
        distance,
        angle,
    )
}

/// The capped frustum's rim chamfered by `distance` down its cone and the chamfer at `angle` to
/// the cone.
pub fn capped_frustum_rim_by_angle(distance: f64, angle: f64) -> Result<Brep, String> {
    by_distance_and_angle(&capped_frustum()?, "cap.side", "cap.dome", distance, angle)
}

/// The hemisphere's rim chamfered by `distance` in across its flat face and the chamfer at `angle`
/// to that face.
pub fn hemisphere_rim_by_angle(distance: f64, angle: f64) -> Result<Brep, String> {
    by_distance_and_angle(&hemisphere()?, "half.flat", "half.skin", distance, angle)
}

/// The groove where the ball stands in the conical seat chamfered by `distance` up the seat's cone
/// and the chamfer at `angle` to the cone.
pub fn conical_seat_by_angle(distance: f64, angle: f64) -> Result<Brep, String> {
    by_distance_and_angle(
        &ball_in_conical_seat()?,
        "seat.cone",
        "ball.skin",
        distance,
        angle,
    )
}

/// The groove where the ball stands on the plate chamfered by `distance` out across the plate's
/// top and the chamfer at `angle` to the top.
pub fn plate_rim_by_angle(distance: f64, angle: f64) -> Result<Brep, String> {
    by_distance_and_angle(&ball_on_plate()?, "plate.end", "ball.skin", distance, angle)
}

/// The sign of each axis for each of the octahedron's eight faces, face `i` named `f<i>`.
pub const OCTANTS: [[f64; 3]; 8] = [
    [1., 1., 1.],
    [-1., 1., 1.],
    [1., -1., 1.],
    [-1., -1., 1.],
    [1., 1., -1.],
    [-1., 1., -1.],
    [1., -1., -1.],
    [-1., -1., -1.],
];

/// The octahedron |x| + |y| + |z| ≤ 10, the common part of the eight half-spaces its faces bound;
/// the face in the octant of `OCTANTS[i]` is `f<i>.start`.
pub fn octahedron() -> Result<Brep, String> {
    let mut solid: Option<Brep> = None;
    for (i, s) in OCTANTS.iter().enumerate() {
        let n = Vec3::from_array(*s) * (1. / 3f64.sqrt());
        let side = half_space(n * (10. / 3f64.sqrt()), -n, 60., &format!("f{i}"))?;
        solid = Some(match solid {
            None => side,
            Some(s) => springrep::boolean::intersection(&s, &side).map_err(e)?,
        });
    }
    Ok(solid.expect("eight faces"))
}

/// The octahedron's twelve edges, as the pairs of its faces' names that meet in them: faces whose
/// octants differ in one sign.
pub fn octahedron_edges() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for i in 0..8 {
        for j in i + 1..8 {
            let differ = (0..3).filter(|&k| OCTANTS[i][k] != OCTANTS[j][k]).count();
            if differ == 1 {
                out.push((format!("f{i}.start"), format!("f{j}.start")));
            }
        }
    }
    out
}
