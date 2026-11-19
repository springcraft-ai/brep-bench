//! A 10 × 10 bar along (0, 0, 0) → (40, 0, 0) → (20, 20√3, 0), mitered at the corner in the
//! plane x + √3y = 40 that bisects it.
use std::f64::consts::PI;

use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::{Leg, along};
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let square = Profile::rect(Vec2::ZERO, 10., 10.);
    // the square stands on the plane x = 0, its sides along y and z
    let start = frame([0., 0., 0.], [1., 0., 0.], [0., 0., 1.])?;
    let legs = [
        Leg::Line { length: 40. },
        Leg::Miter {
            angle: 2. * PI / 3.,
            axis: Vec3::Z,
        },
        Leg::Line { length: 40. },
    ];
    along(&square, &start, &legs, "bar").map_err(e)
}
