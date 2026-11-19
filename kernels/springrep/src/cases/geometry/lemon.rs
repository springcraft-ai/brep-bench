//! The lemon: the circular segment of the disk of radius 5 about (−3, 0) in the xz plane on the
//! side x ≥ 0, cut off by its chord on the z axis from (0, −4) to (0, 4), turned a whole turn about
//! the z axis. Its arc's circle lies across the axis from the arc.
use springrep::math::{Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::revolve;
use springrep::topo::Brep;
use std::f64::consts::TAU;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the arc runs from (0, −4) out through (2, 0) to (0, 4): ±acos(3/5) about (−3, 0)
    let reach = (3f64 / 5.).acos();
    let segment = Profile::outline(vec![
        NamedSegment::arc(Vec2::new(-3., 0.), 5., -reach, 2. * reach, "skin"),
        NamedSegment::line(Vec2::new(0., 4.), Vec2::new(0., -4.), "axis"),
    ]);
    let xz = frame([0., 0., 0.], [0., -1., 0.], [1., 0., 0.])?;
    revolve(&segment, &xz, Vec3::ZERO, Vec3::Z, TAU, "lemon").map_err(e)
}
