//! The box (−6, −6, −8)–(36, 26, 0) less everything a bull-nose mill leaning 30° passes through
//! as a frame riding a cubic Bézier curve carries it, turning with the curve's tangent.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::profile::Profile;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{arc, boxed, carried, deg, line, outline, revolved, v3};

/// The bull-nose mill's half meridian, drawn out from its axis and along it from its flat end's
/// middle: flat to 1.5, a round of radius 0.5, a side of radius 2 up to 3.
fn bull_nose() -> Profile {
    outline(vec![
        line((0.0, 0.0), (1.5, 0.0), "end"),
        arc((1.5, 0.5), 0.5, -PI / 2.0, PI / 2.0, "round"),
        line((2.0, 0.5), (2.0, 3.0), "side"),
        line((2.0, 3.0), (0.0, 3.0), "top"),
        line((0.0, 3.0), (0.0, 0.0), "axis"),
    ])
}

pub fn build() -> Result<Brep, String> {
    let stock = boxed([-6.0, -6.0, -8.0], [36.0, 26.0, 0.0], "block")?;
    let (s, c) = deg(30.0).sin_cos();
    let mill = revolved(
        &bull_nose(),
        v3(0.0, 0.0, -1.0),
        v3(s, 0.0, c),
        v3(c, 0.0, -s),
        "mill",
    )?;
    let swept = solid(&mill, &carried()?, "carried").map_err(e)?;
    difference(&stock, &swept).map_err(e)
}
