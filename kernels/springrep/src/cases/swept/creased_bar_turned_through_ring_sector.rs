//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a bar
//! drafted 2° both ways from a plane leaning 20° across the turn passes through as it turns
//! about the z axis from −140° to 140°.
use springrep::boolean::{difference, intersection};
use springrep::math::{Frame, Vec3};
use springrep::primitive::block;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, sector_block, turning, v3};

/// The points `x` with `(x − p)·m ≥ 0`, within 50 of `p`.
fn half_space(p: Vec3, m: Vec3) -> Result<Brep, String> {
    let m = m.unit().ok_or("no normal")?;
    let x = m.cross(Vec3::X).unit().ok_or("a normal along x")?;
    let corner = Frame::new(p - (x + m.cross(x)) * 50.0, m, x).map_err(e)?;
    block(&corner, v3(100.0, 100.0, 50.0), "draft").map_err(e)
}

pub fn build() -> Result<Brep, String> {
    let (s, c) = deg(20.0).sin_cos();
    let (u, v, n) = (Vec3::X, v3(0.0, c, -s), v3(0.0, s, c));
    let t = deg(2.0).tan();
    let corner = Frame::new(u * 10.0 - v * 2.0 - n * 4.0, n, u).map_err(e)?;
    let mut bar = block(&corner, v3(3.0, 4.0, 8.0), "bar").map_err(e)?;
    // each side leans in by 2° from the plane through the bar's middle both ways: two half-spaces
    for side in [1.0, -1.0] {
        for (p, m) in [(u * 10.0, u), (u * 13.0, -u), (v * 2.0, -v), (v * -2.0, v)] {
            bar = intersection(&bar, &half_space(p, m - n * (side * t))?).map_err(e)?;
        }
    }
    let swept = solid(&bar, &turning(deg(-140.0), deg(140.0)), "turn").map_err(e)?;
    difference(&sector_block()?, &swept).map_err(e)
}
