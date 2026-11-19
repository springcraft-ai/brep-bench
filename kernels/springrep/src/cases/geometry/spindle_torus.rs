//! The spindle torus about the z axis, middle circle radius 3 and tube radius 5, counted once: its
//! tube crosses the axis, and the solid is bounded by the tube's outer arc, turned.
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    torus(&at([0., 0., 0.]), 3., 5., "apple").map_err(e)
}
