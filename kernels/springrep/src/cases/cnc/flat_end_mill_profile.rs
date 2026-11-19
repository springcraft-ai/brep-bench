//! A plate, the box (0, 0, −15)–(120, 90, 0), less what a flat end mill of radius 3 sweeps
//! running round a cross with its corners rounded to 6, at depths 3, 6 and 9, plunging from
//! (60, 10, 5) and rising back there.
use std::f64::consts::FRAC_PI_2;

use springrep::boolean::difference_all;
use springrep::math::{Frame, Point3, Transform, Vec3};
use springrep::primitive::cylinder;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::cnc::plate;
use crate::shared::e;
use crate::shared::swept::{shifted, v3};

const CROSS: [[f64; 2]; 12] = [
    [45., 10.],
    [75., 10.],
    [75., 30.],
    [100., 30.],
    [100., 60.],
    [75., 60.],
    [75., 80.],
    [45., 80.],
    [45., 60.],
    [20., 60.],
    [20., 30.],
    [45., 30.],
];

/// A move of the program: straight to a point, or round the upright line through `center` by
/// `angle` (anticlockwise from above when positive).
enum Move {
    Line(Point3),
    Arc { center: Point3, angle: f64 },
}

/// What `mill`, drawn with its tip at the origin, sweeps in each of `moves`, run in turn from
/// `start`. The program's moves are pieces one after another, so each is swept on its own.
fn sweep(mill: &Brep, start: Point3, moves: &[Move]) -> Result<Vec<Brep>, String> {
    let mut tip = start;
    let mut cuts = Vec::new();
    for m in moves {
        // a turn about an arc's center carries the mill as the slide round the arc does, the mill
        // being round about its own upright axis
        let (motion, end) = match *m {
            Move::Line(to) => (Motion::Slide(to - tip), to),
            Move::Arc { center, angle } => (
                Motion::Turn {
                    origin: center,
                    axis: Vec3::Z,
                    from: 0.0,
                    to: angle,
                },
                Transform::rotation(center, Vec3::Z, angle).point(tip),
            ),
        };
        cuts.push(solid(&shifted(mill, tip), &motion, "move").map_err(e)?);
        tip = end;
    }
    Ok(cuts)
}

/// Once anticlockwise round the polygon through `corners`, each corner rounded by a quarter circle
/// of radius `round`, at the height of `start`, from `start` on the side from the first corner to
/// the second and back to it.
fn lap(corners: &[[f64; 2]], round: f64, start: Point3) -> Vec<Move> {
    let n = corners.len();
    let corner = |i: usize| v3(corners[i % n][0], corners[i % n][1], start.z);
    let mut moves = Vec::new();
    for i in 1..=n {
        let (prev, c, next) = (corner(i - 1), corner(i), corner(i + 1));
        let ahead = (c - prev).unit_or_zero();
        let turn = ahead.cross((next - c).unit_or_zero()).z.signum();
        let from = c - ahead * round;
        moves.push(Move::Line(from));
        moves.push(Move::Arc {
            center: from + Vec3::Z.cross(ahead) * (round * turn),
            angle: turn * FRAC_PI_2,
        });
    }
    moves.push(Move::Line(start));
    moves
}

pub fn build() -> Result<Brep, String> {
    let stock = plate([0., 0., -15.], [120., 90., 0.])?;
    let mill = cylinder(&Frame::WORLD, 3., 20., "mill").map_err(e)?;
    let mut moves = Vec::new();
    for z in [-3., -6., -9.] {
        moves.push(Move::Line(v3(60., 10., z)));
        moves.extend(lap(&CROSS, 6., v3(60., 10., z)));
    }
    moves.push(Move::Line(v3(60., 10., 5.)));
    let cuts = sweep(&mill, v3(60., 10., 5.), &moves)?;
    difference_all(&stock, &cuts.iter().collect::<Vec<_>>()).map_err(e)
}
