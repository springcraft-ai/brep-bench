//! A hollow square tube tapering along y from 24 × 24 (hole 20 × 20) at y = 0 to 12 × 12 (hole
//! 8 × 8) at y = 40, every face flat.
use springrep::loft::{Section, loft};
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::topo::Brep;

use crate::shared::{e, frame};

/// The square of side `outside` with a square hole of side `hole`, on the plane y = `y`.
fn hollow(y: f64, outside: f64, hole: f64) -> Result<Section, String> {
    let hole = Profile::rect(Vec2::ZERO, hole, hole).loops.remove(0);
    Ok(Section {
        profile: Profile::rect(Vec2::ZERO, outside, outside).with_holes([hole]),
        frame: frame([0., y, 0.], [0., 1., 0.], [1., 0., 0.])?,
    })
}

pub fn build() -> Result<Brep, String> {
    loft(
        &[hollow(0., 24., 20.)?, hollow(40., 12., 8.)?],
        true,
        "reducer",
    )
    .map_err(e)
}
