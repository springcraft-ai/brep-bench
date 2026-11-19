//! The L-shaped plate on the outline (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), 8 thick,
//! shrunk by 1: its convex edges sharp, the inside corner's edge a concave quarter cylinder.
use springrep::edit::offset;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::offset::l_block;

pub fn build() -> Result<Brep, String> {
    offset(&l_block()?, -1., "shrunk").map_err(e)
}
