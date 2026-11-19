//! A square column from z = 0 to 40 of side 30 − z/2, turned 4.5° per unit of height about the
//! z axis: from the square of side 30 to the square of side 10, half a turn on.
use std::f64::consts::PI;

use springrep::loft::{Guide, Section, loft_guided};
use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::Helix;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // each corner's conical spiral about z, right-handed: half a turn while rising 40, its
    // distance from the axis falling from 15√2 to 5√2
    let screw = Helix {
        origin: Vec3::ZERO,
        axis: Vec3::Z,
        rise: 40. / PI,
        sign: 1.,
    };
    let growth = -10. * 2f64.sqrt() / PI;
    let guides = [(15., 15.), (-15., 15.), (-15., -15.), (15., -15.)]
        .iter()
        .map(|&(x, y)| {
            Ok(Guide {
                curve: screw.spiral(Vec3::new(x, y, 0.), PI, growth).map_err(e)?,
                range: (0., PI),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let square = |z: f64, side: f64| Section {
        profile: Profile::rect(Vec2::ZERO, side, side),
        frame: at([0., 0., z]),
    };
    loft_guided(
        &[square(0., 30.), square(40., 10.)],
        &guides,
        None,
        "column",
    )
    .map_err(e)
}
