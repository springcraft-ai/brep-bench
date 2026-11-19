//! A pipe of radius 1 along three turns of the helix of radius 10 about the z axis, rising 5 a
//! turn, each end closed by a half ball: everything within 1 of the helix.
use std::f64::consts::PI;

use springrep::math::{Point3, Vec3};
use springrep::primitive::sphere;
use springrep::sweep::{Helix, Motion, solid};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // everything within 1 of the helix is what the ball of radius 1 about its start passes through
    // as it is carried along it
    let ball = sphere(&at([10., 0., 0.]), 1., "ball").map_err(e)?;
    let screw = Helix {
        origin: Point3::ZERO,
        axis: Vec3::Z,
        rise: 5. / (2. * PI),
        sign: 1.,
    };
    let along = Motion::Screw {
        screw,
        from: 0.,
        to: 6. * PI,
    };
    solid(&ball, &along, "pipe").map_err(e)
}
