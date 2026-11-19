//! A blade, the rectangle 40 × 0.4 centred on the z axis, turned 6° per unit of height from z = 0
//! to 60: a whole turn, its corners on four helices.
use std::f64::consts::TAU;

use springrep::loft::{Guide, Section, loft_guided};
use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::Helix;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // each corner's helix about z, right-handed, rising 60 in its turn
    let screw = Helix {
        origin: Vec3::ZERO,
        axis: Vec3::Z,
        rise: 60. / TAU,
        sign: 1.,
    };
    let guides = [(20., -0.2), (20., 0.2), (-20., 0.2), (-20., -0.2)]
        .iter()
        .map(|&(x, y)| {
            Ok(Guide {
                curve: screw.spiral(Vec3::new(x, y, 0.), TAU, 0.).map_err(e)?,
                range: (0., TAU),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let blade = |z: f64| Section {
        profile: Profile::rect(Vec2::ZERO, 40., 0.4),
        frame: at([0., 0., z]),
    };
    loft_guided(&[blade(0.), blade(60.)], &guides, None, "blade").map_err(e)
}
