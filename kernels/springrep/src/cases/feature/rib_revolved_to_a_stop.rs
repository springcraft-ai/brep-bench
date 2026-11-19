//! A hub of radius 10 with a stop (the wedge within 25 of the axis between 120° and 135°), and a
//! rib, the rectangle 5 ≤ x ≤ 20, 0 ≤ z ≤ 4 on y = 0, turned about the z axis until it meets the
//! stop, all joined.
use std::f64::consts::PI;

use springrep::boolean::union;
use springrep::math::{Frame, Vec2, Vec3};
use springrep::primitive::cylinder;
use springrep::profile::Profile;
use springrep::sweep::{Meet, revolve, revolve_to_next};
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

/// The half-plane at angle `from` about the z axis, as a sketch plane: its x is the half-plane's
/// direction and its y the z axis.
fn half_plane(from: f64) -> Result<Frame, String> {
    frame(
        [0., 0., 0.],
        [from.sin(), -from.cos(), 0.],
        [from.cos(), from.sin(), 0.],
    )
}

/// The rectangle from (r0, 0) to (r1, h) in the half-plane's sketch coordinates.
fn rect(r0: f64, r1: f64, h: f64) -> Profile {
    Profile::rect(Vec2::new((r0 + r1) / 2., h / 2.), r1 - r0, h)
}

pub fn build() -> Result<Brep, String> {
    let hub = cylinder(&at([0., 0., 0.]), 10., 10., "hub").map_err(e)?;
    let deg = PI / 180.;
    let plane = half_plane(120. * deg)?;
    let stop = revolve(
        &rect(0., 25., 10.),
        &plane,
        Vec3::ZERO,
        Vec3::Z,
        15. * deg,
        "stop",
    )
    .map_err(e)?;
    let body = union(&hub, &stop).map_err(e)?;
    // from 0° toward +y until it meets the stop
    let rib = revolve_to_next(
        &rect(5., 20., 4.),
        &half_plane(0.)?,
        Vec3::ZERO,
        Vec3::Z,
        &body,
        Meet::Into,
        "rib",
    )
    .map_err(e)?;
    union(&body, &rib).map_err(e)
}
