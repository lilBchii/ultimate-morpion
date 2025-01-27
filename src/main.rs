use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, Rect, Text};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
use glam::Vec2;

use std::{env, path};

mod assets;
mod constants;

use assets::Assets;
use constants::{
    BIG_CELL_SIZE, BORDER_PADDING, CELL_PADDING, CELL_SIZE, CROSS_CIRCLE_SCALE_FACTOR, DESIRED_FPS,
    SCREEN_SIZE,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Player {
    X,
    O,
}

impl Player {
    fn other(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => "X",
                Self::O => "O",
            }
        )
    }
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
    fn new() -> Cell {
        Cell {
            board: [CellState::Free; 9],
            state: CellState::Free,
        }
    }
    fn all_occupied(&self) -> bool {
        self.board.iter().all(|cell| *cell != CellState::Free)
    }
    fn is_won_by(&self, last_player: Player) -> bool {
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

struct Morpion {
    board: [Cell; 9],
    state: GameState,
    player: Player,
    focused_big_cell: Option<usize>,
    meshes: Assets,
    text: Text,
    clicked: Option<(usize, usize)>,
}

impl Morpion {
    fn new(ctx: &mut Context) -> GameResult<Morpion> {
        Ok(Morpion {
            board: [Cell::new(); 9],
            state: GameState::Continue,
            player: Player::X,
            focused_big_cell: None,
            meshes: Assets::new(ctx)?,
            text: Text::new("X begins !"),
            clicked: None,
        })
    }

    fn play_at(&mut self, ult_index: usize, index: usize) {
        // Cell becomes occupied by player
        self.board[ult_index].board[index] = CellState::Occupied(self.player);
        let big_cell = self.board[ult_index];
        // If big cell is won by player big cell is now occupied
        if big_cell.is_won_by(self.player) {
            self.board[ult_index].state = CellState::Occupied(self.player);
        } else if big_cell.all_occupied() {
            // Else if all cells of big cell are occupied then big cell is tie
            self.board[ult_index].state = CellState::Tie;
        }
        // Check if index is free to determine next focused big cell
        match self.board[index].state {
            CellState::Free => self.focused_big_cell = Some(index),
            _ => self.focused_big_cell = None,
        }
        // Change player
        self.player = self.player.other();
    }

    fn player_plays(&mut self) {
        // If cell clicked
        if let Some((ult_index, index)) = self.clicked {
            let big_cell = self.board[ult_index];
            let cell = big_cell.board[index];
            // If big cell is free and cell is free
            if big_cell.state == CellState::Free && cell == CellState::Free {
                // Get where to play
                match self.focused_big_cell {
                    // There is no focused big cell
                    None => {
                        self.play_at(ult_index, index);
                    }
                    // There is a focused big cell
                    Some(obliged_index) => {
                        // If player clicked on right big cell
                        if ult_index == obliged_index {
                            self.play_at(ult_index, index);
                        }
                    }
                }
                self.text = Text::new(format!("{}'s turn !", self.player));
            }
        }
    }

    fn ia_plays(&mut self) {
        todo!()
    }

    fn all_occupied(&self) -> bool {
        self.board.iter().all(|cell| cell.state != CellState::Free)
    }
    fn is_won(&self) -> bool {
        let player = CellState::Occupied(self.player.other());

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
    fn reset(&mut self) {
        self.board = [Cell::new(); 9];
        self.state = GameState::Continue;
        self.player = Player::X;
        self.focused_big_cell = None;
        self.text = Text::new("X begins !");
    }
}

impl EventHandler for Morpion {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(30, 30, 38));
        // Grid
        canvas.draw(&self.meshes.big_grid, graphics::DrawParam::default());
        // Grids
        for i in 0..9 {
            let dst = Vec2::new(
                BORDER_PADDING + CELL_PADDING + ((i as u32 % 3) as f32) * BIG_CELL_SIZE,
                BORDER_PADDING + CELL_PADDING + (((i - i % 3) / 3) as f32) * BIG_CELL_SIZE,
            );
            let mesh = match self.focused_big_cell {
                Some(index) if index == i => &self.meshes.focused_grid,
                None if self.board[i].state == CellState::Free => &self.meshes.focused_grid,
                _ => &self.meshes.lil_grid,
            };
            canvas.draw(mesh, graphics::DrawParam::new().dest(dst));
        }
        // Crosses and Circles
        for (ult_index, ult_cell) in self.board.iter().enumerate() {
            for (index, cell) in ult_cell.board.iter().enumerate() {
                let (x, y) = coord_from_ids(ult_index, index);
                match cell {
                    CellState::Free | CellState::Tie => {}
                    CellState::Occupied(Player::X) => {
                        canvas.draw(
                            &self.meshes.cross245,
                            graphics::DrawParam::new().dest_rect(Rect::new(
                                x,
                                y,
                                CROSS_CIRCLE_SCALE_FACTOR,
                                CROSS_CIRCLE_SCALE_FACTOR,
                            )),
                        );
                    }
                    CellState::Occupied(Player::O) => {
                        canvas.draw(
                            &self.meshes.circle245,
                            graphics::DrawParam::new().dest_rect(Rect::new(
                                x,
                                y,
                                CROSS_CIRCLE_SCALE_FACTOR,
                                CROSS_CIRCLE_SCALE_FACTOR,
                            )),
                        );
                    }
                }
            }
            let (x, y) = coord_from_ids(ult_index, 0);
            match ult_cell.state {
                CellState::Free | CellState::Tie => {}
                CellState::Occupied(Player::X) => {
                    canvas.draw(
                        &self.meshes.cross245,
                        graphics::DrawParam::new()
                            .dest(Vec2::new(x - CELL_PADDING, y - CELL_PADDING)),
                    );
                }
                CellState::Occupied(Player::O) => {
                    canvas.draw(
                        &self.meshes.circle245,
                        graphics::DrawParam::new()
                            .dest(Vec2::new(x - CELL_PADDING, y - CELL_PADDING)),
                    );
                }
            }
        }
        // Text
        canvas.draw(
            &self.text,
            graphics::DrawParam::from([BORDER_PADDING, SCREEN_SIZE.1 - BORDER_PADDING])
                .color(Color::WHITE),
        );
        canvas.finish(ctx)
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {
            match self.state {
                GameState::Continue => {
                    self.player_plays();
                    if self.is_won() {
                        self.state = GameState::Win(self.player.other());
                    } else if self.all_occupied() {
                        self.state = GameState::Tie;
                    }
                }
                GameState::Tie => {
                    self.text = Text::new("Tie !\nPress R to restart");
                    if ctx.keyboard.is_key_pressed(KeyCode::R) {
                        self.reset();
                    }
                }
                GameState::Win(player) => {
                    self.text = Text::new(format!("{} has won\nPress R to restart", player));
                    if ctx.keyboard.is_key_pressed(KeyCode::R) {
                        self.reset();
                    }
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
        if let Some((ult_index, index)) = ids_from_coord(x, y) {
            self.clicked = Some((ult_index, index));
        }
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        self.clicked = None;
        Ok(())
    }
}

fn ids_from_coord(x: f32, y: f32) -> Option<(usize, usize)> {
    if (x > BORDER_PADDING && x < BORDER_PADDING + 3.0 * BIG_CELL_SIZE)
        && (y > BORDER_PADDING && y < BORDER_PADDING + 3.0 * BIG_CELL_SIZE)
    {
        let ult_col = ((x - BORDER_PADDING) / BIG_CELL_SIZE) as usize + 1;
        let ult_line = ((y - BORDER_PADDING) / BIG_CELL_SIZE) as usize + 1;
        let ultimate_coord = 3 * ult_line - (3 - ult_col) - 1;
        let col = ((x - BORDER_PADDING - CELL_PADDING - ((ult_col - 1) as f32 * BIG_CELL_SIZE))
            / CELL_SIZE) as usize
            + 1;
        let line = ((y - BORDER_PADDING - CELL_PADDING - ((ult_line - 1) as f32 * BIG_CELL_SIZE))
            / CELL_SIZE) as usize
            + 1;
        let coord = 3 * line - (3 - col) - 1;
        Some((ultimate_coord, coord))
    } else {
        None
    }
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

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("ressources");
        path
    } else {
        path::PathBuf::from("./ressources")
    };

    let (mut ctx, events_loop) = ggez::ContextBuilder::new("ultimate-morpion", "lilBchii")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("ultimate-morpion"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = Morpion::new(&mut ctx).unwrap();
    event::run(ctx, events_loop, state)
}
