//! A closed hollow dome (a plate of radius 20 and a hemispherical shell from 18 to 20 about the
//! origin, a half-ball cavity inside) with a post of radius 3 extruded up from the cavity's floor
//! until it meets the inside of the dome, and joined to it.
use std::f64::consts::{FRAC_PI_2, TAU};

use springrep::boolean::{difference, union};
use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::{Meet, extrude_to_next, revolve};
use springrep::topo::Brep;

use crate::shared::{at, e, frame};

/// The solid the meridian `segments`, in (ρ, z), sweeps about the z axis.
fn turned(segments: Vec<NamedSegment>, feature: &str) -> Result<Brep, String> {
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    revolve(
        &Profile::outline(segments),
        &xz,
        Vec3::ZERO,
        Vec3::Z,
        TAU,
        feature,
    )
    .map_err(e)
}

pub fn build() -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let outside = turned(
        vec![
            NamedSegment::line(p(0., -2.), p(20., -2.), "bottom"),
            NamedSegment::line(p(20., -2.), p(20., 0.), "rim"),
            NamedSegment::arc(p(0., 0.), 20., 0., FRAC_PI_2, "outer"),
            NamedSegment::line(p(0., 20.), p(0., -2.), "axis"),
        ],
        "dome",
    )?;
    let cavity = turned(
        vec![
            NamedSegment::line(p(0., 0.), p(18., 0.), "floor"),
            NamedSegment::arc(p(0., 0.), 18., 0., FRAC_PI_2, "inner"),
            NamedSegment::line(p(0., 18.), p(0., 0.), "axis"),
        ],
        "cavity",
    )?;
    let dome = difference(&outside, &cavity).map_err(e)?;
    // up from the floor, z = 0, until it meets the dome's inside
    let disk = Profile::circle(Vec2::ZERO, 6.);
    let post =
        extrude_to_next(&disk, &at([0., 0., 0.]), false, &dome, Meet::Into, "post").map_err(e)?;
    union(&dome, &post).map_err(e)
}
