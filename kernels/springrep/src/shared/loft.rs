//! Helpers for the loft cases: a solid whose level sections are rectangles centred on the z axis,
//! their corners on four mirrored cubic Bézier guides.
use springrep::geom::{Curve, NurbsCurve};
use springrep::loft::{Guide, Section, loft_guided};
use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::topo::Brep;

use super::{at, e};

/// The rectangle |x| ≤ p, |y| ≤ q at height z, for the guide's point (p, q, z).
fn rectangle([p, q, z]: [f64; 3]) -> Section {
    Section {
        profile: Profile::rect(Vec2::ZERO, 2. * p, 2. * q),
        frame: at([0., 0., z]),
    }
}

/// The solid whose section at each height is the rectangle |x| ≤ p, |y| ≤ q, where the guide
/// through the corner with x, y > 0 is the cubic Bézier `(p, q, z)` on `guide`, and the other three
/// guides its mirror images in the xz plane, the yz plane and both. A guide that ends on the axis
/// closes the solid in a point there.
pub fn on_guides(guide: [[f64; 3]; 4], feature: &str) -> Result<Brep, String> {
    let guides = [(1., 1.), (-1., 1.), (-1., -1.), (1., -1.)]
        .iter()
        .map(|&(sx, sy)| {
            let pts = guide
                .iter()
                .map(|g| Vec3::new(sx * g[0], sy * g[1], g[2]))
                .collect();
            let knots = vec![0., 0., 0., 0., 1., 1., 1., 1.];
            let cubic = NurbsCurve::new(3, knots, pts, None).map_err(e)?;
            Ok(Guide {
                curve: Curve::Nurbs(Box::new(cubic)),
                range: (0., 1.),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let [foot, .., top] = guide;
    if top[0] == 0. && top[1] == 0. {
        let apex = Some(Vec3::new(0., 0., top[2]));
        loft_guided(&[rectangle(foot)], &guides, apex, feature).map_err(e)
    } else {
        loft_guided(&[rectangle(foot), rectangle(top)], &guides, None, feature).map_err(e)
    }
}
