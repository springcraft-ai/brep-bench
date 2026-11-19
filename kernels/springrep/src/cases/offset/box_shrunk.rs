//! The box from the origin to (10, 8, 6), shrunk by 1: every point at least 1 from its outside,
//! the box from (1, 1, 1) to (9, 7, 5).
use springrep::edit::offset;
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(10., 8., 6.), "box").map_err(e)?;
    offset(&b, -1., "shrunk").map_err(e)
}
