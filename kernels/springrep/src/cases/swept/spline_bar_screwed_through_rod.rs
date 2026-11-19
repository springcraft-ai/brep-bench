//! The rod of radius 5 about the z axis from z = 0 to 20 less everything a prism of closed
//! B-spline section in the plane y = 0 passes through as it is screwed about the z axis, rising
//! 6 a turn.
use springrep::boolean::difference;
use springrep::geom::BSpline;
use springrep::math::{Frame, Vec3};
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::{extrude, solid};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, rod, screwing, v3, within};

pub fn build() -> Result<Brep, String> {
    let points = [
        (3.5, 0.0),
        (4.5, -1.5),
        (8.0, -1.5),
        (12.0, -1.0),
        (12.0, 1.0),
        (8.0, 1.5),
        (4.5, 1.5),
    ];
    // drawn in (x, z) on the plane y = −8, carried along +y
    let ctrl = (0..points.len() + 3)
        .map(|i| points[i % points.len()])
        .map(|(x, z)| v3(x, z, 0.0))
        .collect();
    let knots = (-3..=10).map(f64::from).collect();
    let curve = BSpline::periodic_cubic(knots, ctrl).map_err(e)?;
    let side = Segment::Spline {
        curve,
        reversed: false,
    };
    let section = Profile::outline(vec![NamedSegment::new(side, "side")]);
    let sketch = Frame::new(v3(0.0, -8.0, 0.0), -Vec3::Y, Vec3::X).map_err(e)?;
    let bar = extrude(&section, &sketch, -16.0, "bar").map_err(e)?;
    // No sweep of the whole tool: a screw keeps each point's distance from the z axis, so only the
    // tool within 5.5 of it reaches the rod, and only that part is swept.
    let tip = within(&bar, 5.5)?;
    let swept = solid(&tip, &screwing(6.0, deg(-90.0), deg(1350.0)), "screw").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
