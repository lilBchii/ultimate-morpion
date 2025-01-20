use ggez::{
    graphics::{Color, Image, Mesh, MeshBuilder},
    Context, GameResult,
};
use glam::Vec2;

use crate::constants::{BIG_CELL_SIZE, BORDER_PADDING, CELL_SIZE};

pub struct Assets {
    pub big_grid: Mesh,
    pub focused_grid: Mesh,
    pub lil_grid: Mesh,
    pub cross245: Image,
    pub circle245: Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        Ok(Assets {
            big_grid: make_grid_lines(
                ctx,
                6.5,
                Color::from_rgb(55, 60, 75),
                (BORDER_PADDING, BORDER_PADDING),
                BIG_CELL_SIZE,
            )?,
            focused_grid: make_grid_lines(
                ctx,
                4.5,
                Color::from_rgb(90, 100, 125),
                (0.0, 0.0),
                CELL_SIZE,
            )?,
            lil_grid: make_grid_lines(
                ctx,
                4.5,
                Color::from_rgb(55, 60, 75),
                (0.0, 0.0),
                CELL_SIZE,
            )?,
            cross245: Image::from_path(ctx, "/cross_245x245.png")?,
            circle245: Image::from_path(ctx, "/circle_245x245.png")?,
        })
    }
}

// New mesh for the 3x3 grid
fn make_grid_lines(
    ctx: &mut Context,
    width: f32,
    color: Color,
    anchor: (f32, f32),
    cellsize: f32,
) -> GameResult<Mesh> {
    let l = &mut MeshBuilder::new();
    l.line(
        &[
            Vec2::new(anchor.0 + cellsize, anchor.1),
            Vec2::new(anchor.0 + cellsize, anchor.1 + cellsize * 3.0),
        ],
        width,
        color,
    )?;
    l.line(
        &[
            Vec2::new(anchor.0 + 2.0 * cellsize, anchor.1),
            Vec2::new(anchor.0 + 2.0 * cellsize, anchor.1 + cellsize * 3.0),
        ],
        width,
        color,
    )?;
    l.line(
        &[
            Vec2::new(anchor.0, anchor.1 + cellsize),
            Vec2::new(anchor.0 + 3.0 * cellsize, anchor.1 + cellsize),
        ],
        width,
        color,
    )?;
    l.line(
        &[
            Vec2::new(anchor.0, anchor.1 + 2.0 * cellsize),
            Vec2::new(anchor.0 + 3.0 * cellsize, anchor.1 + 2.0 * cellsize),
        ],
        width,
        color,
    )?;
    Ok(Mesh::from_data(ctx, l.build()))
}
