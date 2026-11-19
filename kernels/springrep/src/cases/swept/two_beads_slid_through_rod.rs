//! The rod of radius 5 about the z axis from z = 0 to z = 20 less everything a wheel of two beads
//! passes through as it slides 40 along z: two tori on the line through (11.5, 0, −9) along y, the
//! first about (11.5, 0, −9) with radii 6 and 2, the second about (11.5, 1.5, −9) with radii 6 and
//! 1.5.
use springrep::boolean::difference;
use springrep::primitive::torus;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::swept::{rod, unite, v3};
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    let first = frame([11.5, 0.0, -9.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0])?;
    let second = frame([11.5, 1.5, -9.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0])?;
    let wheel = unite(&[
        &torus(&first, 6.0, 2.0, "bead").map_err(e)?,
        &torus(&second, 6.0, 1.5, "bead").map_err(e)?,
    ])?;
    let groove = solid(&wheel, &Motion::Slide(v3(0.0, 0.0, 40.0)), "groove").map_err(e)?;
    difference(&rod()?, &groove).map_err(e)
}
