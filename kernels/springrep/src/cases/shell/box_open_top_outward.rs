//! The box from the origin to (10, 8, 6) shelled outward to a wall 1 thick, open at its top, its
//! outside rounded: the box grown by 1 and cut off at z = 6, less the box.
use springrep::edit::{Wall, shell_rounded};
use springrep::math::Vec3;
use springrep::primitive::block;
use springrep::topo::Brep;

use crate::shared::shell::faces;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([0., 0., 0.]), Vec3::new(10., 8., 6.), "box").map_err(e)?;
    let open = faces(&b, &["box.end"])?;
    shell_rounded(&b, &open, 1., Wall::Outward, "shell").map_err(e)
}
