//! A disk plate of radius 8 with a boss of radius 3 standing 8 on it; the boss's side and top then
//! deleted, the plate's top closing over its foot.
use springrep::boolean::union;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = cylinder(&at([0., 0., -4.]), 8., 4., "plate").map_err(e)?;
    let boss = cylinder(&at([0., 0., 0.]), 3., 8., "boss").map_err(e)?;
    let part = union(&plate, &boss).map_err(e)?;
    delete_feature(&part, "boss")
}
