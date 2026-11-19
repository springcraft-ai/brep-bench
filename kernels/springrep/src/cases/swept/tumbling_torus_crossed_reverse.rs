//! The solid the torus about (3, 0, 2) (radii 1 and 0.22) fills as it turns by 0.7t about the
//! upright line through (2, 0, 0) and by +t about a skew line, for t from −100° to 100°.
use springrep::topo::Brep;

use crate::shared::swept::crossed;

pub fn build() -> Result<Brep, String> {
    crossed(1.0)
}
