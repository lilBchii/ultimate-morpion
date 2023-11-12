use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::Vec2;

use std::{env, path};

const BG_COLOR: (u8, u8, u8) = (30, 30, 38);

const CELL_PADDING: f32 = 10.0;
const BORDER_PADDING: f32 = 50.0;
const CELL_SIZE: f32 = 75.0;
const BIG_CELL_SIZE: f32 = 3.0 * CELL_SIZE + 2.0 * CELL_PADDING;

const SCREEN_SIZE: (f32, f32) = (
    (CELL_SIZE * 9.0) + (6.0 * CELL_PADDING) + (2.0 * BORDER_PADDING),
    (CELL_SIZE * 9.0) + (6.0 * CELL_PADDING) + (2.0 * BORDER_PADDING),
);

const DESIRED_FPS: u32 = 15;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Player {
    X,
    O,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellState {
    Occupied(Player),
    Free,
    Tie,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cell {
    board: [CellState; 9],
    state: CellState,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            board: [CellState::Free; 9],
            state: CellState::Free,
        }
    }
    pub fn all_occupied(&self) -> bool {
        let mut b = true;
        for cell in self.board {
            if cell == CellState::Free {
                b = false;
            }
        }
        b
    }
    pub fn is_won_by(&self, last_player: Player) -> bool {
        let player = CellState::Occupied(last_player);

        (self.board[0] == player && self.board[1] == player && self.board[2] == player)
            || (self.board[3] == player && self.board[4] == player && self.board[5] == player)
            || (self.board[6] == player && self.board[7] == player && self.board[8] == player)
            || (self.board[0] == player && self.board[3] == player && self.board[6] == player)
            || (self.board[1] == player && self.board[4] == player && self.board[7] == player)
            || (self.board[2] == player && self.board[5] == player && self.board[8] == player)
            || (self.board[0] == player && self.board[4] == player && self.board[8] == player)
            || (self.board[2] == player && self.board[4] == player && self.board[6] == player)
    }
}

#[derive(Debug)]
enum GameState {
    Tie,
    Win(Player),
    Continue,
}

struct Assets {
    grid: graphics::Mesh,
    grids: Vec<graphics::Mesh>,
    cross: graphics::Image,
    circle: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let mut vgrids: Vec<graphics::Mesh> = Vec::new();
        for index in 0..=8 {
            vgrids.push(make_grid_lines(
                ctx,
                4.5,
                Color::from_rgb(55, 60, 75),
                (
                    BORDER_PADDING + CELL_PADDING + ((index as u32 % 3) as f32) * BIG_CELL_SIZE,
                    BORDER_PADDING
                        + CELL_PADDING
                        + (((index - index % 3) / 3) as f32) * BIG_CELL_SIZE,
                ),
                /*coord_from_id(index as usize, BIG_CELL_SIZE, BORDER_PADDING + CELL_PADDING),*/
                CELL_SIZE,
            )?);
        }
        Ok(Assets {
            grid: make_grid_lines(
                ctx,
                6.5,
                Color::from_rgb(55, 60, 75),
                (BORDER_PADDING, BORDER_PADDING),
                BIG_CELL_SIZE,
            )?,
            grids: vgrids,
            cross: graphics::Image::from_path(ctx, "/cross.png")?,
            circle: graphics::Image::from_path(ctx, "/circle.png")?,
        })
    }
}

struct Morpion {
    board: [Cell; 9],
    state: GameState,
    last_play: (Player, Option<usize>),
    meshes: Assets,
    clicked: (bool, Option<(usize, usize)>),
}

impl Morpion {
    pub fn new(ctx: &mut Context) -> GameResult<Morpion> {
        Ok(Morpion {
            board: [Cell::new(); 9],
            state: GameState::Continue,
            last_play: (Player::X, None),
            meshes: Assets::new(ctx)?,
            clicked: (false, None),
        })
    }

    // Determine who's playing, change cellstate where it's playing, determine where to play next
    pub fn play(&mut self, ult_index: usize, index: usize) {
        match self.last_play.0 {
            Player::X => {
                self.board[ult_index].board[index] = CellState::Occupied(Player::O);
                // Check if next turn we play everywhere or not
                if self.board[index].state == CellState::Tie
                    || self.board[index].state == CellState::Occupied(Player::X)
                    || self.board[index].state == CellState::Occupied(Player::O)
                {
                    self.last_play = (Player::O, None);
                } else {
                    self.last_play = (Player::O, Some(index));
                }
            }
            Player::O => {
                self.board[ult_index].board[index] = CellState::Occupied(Player::X);
                if self.board[index].state == CellState::Tie
                    || self.board[index].state == CellState::Occupied(Player::X)
                    || self.board[index].state == CellState::Occupied(Player::O)
                {
                    self.last_play = (Player::X, None);
                } else {
                    self.last_play = (Player::X, Some(index));
                }
            }
        }
    }
    pub fn all_occupied(&self) -> bool {
        let mut b = true;
        for cell in self.board {
            if cell.state == CellState::Free {
                b = false;
            }
        }
        b
    }
    pub fn is_won(&self) -> bool {
        let player = CellState::Occupied(self.last_play.0);

        (self.board[0].state == player
            && self.board[1].state == player
            && self.board[2].state == player)
            || (self.board[3].state == player
                && self.board[4].state == player
                && self.board[5].state == player)
            || (self.board[6].state == player
                && self.board[7].state == player
                && self.board[8].state == player)
            || (self.board[0].state == player
                && self.board[3].state == player
                && self.board[6].state == player)
            || (self.board[1].state == player
                && self.board[4].state == player
                && self.board[7].state == player)
            || (self.board[2].state == player
                && self.board[5].state == player
                && self.board[8].state == player)
            || (self.board[0].state == player
                && self.board[4].state == player
                && self.board[8].state == player)
            || (self.board[2].state == player
                && self.board[4].state == player
                && self.board[6].state == player)
    }
}

impl EventHandler for Morpion {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, Color::from_rgb(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2));
        // Grid
        canvas.draw(&self.meshes.grid, graphics::DrawParam::default());
        // Grids
        for g in self.meshes.grids.iter() {
            canvas.draw(g, graphics::DrawParam::default());
        }
        // Crosses and Circles
        for (ult_index, ult_cell) in self.board.iter().enumerate() {
            for (index, cell) in ult_cell.board.iter().enumerate() {
                let (x, y) = coord_from_ids(ult_index, index);
                match cell {
                    CellState::Free | CellState::Tie => {}
                    CellState::Occupied(Player::X) => {
                        canvas.draw(
                            &self.meshes.cross,
                            graphics::DrawParam::new().dest(Vec2::new(x, y)),
                        );
                    }
                    CellState::Occupied(Player::O) => {
                        canvas.draw(
                            &self.meshes.circle,
                            graphics::DrawParam::new().dest(Vec2::new(x, y)),
                        );
                    }
                }
            }
        }
        canvas.finish(ctx)
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {
            match self.state {
                GameState::Continue => {
                    if self.clicked.0 {
                        let (ult_index, index) = self.clicked.1.unwrap();
                        if self.board[ult_index].state == CellState::Free {
                            match self.last_play.1 {
                                None => {
                                    self.play(ult_index, index);
                                }
                                Some(obliged_index) => {
                                    if ult_index == obliged_index {
                                        self.play(ult_index, index);
                                    }
                                }
                            }
                        }
                        if self.board[ult_index].is_won_by(self.last_play.0) {
                            self.board[ult_index].state = CellState::Occupied(self.last_play.0);
                        } else if self.board[ult_index].all_occupied() {
                            self.board[ult_index].state = CellState::Tie;
                        }
                    }
                    if self.is_won() {
                        self.state = GameState::Win(self.last_play.0);
                    } else if self.all_occupied() {
                        self.state = GameState::Tie;
                    }
                }
                GameState::Tie => {
                    println!("Tie");
                }
                _ => {
                    println!("{:?}", self.state);
                }
            }
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.clicked = (true, Some(ids_from_coord(x, y)));
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        self.clicked = (false, None);
        Ok(())
    }
}

fn ids_from_coord(x: f32, y: f32) -> (usize, usize) {
    let ult_raw = ((x - BORDER_PADDING) / BIG_CELL_SIZE) as usize + 1;
    let ult_line = ((y - BORDER_PADDING) / BIG_CELL_SIZE) as usize + 1;
    let ultimate_coord = 3 * ult_line - (3 - ult_raw) - 1;
    let raw = ((x - BORDER_PADDING - CELL_PADDING - ((ult_raw - 1) as f32 * BIG_CELL_SIZE))
        / CELL_SIZE) as usize
        + 1;
    let line = ((y - BORDER_PADDING - CELL_PADDING - ((ult_line - 1) as f32 * BIG_CELL_SIZE))
        / CELL_SIZE) as usize
        + 1;
    let coord = 3 * line - (3 - raw) - 1;
    //let coord = 1;
    (ultimate_coord, coord)
}

fn coord_from_ids(ult_index: usize, index: usize) -> (f32, f32) {
    (
        BORDER_PADDING
            + (ult_index % 3) as f32 * BIG_CELL_SIZE
            + CELL_PADDING
            + (index % 3) as f32 * CELL_SIZE,
        BORDER_PADDING
            + ((ult_index - (ult_index % 3)) / 3) as f32 * BIG_CELL_SIZE
            + CELL_PADDING
            + ((index - (index % 3)) / 3) as f32 * CELL_SIZE,
    )
}

// New mesh for the 3x3 grid
fn make_grid_lines(
    ctx: &mut Context,
    width: f32,
    color: Color,
    anchor: (f32, f32),
    cellsize: f32,
) -> GameResult<graphics::Mesh> {
    let l = &mut graphics::MeshBuilder::new();
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
    Ok(graphics::Mesh::from_data(ctx, l.build()))
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, events_loop) = ggez::ContextBuilder::new("ultimate-morpion", "lilBchii")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("ultimate-morpion"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = Morpion::new(&mut ctx).unwrap();
    event::run(ctx, events_loop, state)
}
