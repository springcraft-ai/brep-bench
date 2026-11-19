//! A frustum capped by a ball, its rim chamfered 2 down the cone at 45° to the cone's side.
use springrep::topo::Brep;

use crate::shared::chamfer::capped_frustum_rim_by_angle;

pub fn build() -> Result<Brep, String> {
    capped_frustum_rim_by_angle(2., 45f64.to_radians())
}
