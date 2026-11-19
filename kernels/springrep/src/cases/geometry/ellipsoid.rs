//! The solid ellipsoid x²/36 + y²/16 + z²/9 ≤ 1: semi-axes 6, 4 and 3 along x, y and z.
use springrep::math::Vec3;
use springrep::primitive::ellipsoid;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    ellipsoid(&at([0., 0., 0.]), Vec3::new(6., 4., 3.), "ellipsoid").map_err(e)
}
