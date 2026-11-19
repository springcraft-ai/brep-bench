//! A pipe of radius 2 along three edges of the cube of side 30, (0, 0, 0) → (30, 0, 0) →
//! (30, 30, 0) → (30, 30, 30), mitered at both corners.
use std::f64::consts::FRAC_PI_2;

use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::{Leg, along};
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let disk = Profile::circle(Vec2::ZERO, 4.);
    let start = frame([0., 0., 0.], [1., 0., 0.], [0., 0., 1.])?;
    // a quarter turn about z from +x to +y, then one about x from +y to +z
    let legs = [
        Leg::Line { length: 30. },
        Leg::Miter {
            angle: FRAC_PI_2,
            axis: Vec3::Z,
        },
        Leg::Line { length: 30. },
        Leg::Miter {
            angle: FRAC_PI_2,
            axis: Vec3::X,
        },
        Leg::Line { length: 30. },
    ];
    along(&disk, &start, &legs, "pipe").map_err(e)
}
