//! The spherical sector of the ball of radius 5 inside the cone of half angle α, cos α = 3/5, its
//! rim chamfered with two distances, 2 down the cone and 1 on the ball.
use springrep::topo::Brep;

use crate::shared::chamfer::{rim_by_two_distances, sector};

pub fn build() -> Result<Brep, String> {
    rim_by_two_distances(&sector(3. / 5.)?, (4., 3.), "sector.cone", 2., 1.)
}
