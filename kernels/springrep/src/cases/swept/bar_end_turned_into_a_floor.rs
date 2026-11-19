//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a leaning
//! bar, its end a 3 × 4 rectangle turned 5° in its plane, passes through as it turns about the z
//! axis from −140° to 140°.
use springrep::boolean::difference;
use springrep::math::{Frame, Vec3};
use springrep::primitive::block;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{deg, sector_block, turning, v3};

pub fn build() -> Result<Brep, String> {
    let (lean, twist) = (deg(20.0), deg(5.0));
    let (u, v) = (v3(lean.cos(), 0.0, -lean.sin()), Vec3::Y);
    let n = v3(lean.sin(), 0.0, lean.cos());
    // the end's sides turned 5° in its plane: a′ = a cos 5° + b sin 5° from 0 to 3, and
    // b′ = b cos 5° − a sin 5° from −2 to 2
    let along = u * twist.cos() + v * twist.sin();
    let across = v * twist.cos() - u * twist.sin();
    let corner = v3(10.0, 0.0, 0.0) - across * 2.0;
    let end = Frame::new(corner, n, along).map_err(e)?;
    let bar = block(&end, v3(3.0, 4.0, 3.0), "bar").map_err(e)?;
    let swept = solid(&bar, &turning(deg(-140.0), deg(140.0)), "turn").map_err(e)?;
    difference(&sector_block()?, &swept).map_err(e)
}
