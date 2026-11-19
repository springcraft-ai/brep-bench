//! The upper half of the ellipsoid x²/36 + y²/16 + z²/9 ≤ 1, cut by the plane z = 0, keeping what
//! lies above it.
use springrep::boolean::split;
use springrep::math::Vec3;
use springrep::primitive::ellipsoid;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let whole = ellipsoid(&at([0., 0., 0.]), Vec3::new(6., 4., 3.), "ellipsoid").map_err(e)?;
    // the part on the plane normal's side comes first
    let (above, _) = split(&whole, Vec3::ZERO, Vec3::Z).map_err(e)?;
    Ok(above)
}
