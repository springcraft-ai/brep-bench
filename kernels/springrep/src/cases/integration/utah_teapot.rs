//! The Utah teapot: Newell's 32 bicubic Bézier patches made into four solids and united.
use springrep::boolean::union;
use springrep::geom::{Curve, NurbsSurface};
use springrep::math::Vec3;
use springrep::sew::{Piece, Span, sew};
use springrep::topo::Brep;

use crate::shared::e;

type Net = [[Vec3; 4]; 4];

fn v3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

/// The bicubic Bézier patch on `net`, row `i` along `u`.
fn patch(net: &Net) -> Result<NurbsSurface, String> {
    let knots = vec![0., 0., 0., 0., 1., 1., 1., 1.];
    let ctrl = net.iter().flatten().copied().collect();
    NurbsSurface::new((3, 3), (knots.clone(), knots), (4, 4), ctrl, None).map_err(e)
}

/// `net` turned a quarter turn clockwise about z, seen from above, `k` times.
fn turned(net: &Net, k: usize) -> Net {
    net.map(|row| {
        row.map(|mut p| {
            for _ in 0..k {
                p = v3(p.y, -p.x, p.z);
            }
            p
        })
    })
}

/// `net` mirrored in the plane y = 0.
fn mirrored(net: &Net) -> Net {
    net.map(|row| row.map(|p| v3(p.x, -p.y, p.z)))
}

/// The ring of four patches from the profile `(r, z)`, the first from +x to −y.
fn ring(profile: [(f64, f64); 4]) -> [Net; 4] {
    let first = profile.map(|(r, z)| {
        [
            v3(r, 0., z),
            v3(r, -0.56 * r, z),
            v3(0.56 * r, -r, z),
            v3(0., -r, z),
        ]
    });
    [0, 1, 2, 3].map(|k| turned(&first, k))
}

/// The row `i` of `net` as the curve along `v` at `u = i / 3`, for `i` 0 or 3.
fn row(net: &Net, i: usize) -> Result<Curve, String> {
    let c = patch(net)?.iso_curve(true, i as f64 / 3.).map_err(e)?;
    Ok(Curve::Nurbs(Box::new(c)))
}

/// The flat face inside the rows `i` of `nets`, each run along `v`, joined head to tail.
fn lid(nets: &[Net], i: usize, back: &[bool], name: &str) -> Result<Piece, String> {
    let spans = nets
        .iter()
        .zip(back)
        .map(|(n, &b)| {
            Ok(if b {
                Span::new(row(n, i)?, 1., 0.)
            } else {
                Span::new(row(n, i)?, 0., 1.)
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Piece::planar(vec![spans], name).map_err(e)
}

/// The solid sewn from the patches on `nets` and the flat faces `flats`.
fn sewn(nets: &[Net], flats: Vec<Piece>, feature: &str) -> Result<Brep, String> {
    let mut pieces = flats;
    for (k, n) in nets.iter().enumerate() {
        pieces.push(Piece::whole(patch(n)?, &format!("patch_{k}")).map_err(e)?);
    }
    sew(&pieces, 0., feature).map_err(|err| format!("{feature}: {}", e(err)))
}

/// The net of the four rows `r`, `s`, `t`, `w`.
fn rows(r: [[f64; 3]; 4], s: [[f64; 3]; 4], t: [[f64; 3]; 4], w: [[f64; 3]; 4]) -> Net {
    [r, s, t, w].map(|row| row.map(|[x, y, z]| v3(x, y, z)))
}

pub fn build() -> Result<Brep, String> {
    let rim = ring([(1.4, 2.4), (1.3375, 2.53125), (1.4375, 2.53125), (1.5, 2.4)]);
    let upper = ring([(1.5, 2.4), (1.75, 1.875), (2., 1.35), (2., 0.9)]);
    let lower = ring([(2., 0.9), (2., 0.45), (1.5, 0.225), (1.5, 0.15)]);
    let bottom = ring([(1.5, 0.15), (1.5, 0.075), (1.425, 0.), (0., 0.)]);
    let mut knob = ring([(0., 3.15), (0.8, 3.15), (0., 2.85), (0.2, 2.7)]);
    let knob_row_1 = [
        v3(0.8, 0., 3.15),
        v3(0.8, -0.45, 3.15),
        v3(0.45, -0.8, 3.15),
        v3(0., -0.8, 3.15),
    ];
    for (k, n) in knob.iter_mut().enumerate() {
        n[1] = turned(&[knob_row_1; 4], k)[0];
    }
    let cover = ring([(0.2, 2.7), (0.4, 2.55), (1.3, 2.55), (1.3, 2.4)]);

    let body_nets = [rim, upper, lower, bottom].concat();
    let body = sewn(
        &body_nets,
        vec![lid(&rim, 0, &[false; 4], "rim_disk")?],
        "body",
    )?;
    let lid_nets = [knob, cover].concat();
    let top = sewn(
        &lid_nets,
        vec![lid(&cover, 3, &[false; 4], "lid_disk")?],
        "lid",
    )?;

    let spout_lower = rows(
        [
            [1.7, 0., 1.425],
            [1.7, -0.66, 1.425],
            [1.7, -0.66, 0.6],
            [1.7, 0., 0.6],
        ],
        [
            [2.6, 0., 1.425],
            [2.6, -0.66, 1.425],
            [3.1, -0.66, 0.825],
            [3.1, 0., 0.825],
        ],
        [
            [2.3, 0., 2.1],
            [2.3, -0.25, 2.1],
            [2.4, -0.25, 2.025],
            [2.4, 0., 2.025],
        ],
        [
            [2.7, 0., 2.4],
            [2.7, -0.25, 2.4],
            [3.3, -0.25, 2.4],
            [3.3, 0., 2.4],
        ],
    );
    let spout_lip = rows(
        [
            [2.7, 0., 2.4],
            [2.7, -0.25, 2.4],
            [3.3, -0.25, 2.4],
            [3.3, 0., 2.4],
        ],
        [
            [2.8, 0., 2.475],
            [2.8, -0.25, 2.475],
            [3.525, -0.25, 2.49375],
            [3.525, 0., 2.49375],
        ],
        [
            [2.9, 0., 2.475],
            [2.9, -0.15, 2.475],
            [3.45, -0.15, 2.5125],
            [3.45, 0., 2.5125],
        ],
        [
            [2.8, 0., 2.4],
            [2.8, -0.15, 2.4],
            [3.2, -0.15, 2.4],
            [3.2, 0., 2.4],
        ],
    );
    let handle_upper = rows(
        [
            [-1.6, 0., 2.025],
            [-1.6, -0.3, 2.025],
            [-1.5, -0.3, 2.25],
            [-1.5, 0., 2.25],
        ],
        [
            [-2.3, 0., 2.025],
            [-2.3, -0.3, 2.025],
            [-2.5, -0.3, 2.25],
            [-2.5, 0., 2.25],
        ],
        [
            [-2.7, 0., 2.025],
            [-2.7, -0.3, 2.025],
            [-3., -0.3, 2.25],
            [-3., 0., 2.25],
        ],
        [
            [-2.7, 0., 1.8],
            [-2.7, -0.3, 1.8],
            [-3., -0.3, 1.8],
            [-3., 0., 1.8],
        ],
    );
    let handle_lower = rows(
        [
            [-2.7, 0., 1.8],
            [-2.7, -0.3, 1.8],
            [-3., -0.3, 1.8],
            [-3., 0., 1.8],
        ],
        [
            [-2.7, 0., 1.575],
            [-2.7, -0.3, 1.575],
            [-3., -0.3, 1.35],
            [-3., 0., 1.35],
        ],
        [
            [-2.5, 0., 1.125],
            [-2.5, -0.3, 1.125],
            [-2.65, -0.3, 0.9375],
            [-2.65, 0., 0.9375],
        ],
        [
            [-2., 0., 0.9],
            [-2., -0.3, 0.9],
            [-1.9, -0.3, 0.6],
            [-1.9, 0., 0.6],
        ],
    );
    // Each end's loop is a patch's row and the same row of its mirror image run back.
    let both = [false, true];
    let spout_nets = [
        spout_lower,
        mirrored(&spout_lower),
        spout_lip,
        mirrored(&spout_lip),
    ];
    let spout = sewn(
        &spout_nets,
        vec![
            lid(&spout_nets[..2], 0, &both, "base")?,
            lid(&spout_nets[2..], 3, &both, "mouth")?,
        ],
        "spout",
    )?;
    let handle_nets = [
        handle_upper,
        mirrored(&handle_upper),
        handle_lower,
        mirrored(&handle_lower),
    ];
    let handle = sewn(
        &handle_nets,
        vec![
            lid(&handle_nets[..2], 0, &both, "upper_end")?,
            lid(&handle_nets[2..], 3, &both, "lower_end")?,
        ],
        "handle",
    )?;

    let pot = union(&body, &top).map_err(|err| format!("body and lid: {}", e(err)))?;
    let pot = union(&pot, &spout).map_err(|err| format!("with the spout: {}", e(err)))?;
    union(&pot, &handle).map_err(|err| format!("with the handle: {}", e(err)))
}
