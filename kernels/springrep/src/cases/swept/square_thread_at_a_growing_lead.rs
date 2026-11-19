//! The rod of radius 5 about the z axis from z = 0 to 20 less everything the bar 4 ≤ x ≤ 12, |y|
//! ≤ 8, |z| ≤ 1 passes through as it turns by θ about the z axis while rising 6s + 0.3s², s =
//! θ/360°.
use springrep::boolean::difference;
use springrep::geom::{BSpline, Curve};
use springrep::math::Vec3;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, deg, rod, turning, v3, within};

pub fn build() -> Result<Brep, String> {
    // Turned by θ = −90° + 1350°τ it has risen h = 6s + 0.3s², s = θ/360°: the quadratic Bézier
    // through h(0) and h(1) whose middle point is h(0) + h′(0)/2.
    let h = |s: f64| 6.0 * s + 0.3 * s * s;
    let (s0, s1) = (-0.25, 3.5);
    let slope = (6.0 + 0.6 * s0) * (s1 - s0);
    let ctrl = vec![
        v3(0.0, 0.0, h(s0)),
        v3(0.0, 0.0, h(s0) + slope / 2.0),
        v3(0.0, 0.0, h(s1)),
    ];
    let knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let rise = Curve::BSpline(BSpline::new(2, knots, ctrl).map_err(e)?);
    let bar = boxed([4.0, -8.0, -1.0 + h(s0)], [12.0, 8.0, 1.0 + h(s0)], "bar")?;
    // No sweep of the whole tool: a screw keeps each point's distance from the z axis, so only the
    // tool within 5.5 of it reaches the rod, and only that part is swept.
    let tip = within(&bar, 5.5)?;
    let motion = Motion::Then(
        Box::new(turning(deg(-90.0), deg(1260.0))),
        Box::new(Motion::Along {
            path: rise,
            range: (0.0, 1.0),
            up: Vec3::X,
        }),
    );
    let swept = solid(&tip, &motion, "thread").map_err(e)?;
    difference(&rod()?, &swept).map_err(e)
}
