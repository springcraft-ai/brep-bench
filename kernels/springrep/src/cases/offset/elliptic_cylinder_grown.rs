//! The ellipse x²/25 + y²/9 ≤ 1 extruded from z = 0 to z = 10, grown by 1: its side moves out onto
//! the ellipse's offset curve and its rims become canal surfaces about the ellipses.
use springrep::boolean::unify;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::offset::{elliptic_cylinder, grown};

pub fn build() -> Result<Brep, String> {
    // the ellipse is drawn as two halves; unified, the side and each rim are one face and one edge
    let drum = unify(&elliptic_cylinder()?).map_err(e)?;
    grown(&drum, 1.)
}
