//! The lens common to the balls of radius 5 about (0, 0, 3) and (0, 0, −3), its rim chamfered with
//! two distances, 2 on its upper face and 1 on its lower.
use springrep::boolean::intersection;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::chamfer::rim_by_two_distances;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    // the upper face is the ball below's, the lower face the ball above's
    let below = sphere(&at([0., 0., -3.]), 5., "below").map_err(e)?;
    let above = sphere(&at([0., 0., 3.]), 5., "above").map_err(e)?;
    let body = intersection(&below, &above).map_err(e)?;
    rim_by_two_distances(&body, (4., 0.), "below.skin", 2., 1.)
}
