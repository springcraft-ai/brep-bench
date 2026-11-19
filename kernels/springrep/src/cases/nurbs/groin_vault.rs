//! A groin vault: the common part of the block on −12 ≤ x, y ≤ 12 under a weighted rational
//! barrel vault arching across y, and the same block turned a quarter turn about z.
use std::f64::consts::PI;

use springrep::boolean::intersection;
use springrep::math::{Point3, Transform, Vec3};
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::nurbs::vault;

pub fn build() -> Result<Brep, String> {
    let along_x = vault()?;
    let mut along_y = along_x.clone();
    along_y.transform(&Transform::rotation(Point3::ZERO, Vec3::Z, PI / 2.));
    intersection(&along_x, &along_y).map_err(e)
}
