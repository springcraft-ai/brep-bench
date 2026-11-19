//! The solid a rounded block fills as it slides along (8, 5, 3): everything within 1 of the box
//! (1, 1, 1)–(9, 5, 3), slid. springrep's offset grows the box into the rounded block.
use springrep::edit::offset;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, v3};

pub fn build() -> Result<Brep, String> {
    let core = boxed([1.0, 1.0, 1.0], [9.0, 5.0, 3.0], "block")?;
    let block = offset(&core, 1.0, "rounded").map_err(e)?;
    solid(&block, &Motion::Slide(v3(8.0, 5.0, 3.0)), "swept").map_err(e)
}
