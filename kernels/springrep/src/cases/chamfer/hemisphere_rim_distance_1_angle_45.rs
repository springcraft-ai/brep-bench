//! The upper half of a ball of radius 5, its rim chamfered 1 in across the flat face at 45° to it.
use springrep::topo::Brep;

use crate::shared::chamfer::hemisphere_rim_by_angle;

pub fn build() -> Result<Brep, String> {
    hemisphere_rim_by_angle(1., 45f64.to_radians())
}
