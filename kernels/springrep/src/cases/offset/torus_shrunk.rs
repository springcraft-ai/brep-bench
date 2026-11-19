//! The torus about the z axis of middle radius 10 and tube radius 3, shrunk by 1.5: the torus of
//! tube radius 1.5 about the same middle circle.
use springrep::edit::offset;
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ring = torus(&at([0., 0., 0.]), 10., 3., "ring").map_err(e)?;
    offset(&ring, -1.5, "shrunk").map_err(e)
}
