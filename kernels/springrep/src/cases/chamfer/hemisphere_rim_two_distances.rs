//! The upper half of the ball of radius 5 about the origin, the rim of its flat face chamfered
//! with two distances, 2 across the flat face and 1 on the ball.
use springrep::topo::Brep;

use crate::shared::chamfer::{hemisphere, rim_by_two_distances};

pub fn build() -> Result<Brep, String> {
    rim_by_two_distances(&hemisphere()?, (5., 0.), "half.flat", 2., 1.)
}
