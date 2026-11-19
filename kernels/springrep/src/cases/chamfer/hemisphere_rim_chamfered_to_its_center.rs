//! The upper half of the ball of radius 5 about the origin, the rim of its flat face chamfered
//! with two distances: 5 across the flat face, all the way to its center, and 1 on the ball, so
//! the flat face shrinks to a point and the solid is a spherical sector.
use springrep::topo::Brep;

use crate::shared::chamfer::{hemisphere, rim_by_two_distances};

pub fn build() -> Result<Brep, String> {
    rim_by_two_distances(&hemisphere()?, (5., 0.), "half.flat", 5., 1.)
}
