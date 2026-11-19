//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a torus
//! (radii 2.5 and 2 about (5, 0, 0), its axis leaning 25° toward +y) passes through as it turns
//! about the z axis from −140° to 140°.
use springrep::boolean::difference;
use springrep::math::{Frame, Vec3};
use springrep::primitive::{cylinder, torus};
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, sector_block, turning, v3};

pub fn build() -> Result<Brep, String> {
    let lean = deg(25.0);
    let axis =
        Frame::new(v3(5.0, 0.0, 0.0), v3(0.0, lean.sin(), lean.cos()), Vec3::X).map_err(e)?;
    let cutter = torus(&axis, 2.5, 2.0, "cutter").map_err(e)?;
    // No sweep of the whole tool: a turn keeps each point's distance from its axis, so only the
    // cutter beyond the block's inner radius, 8, reaches the block, and only that part is swept.
    let core = cylinder(&crate::shared::at([0.0, 0.0, -10.0]), 7.9, 20.0, "core").map_err(e)?;
    let reach = difference(&cutter, &core).map_err(e)?;
    let swept = solid(&reach, &turning(deg(-140.0), deg(140.0)), "turn").map_err(e)?;
    difference(&sector_block()?, &swept).map_err(e)
}
