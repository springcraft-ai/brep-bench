//! A transition from the square −20 ≤ x, y ≤ 20 at z = 0 to the octagon (the 28 × 28 square with
//! its corners cut, corners (±6, ±14), (±14, ±6)) at z = 30: four flat trapezoids from side to
//! side and four flat triangles from corner to cut side.
use springrep::loft::{Section, loft_developable};
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let corners = [
        (6., -14.),
        (14., -6.),
        (14., 6.),
        (6., 14.),
        (-6., 14.),
        (-14., 6.),
        (-14., -6.),
        (-6., -14.),
    ]
    .map(|(x, y)| Vec2::new(x, y));
    let sections = [
        Section {
            profile: Profile::rect(Vec2::ZERO, 40., 40.),
            frame: at([0., 0., 0.]),
        },
        Section {
            profile: Profile::polygon(&corners, &["octagon"]),
            frame: at([0., 0., 30.]),
        },
    ];
    // each ruling joins points where the two outlines' tangents run the same way: a corner of the
    // square meets the whole cut side facing it
    loft_developable(&sections, "hood").map_err(e)
}
