use std::time::Duration;

use ggez::graphics::{Color, DrawParam, Drawable, Rect, Text};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
use glam::Vec2;

use crate::ai::{
    alpha_beta, center_heuristic, corner_heuristic, everywhere_heuristic, generate_children, noise,
    winning_sequence_heuristic, AILevel,
};
use crate::{assets::Assets, coord_from_ids};
use crate::{constants::*, GameMode, GameState};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn other(&self) -> Player {
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
pub enum CellState {
    Occupied(Player),
    Free,
    Tie,
}

impl std::fmt::Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Occupied(player) => player.to_string(),
                Self::Free => String::from("*"),
                _ => String::from(""),
            }
        )
    }
}

#[derive(Clone)]
pub struct Board {
    pub cells: [[CellState; 9]; 9],
    pub states: [CellState; 9],
}

impl Board {
    fn new() -> Self {
        Self {
            cells: [[CellState::Free; 9]; 9],
            states: [CellState::Free; 9],
        }
    }
}

pub fn all_occupied(states: &[CellState; 9]) -> bool {
    states
        .iter()
        .all(|cell_state| !matches!(cell_state, CellState::Free))
}

pub fn is_won_by(states: &[CellState; 9], player: Player) -> bool {
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
pub enum PlayingState {
    Tie,
    Win(Player),
    Continue,
}

#[derive(Clone)]
pub struct Morpion {
    pub board: Board,
    pub player: Player,
    pub state: PlayingState,
    pub focused_big_cell: Option<usize>,
}

impl std::fmt::Display for Morpion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = &self.board;
        write!(
            f,
            "
{}{}{} | {}{}{} | {}{}{}\n\
{}{}{} | {}{}{} | {}{}{}\n\
{}{}{} | {}{}{} | {}{}{}\n\
---------------------------
{}{}{} | {}{}{} | {}{}{}\n\
{}{}{} | {}{}{} | {}{}{}\n\
{}{}{} | {}{}{} | {}{}{}\n\
---------------------------
{}{}{} | {}{}{} | {}{}{}\n\
{}{}{} | {}{}{} | {}{}{}\n\
{}{}{} | {}{}{} | {}{}{}
            ",
            board.cells[0][0],
            board.cells[0][1],
            board.cells[0][2],
            board.cells[1][0],
            board.cells[1][1],
            board.cells[1][2],
            board.cells[2][0],
            board.cells[2][1],
            board.cells[2][2],
            board.cells[0][3],
            board.cells[0][4],
            board.cells[0][5],
            board.cells[1][3],
            board.cells[1][4],
            board.cells[1][5],
            board.cells[2][3],
            board.cells[2][4],
            board.cells[2][5],
            board.cells[0][6],
            board.cells[0][7],
            board.cells[0][8],
            board.cells[1][6],
            board.cells[1][7],
            board.cells[1][8],
            board.cells[2][6],
            board.cells[2][7],
            board.cells[2][8],
            board.cells[3][0],
            board.cells[3][1],
            board.cells[3][2],
            board.cells[4][0],
            board.cells[4][1],
            board.cells[4][2],
            board.cells[5][0],
            board.cells[5][1],
            board.cells[5][2],
            board.cells[3][3],
            board.cells[3][4],
            board.cells[3][5],
            board.cells[4][3],
            board.cells[4][4],
            board.cells[4][5],
            board.cells[5][3],
            board.cells[5][4],
            board.cells[5][5],
            board.cells[3][6],
            board.cells[3][7],
            board.cells[3][8],
            board.cells[4][6],
            board.cells[4][7],
            board.cells[4][8],
            board.cells[5][6],
            board.cells[5][7],
            board.cells[5][8],
            board.cells[6][0],
            board.cells[6][1],
            board.cells[6][2],
            board.cells[7][0],
            board.cells[7][1],
            board.cells[7][2],
            board.cells[8][0],
            board.cells[8][1],
            board.cells[8][2],
            board.cells[6][3],
            board.cells[6][4],
            board.cells[6][5],
            board.cells[7][3],
            board.cells[7][4],
            board.cells[7][5],
            board.cells[8][3],
            board.cells[8][4],
            board.cells[8][5],
            board.cells[6][6],
            board.cells[6][7],
            board.cells[6][8],
            board.cells[7][6],
            board.cells[7][7],
            board.cells[7][8],
            board.cells[8][6],
            board.cells[8][7],
            board.cells[8][8],
        )
    }
}

impl Morpion {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            player: Player::X,
            state: PlayingState::Continue,
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
        self.state = self.check_playing_state();
    }

    pub fn check_playing_state(&self) -> PlayingState {
        if is_won_by(&self.board.states, self.player.other()) {
            PlayingState::Win(self.player.other())
        } else if all_occupied(&self.board.states)
            || self
                .focused_big_cell
                .is_some_and(|ult_index| all_occupied(&self.board.cells[ult_index]))
            || self
                .board
                .states
                .iter()
                .zip(self.board.cells.iter())
                .filter(|(big_cell_state, _)| **big_cell_state == CellState::Free)
                .all(|(_, lil_cell_state)| all_occupied(lil_cell_state))
        {
            PlayingState::Tie
        } else {
            PlayingState::Continue
        }
    }

    pub fn reset(&mut self) {
        self.board = Board::new();
        self.player = Player::X;
        self.state = PlayingState::Continue;
        self.focused_big_cell = None;
    }
}

pub struct MorpionScene {
    pub morpion: Morpion,
    assets: Assets,
    text: Text,
    pub clicked: Option<(usize, usize)>,
    turn: usize,
}

impl MorpionScene {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            morpion: Morpion::new(),
            assets: Assets::new(ctx)?,
            text: Text::new("X begins !"),
            clicked: None,
            turn: 1,
        })
    }

    pub fn reset(&mut self) {
        self.morpion.reset();
        self.turn = 1;
        self.text = Text::new("X begins !");
    }

    fn player_plays(&mut self) {
        // If cell clicked
        if let Some((ult_index, index)) = self.clicked {
            if self.morpion.index_is_playable(ult_index, index) {
                self.morpion.play_at(ult_index, index);
                self.turn += 1;
            }
        }
    }

    fn ai_plays(&mut self, ai_level: &AILevel) {
        //ggez::timer::sleep(Duration::from_millis(500));
        let children = generate_children(&self.morpion);
        let depth = 6;
        if !children.is_empty() {
            let mut best_move_index = 0;
            let mut max_score = isize::MIN;
            for (index, child) in children.iter().enumerate() {
                let mut score = alpha_beta(
                    child,
                    depth,
                    isize::MIN,
                    isize::MAX,
                    self.morpion.player,
                    match ai_level {
                        AILevel::Easy => &corner_heuristic,
                        AILevel::Medium => &center_heuristic,
                        AILevel::Hard => &everywhere_heuristic,
                    },
                );
                score += score * 10 + noise(2);

                //println!("Child {} (score: {}) \n{}", index, score, child);

                if score > max_score {
                    max_score = score;
                    best_move_index = index;
                }
            }
            self.morpion = children[best_move_index].clone();
            self.turn += 1;
            //println!("Best: {} \nDepth: {}", self.morpion, depth);
        }
    }

    pub fn update(&mut self, ctx: &mut Context, state: &mut GameState, game_mode: &GameMode) {
        while ctx.time.check_update_time(DESIRED_FPS) {
            match self.morpion.state {
                PlayingState::Continue => {
                    match game_mode {
                        GameMode::PvAI(o) => match self.morpion.player {
                            Player::X => self.player_plays(),
                            Player::O => self.ai_plays(o),
                        },
                        GameMode::PvP => match self.morpion.player {
                            Player::X => self.player_plays(),
                            Player::O => self.player_plays(),
                        },
                        GameMode::AIvAI(x, o) => match self.morpion.player {
                            Player::X => self.ai_plays(x),
                            Player::O => self.ai_plays(o),
                        },
                    };

                    self.text = Text::new(format!("{}'s turn !", self.morpion.player));

                    self.morpion.state = self.morpion.check_playing_state();

                    if ctx.keyboard.is_key_pressed(KeyCode::Q) {
                        *state = GameState::StartMenu;
                        self.reset();
                    }
                }
                PlayingState::Tie => {
                    self.text = Text::new("Tie !\nPress R to restart or Q to go to the menu");
                    if ctx.keyboard.is_key_pressed(KeyCode::R) {
                        self.reset();
                    }
                    if ctx.keyboard.is_key_pressed(KeyCode::Q) {
                        self.reset();
                        *state = GameState::StartMenu;
                    }
                }
                PlayingState::Win(player) => {
                    self.text = Text::new(format!(
                        "{} has won\nPress R to restart or Q to go to the menu",
                        player
                    ));
                    if ctx.keyboard.is_key_pressed(KeyCode::R) {
                        self.reset();
                    }
                    if ctx.keyboard.is_key_pressed(KeyCode::Q) {
                        self.reset();
                        *state = GameState::StartMenu;
                    }
                }
            }
        }
    }
}

impl Drawable for MorpionScene {
    fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        _param: impl Into<ggez::graphics::DrawParam>,
    ) {
        // Grid
        canvas.draw(&self.assets.big_grid, DrawParam::default());
        // Grids
        for i in 0..9 {
            let dst = Vec2::new(
                BORDER_PADDING + CELL_PADDING + ((i as u32 % 3) as f32) * BIG_CELL_SIZE,
                BORDER_PADDING + CELL_PADDING + (((i - i % 3) / 3) as f32) * BIG_CELL_SIZE,
            );
            let mesh = match self.morpion.focused_big_cell {
                Some(index) if index == i => &self.assets.focused_grid,
                None if self.morpion.board.states[i] == CellState::Free => {
                    &self.assets.focused_grid
                }
                _ => &self.assets.lil_grid,
            };
            canvas.draw(mesh, DrawParam::new().dest(dst));
        }
        // Crosses and Circles
        for (ult_index, ult_cell) in self.morpion.board.cells.iter().enumerate() {
            for (index, cell) in ult_cell.iter().enumerate() {
                let (x, y) = coord_from_ids(ult_index, index);
                match cell {
                    CellState::Free | CellState::Tie => {}
                    CellState::Occupied(Player::X) => {
                        canvas.draw(
                            &self.assets.cross245,
                            DrawParam::new().dest_rect(Rect::new(
                                x,
                                y,
                                CROSS_CIRCLE_SCALE_FACTOR,
                                CROSS_CIRCLE_SCALE_FACTOR,
                            )),
                        );
                    }
                    CellState::Occupied(Player::O) => {
                        canvas.draw(
                            &self.assets.circle245,
                            DrawParam::new().dest_rect(Rect::new(
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
                        &self.assets.cross245,
                        DrawParam::new().dest(Vec2::new(x - CELL_PADDING, y - CELL_PADDING)),
                    );
                }
                CellState::Occupied(Player::O) => {
                    canvas.draw(
                        &self.assets.circle245,
                        DrawParam::new().dest(Vec2::new(x - CELL_PADDING, y - CELL_PADDING)),
                    );
                }
            }
        }
        // Text
        canvas.draw(
            &self.text,
            DrawParam::from([BORDER_PADDING, SCREEN_SIZE.1 - BORDER_PADDING]).color(Color::WHITE),
        );
    }

    fn dimensions(
        &self,
        _gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>,
    ) -> Option<ggez::graphics::Rect> {
        None
    }
}
