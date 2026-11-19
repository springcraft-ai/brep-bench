//! A transition from the square −20 ≤ x, y ≤ 20 at z = 0 to the circle of radius 15 at z = 30:
//! four flat triangles and four pieces of oblique cones, the hull of the two.
use springrep::loft::{Section, loft_developable};
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let sections = [
        Section {
            profile: Profile::rect(Vec2::ZERO, 40., 40.),
            frame: at([0., 0., 0.]),
        },
        Section {
            profile: Profile::circle(Vec2::ZERO, 30.),
            frame: at([0., 0., 30.]),
        },
    ];
    // each ruling joins points where the two outlines' tangents run the same way: a side of the
    // square meets the circle's nearest point, and a corner the quarter circle facing it
    loft_developable(&sections, "transition").map_err(e)
}
