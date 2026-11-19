//! A cylinder of radius 10 from z = 0 to z = 20, its side parted at z = 10 and each half drafted
//! 3° away from the parting line: two frustums joined at their wide ends.
use springrep::edit::draft_parted;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::draft::carrying;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let post = cylinder(&at([0., 0., 0.]), 10., 20., "post").map_err(e)?;
    let side = carrying(&post, "post.side")?;
    // parted on the plane z = 10, the upper half pulled along +z and the lower along −z
    draft_parted(&post, &side, &at([0., 0., 10.]), 3f64.to_radians()).map_err(e)
}
