//! An L-shaped block 8 tall, its inside corner rounded to radius 2; the round then deleted, the
//! corner sharp again.
use springrep::blend::fillet;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::edit::delete_feature;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let outline: Vec<Vec2> = [
        (0., 0.),
        (20., 0.),
        (20., 5.),
        (5., 5.),
        (5., 15.),
        (0., 15.),
    ]
    .iter()
    .map(|&(x, y)| Vec2::new(x, y))
    .collect();
    let sides = ["front", "end", "tread", "riser", "back", "side"];
    let l = extrude(
        &Profile::polygon(&outline, &sides),
        &at([0., 0., 0.]),
        8.,
        "l",
    )
    .map_err(e)?;
    let rounded = fillet(&l, &[("l.tread", "l.riser")], 2., "round").map_err(e)?;
    delete_feature(&rounded, "round")
}
