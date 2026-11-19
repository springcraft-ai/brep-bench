//! A disk plate of radius 8 with a boss of radius 3 on it, the concave edge at the boss's foot
//! rounded to radius 1; the round then deleted, the foot's edge sharp again.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = cylinder(&at([0., 0., -4.]), 8., 4., "plate").map_err(e)?;
    let boss = cylinder(&at([0., 0., 0.]), 3., 8., "boss").map_err(e)?;
    let part = union(&plate, &boss).map_err(e)?;
    let rounded = fillet(&part, &[("plate.end", "boss.side")], 1., "round").map_err(e)?;
    delete_feature(&rounded, "round")
}
