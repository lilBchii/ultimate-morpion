use ai::{alpha_beta, generate_children, minimax};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, Rect, Text};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
use glam::Vec2;

use std::time::Duration;
use std::{env, path};

mod ai;
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

#[derive(Clone)]
struct Board {
    cells: [[CellState; 9]; 9],
    states: [CellState; 9],
}

impl Board {
    fn new() -> Self {
        Self {
            cells: [[CellState::Free; 9]; 9],
            states: [CellState::Free; 9],
        }
    }
}

fn all_occupied(states: &[CellState; 9]) -> bool {
    states
        .iter()
        .all(|cell_state| !matches!(cell_state, CellState::Free))
}

fn is_won_by(states: &[CellState; 9], player: Player) -> bool {
    let player = CellState::Occupied(player);
    (states[0] == player && states[1] == player && states[2] == player)
        || (states[3] == player && states[4] == player && states[5] == player)
        || (states[6] == player && states[7] == player && states[8] == player)
        || (states[0] == player && states[3] == player && states[6] == player)
        || (states[1] == player && states[4] == player && states[7] == player)
        || (states[2] == player && states[5] == player && states[8] == player)
        || (states[0] == player && states[4] == player && states[8] == player)
        || (states[2] == player && states[4] == player && states[6] == player)
}

#[derive(Debug, PartialEq, Clone)]
enum GameState {
    Tie,
    Win(Player),
    Continue,
}

#[derive(Clone)]
struct Morpion {
    board: Board,
    state: GameState,
    player: Player,
    focused_big_cell: Option<usize>,
}

impl Morpion {
    fn new() -> Self {
        Self {
            board: Board::new(),
            state: GameState::Continue,
            player: Player::X,
            focused_big_cell: None,
        }
    }

    pub fn index_is_playable(&self, ult_index: usize, index: usize) -> bool {
        self.board.states[ult_index] == CellState::Free
            && self.board.cells[ult_index][index] == CellState::Free
            && (self
                .focused_big_cell
                .is_some_and(|obliged_index| obliged_index == ult_index)
                || self.focused_big_cell.is_none())
    }

    pub fn play_at(&mut self, ult_index: usize, index: usize) {
        // Cell becomes occupied by player
        self.board.cells[ult_index][index] = CellState::Occupied(self.player);
        // If big cell is won by player big cell is now occupied
        if is_won_by(&self.board.cells[ult_index], self.player) {
            self.board.states[ult_index] = CellState::Occupied(self.player);
        } else if all_occupied(&self.board.states) {
            // Else if all cells of big cell are occupied then big cell is tie
            self.board.states[ult_index] = CellState::Tie;
        }
        // Check if index is free to determine next focused big cell
        match self.board.states[index] {
            CellState::Free => self.focused_big_cell = Some(index),
            _ => self.focused_big_cell = None,
        }
        // Change player
        self.player = self.player.other();
    }

    fn reset(&mut self) {
        self.board = Board::new();
        self.state = GameState::Continue;
        self.player = Player::X;
        self.focused_big_cell = None;
    }
}

struct Game {
    morpion: Morpion,
    meshes: Assets,
    text: Text,
    clicked: Option<(usize, usize)>,
}

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            morpion: Morpion::new(),
            meshes: Assets::new(ctx)?,
            text: Text::new("X begins !"),
            clicked: None,
        })
    }

    fn player_plays(&mut self) {
        // If cell clicked
        if let Some((ult_index, index)) = self.clicked {
            if self.morpion.index_is_playable(ult_index, index) {
                self.morpion.play_at(ult_index, index);
            }
        }
    }

    fn ai_plays(&mut self) {
        ggez::timer::sleep(Duration::from_millis(500));

        let children = generate_children(&self.morpion);
        let mut best_move_index = 0;
        let mut max_score = isize::MIN;
        for (index, child) in children.iter().enumerate() {
            //let score = alpha_beta(child, 6, isize::MIN, isize::MAX, true);
            let score = minimax(child, 5, true);
            if score > max_score {
                max_score = score;
                best_move_index = index;
            }
        }

        self.morpion = children[best_move_index].clone();
    }

    fn reset(&mut self) {
        self.morpion.reset();
        self.text = Text::new("X begins !");
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {
            match self.morpion.state {
                GameState::Continue => {
                    match self.morpion.player {
                        Player::X => {
                            self.player_plays();
                        }
                        Player::O => {
                            self.ai_plays();
                        }
                    }

                    self.text = Text::new(format!("{}'s turn !", self.morpion.player));

                    if is_won_by(&self.morpion.board.states, self.morpion.player.other()) {
                        self.morpion.state = GameState::Win(self.morpion.player.other());
                    } else if all_occupied(&self.morpion.board.states) {
                        self.morpion.state = GameState::Tie;
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
            let mesh = match self.morpion.focused_big_cell {
                Some(index) if index == i => &self.meshes.focused_grid,
                None if self.morpion.board.states[i] == CellState::Free => {
                    &self.meshes.focused_grid
                }
                _ => &self.meshes.lil_grid,
            };
            canvas.draw(mesh, graphics::DrawParam::new().dest(dst));
        }
        // Crosses and Circles
        for (ult_index, ult_cell) in self.morpion.board.cells.iter().enumerate() {
            for (index, cell) in ult_cell.iter().enumerate() {
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
            match self.morpion.board.states[ult_index] {
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
        if col > 3 || line > 3 {
            //not in a cell
            return None;
        }
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

    let state = Game::new(&mut ctx)?;
    event::run(ctx, events_loop, state)
}
