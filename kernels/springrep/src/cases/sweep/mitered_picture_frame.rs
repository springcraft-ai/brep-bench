//! A picture frame: a molding 8 wide and 6 tall with a 3 × 2 rabbet swept round the rectangle
//! (±30, ±20) in the xy plane, mitered at its corners.
use std::f64::consts::FRAC_PI_2;

use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::{Leg, along};
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the section on the plane x = −30 at the corner (−30, −20), facing along the first side: its
    // x is up (z) and its y outward (−y), so the section's point (s, z) is at (z, s)
    let start = frame([-30., -20., 0.], [1., 0., 0.], [0., 0., 1.])?;
    let section = [(0., 2.), (3., 2.), (3., 0.), (8., 0.), (8., 6.), (0., 6.)];
    let points: Vec<Vec2> = section.iter().map(|&(s, z)| Vec2::new(z, s)).collect();
    let names = [
        "rabbet_top",
        "rabbet_side",
        "bottom",
        "outside",
        "top",
        "inside",
    ];
    let molding = Profile::polygon(&points, &names);
    let corner = Leg::Miter {
        angle: FRAC_PI_2,
        axis: Vec3::Z,
    };
    let legs = [
        Leg::Line { length: 60. },
        corner,
        Leg::Line { length: 40. },
        corner,
        Leg::Line { length: 60. },
        corner,
        Leg::Line { length: 40. },
        corner,
    ];
    along(&molding, &start, &legs, "frame").map_err(e)
}
