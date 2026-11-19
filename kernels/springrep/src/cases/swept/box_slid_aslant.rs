//! The solid the box (0, 0, 0)–(10, 6, 4) fills as it slides without turning along (8, 5, 3).
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, v3};

pub fn build() -> Result<Brep, String> {
    let block = boxed([0.0, 0.0, 0.0], [10.0, 6.0, 4.0], "box")?;
    solid(&block, &Motion::Slide(v3(8.0, 5.0, 3.0)), "swept").map_err(e)
}
