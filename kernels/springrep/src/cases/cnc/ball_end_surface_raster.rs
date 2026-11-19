//! A plate, the box (0, 0, −20)–(100, 80, 0), less what a ball-end mill of radius 3 sweeps in 41
//! zigzag raster passes over the surface z = −5 + 4 sin(πx/50) sin(πy/40), 944 aslant moves.
//!
//! springrep's sweep of a solid takes one motion at a time, and 944 of them are too many; so each
//! pass is built from other operations. Within one pass the ball's centre runs a polyline in the
//! plane y = 2j, and the upright mill covers the region R of that plane between the polyline and
//! the same polyline 27 higher (the shank's top). Below the shank's top, which never comes lower
//! than z = 21, the mill's sweep is R grown by a ball of radius 3: R slid 3 to each side of its
//! plane (a prism), and each bottom segment of R grown by the ball (a cylinder with a ball at each
//! end). The plunge, the rise and the moves from one pass to the next all run at x = −5 or
//! x = 105, where the mill stays 2 clear of the plate, so they cut nothing.
use std::f64::consts::PI;

use springrep::boolean::{difference, union_all};
use springrep::math::{Frame, Vec2, Vec3};
use springrep::primitive::{block, cylinder, sphere};
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

const RADIUS: f64 = 3.;
const SHANK_TOP: f64 = 27.;

/// The surface the passes follow, z over (x, y).
fn surface(x: f64, y: f64) -> f64 {
    -5. + 4. * (PI * x / 50.).sin() * (PI * y / 40.).sin()
}

/// What the mill sweeps along the pass at `y`, below its shank's top.
fn pass(y: f64) -> Result<Brep, String> {
    let centres: Vec<Vec3> = (0..=22)
        .map(|i| {
            let x = -5. + 5. * i as f64;
            Vec3::new(x, y, surface(x, y) + RADIUS)
        })
        .collect();
    let mut outline: Vec<Vec2> = centres.iter().map(|c| Vec2::new(c.x, c.z)).collect();
    outline.extend(
        centres
            .iter()
            .rev()
            .map(|c| Vec2::new(c.x, c.z + SHANK_TOP)),
    );
    let names: Vec<String> = (0..outline.len()).map(|i| format!("s{i}")).collect();
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    // the xz plane at y + 3, its normal −y, so the prism runs back to y − 3
    let plane = Frame::new(Vec3::new(0., y + RADIUS, 0.), -Vec3::Y, Vec3::X).map_err(e)?;
    let region = Profile::polygon(&outline, &names);
    let mut pieces = vec![extrude(&region, &plane, 2. * RADIUS, "prism").map_err(e)?];
    for c in &centres {
        pieces.push(sphere(&at(c.to_array()), RADIUS, "ball").map_err(e)?);
    }
    for w in centres.windows(2) {
        let d = w[1] - w[0];
        let base = Frame::from_axis(w[0], d).map_err(e)?;
        pieces.push(cylinder(&base, RADIUS, d.norm(), "tube").map_err(e)?);
    }
    union_all(&pieces.iter().collect::<Vec<_>>())
        .map_err(|err| format!("uniting the pass at y = {y}: {}", e(err)))
}

pub fn build() -> Result<Brep, String> {
    let mut plate = block(&at([0., 0., -20.]), Vec3::new(100., 80., 20.), "plate").map_err(e)?;
    for j in 0..=40 {
        let y = 2. * j as f64;
        plate = difference(&plate, &pass(y)?)
            .map_err(|err| format!("cutting the pass at y = {y}: {}", e(err)))?;
    }
    Ok(plate)
}
