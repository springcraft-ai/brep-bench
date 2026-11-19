//! The plate (−15, −15, 0)–(15, 15, 4) less everything a coil spring (a ball of radius 0.4 swept
//! three turns along the helix of radius 2 rising 1.5 a turn) passes through as it slides along
//! (3, 1, 18).
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::primitive::sphere;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{boxed, screwing, v3};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&crate::shared::at([2.0, 0.0, -12.0]), 0.4, "ball").map_err(e)?;
    let mut plate = boxed([-15.0, -15.0, 0.0], [15.0, 15.0, 4.0], "plate")?;
    // No sweep of the whole spring: it is swept in six half turns, each of whose shadows along the
    // push does not overlap itself, and the plate loses what each of them sweeps.
    for k in 0..6 {
        let from = PI * k as f64;
        let half = solid(&ball, &screwing(1.5, from, from + PI), "coil").map_err(e)?;
        let hole = solid(&half, &Motion::Slide(v3(3.0, 1.0, 18.0)), "push").map_err(e)?;
        plate = difference(&plate, &hole).map_err(e)?;
    }
    Ok(plate)
}
