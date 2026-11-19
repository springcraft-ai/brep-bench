//! A hollow square duct narrowing round a quarter turn of radius 30 about the z axis, its outer
//! half-width 9 → 6 linearly with the angle, its wall 2.
use std::f64::consts::{FRAC_PI_2, PI};

use springrep::loft::{Guide, Section, loft_guided};
use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::Helix;
use springrep::topo::Brep;

use crate::shared::{e, frame};

/// The square of half-width `h` with the square hole of half-width `h − 2`.
fn hollow(h: f64) -> Profile {
    let hole = Profile::rect(Vec2::ZERO, 2. * h - 4., 2. * h - 4.)
        .loops
        .remove(0);
    Profile::rect(Vec2::ZERO, 2. * h, 2. * h).with_holes([hole])
}

pub fn build() -> Result<Brep, String> {
    // the half-width's change per radian of the quarter turn
    let dh = -6. / PI;
    let mut guides = Vec::new();
    for h in [9., 7.] {
        // each corner, (30 ± h) from the z axis at height ±h, runs on a spiral about z: its
        // distance from the axis and its height each change by ±dh per radian
        for (out, up) in [(1., -1.), (1., 1.), (-1., 1.), (-1., -1.)] {
            let screw = Helix {
                origin: Vec3::ZERO,
                axis: Vec3::Z,
                rise: up * dh,
                sign: 1.,
            };
            let corner = Vec3::new(30. + out * h, 0., up * h);
            guides.push(Guide {
                curve: screw.spiral(corner, FRAC_PI_2, out * dh).map_err(e)?,
                range: (0., FRAC_PI_2),
            });
        }
    }
    let sections = [
        Section {
            profile: hollow(9.),
            frame: frame([30., 0., 0.], [0., 1., 0.], [1., 0., 0.])?,
        },
        Section {
            profile: hollow(6.),
            frame: frame([0., 30., 0.], [-1., 0., 0.], [0., 1., 0.])?,
        },
    ];
    loft_guided(&sections, &guides, None, "duct").map_err(e)
}
