//! The rod of radius 5 about the z axis from z = 0 to 20 less everything an oblate spheroid
//! wheel (semi-axes 6, 6, 1.5 about (10.5, 0, 0), leaning 44.83°) passes through as it is
//! screwed about the z axis, rising 30 a turn.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, line, revolved, rod, screwing, v2, v3};

pub fn build() -> Result<Brep, String> {
    let h = (2.0 * PI * 4.75 / 30.0).atan();
    let axis = v3(0.0, h.cos(), -h.sin());
    // the half meridian out from the wheel's axis and along it: the ellipse of semi-axes 6 and 1.5
    let skin = Segment::Ellipse {
        center: v2(0.0, 0.0),
        rotation: 0.0,
        major: 6.0,
        minor: 1.5,
        start: -PI / 2.0,
        sweep: PI,
    };
    let half = Profile::outline(vec![
        NamedSegment::new(skin, "skin"),
        line((0.0, 1.5), (0.0, -1.5), "axis"),
    ]);
    // drawn on the side away from the rod, so the wheel's seam stays clear of the cut
    let wheel = revolved(&half, v3(10.5, 0.0, 0.0), axis, v3(1.0, 0.0, 0.0), "wheel")?;
    let swept = solid(&wheel, &screwing(30.0, deg(-90.0), deg(330.0)), "screw").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
