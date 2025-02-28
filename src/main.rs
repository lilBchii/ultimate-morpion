use ggegui::egui::{self, Button, Label};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawParam, Drawable};
use ggez::{Context, GameResult};

use std::{env, path};

mod ai;
mod assets;
mod constants;
mod menu;
mod morpion;

use constants::{BIG_CELL_SIZE, BORDER_PADDING, CELL_PADDING, CELL_SIZE, SCREEN_SIZE};
use menu::Menu;
use morpion::{CellState, Morpion, MorpionScene, Player, PlayingState};

#[derive(PartialEq, Eq, Clone)]
enum GameState {
    Playing(GameMode),
    StartMenu,
    SelectAIMenu(bool),
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum GameMode {
    PvP,
    PvAI,
    AIvAI,
}

struct Game {
    morpion_scene: MorpionScene,
    state: GameState,
    menu: Menu,
}

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            morpion_scene: MorpionScene::new(ctx)?,
            state: GameState::StartMenu,
            menu: Menu::new(ctx),
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            GameState::Playing(game_mode) => {
                self.morpion_scene.update(ctx, &mut self.state, &game_mode);
            }
            GameState::StartMenu => {
                let gui_ctx = self.menu.gui.ctx();
                let p_p_btn = Button::new("PvP");
                let p_ai_btn = Button::new("PvAI");
                let ai_ai_btn = Button::new("AIvAI");

                egui::CentralPanel::default().show(&gui_ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_sized([150.0, 50.0], Label::new("Ultimate Morpion"));
                        if ui.add_sized([150.0, 50.0], p_p_btn).clicked() {
                            self.morpion_scene.morpion.reset();
                            self.state = GameState::Playing(GameMode::PvP);
                        }
                        if ui.add_sized([150.0, 50.0], p_ai_btn).clicked() {
                            self.state = GameState::SelectAIMenu(false);
                        }
                        if ui.add_sized([150.0, 50.0], ai_ai_btn).clicked() {
                            self.state = GameState::SelectAIMenu(true);
                        }
                    });
                });
                self.menu.gui.update(ctx);
            }
            GameState::SelectAIMenu(multi_ai) => {
                let gui_ctx = self.menu.gui.ctx();
                let easy_btn_1 = Button::new("Easy");
                let medium_btn_1 = Button::new("Medium");
                let hard_btn_1 = Button::new("Hard");
                let easy_btn_2 = Button::new("Easy");
                let medium_btn_2 = Button::new("Medium");
                let hard_btn_2 = Button::new("Hard");
                let back_btn = Button::new("Back");

                egui::CentralPanel::default().show(&gui_ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_sized([150.0, 50.0], Label::new("Ultimate Morpion"));

                        if !multi_ai {
                            if ui.add_sized([150.0, 50.0], easy_btn_1).clicked() {
                                self.morpion_scene.morpion.reset();
                                self.state = GameState::Playing(GameMode::PvAI);
                            }
                            if ui.add_sized([150.0, 50.0], medium_btn_1).clicked() {
                                self.morpion_scene.morpion.reset();
                                self.state = GameState::Playing(GameMode::PvAI);
                            }
                            if ui.add_sized([150.0, 50.0], hard_btn_1).clicked() {
                                self.morpion_scene.morpion.reset();
                                self.state = GameState::Playing(GameMode::PvAI);
                            }
                        } else {
                            if ui.add_sized([150.0, 50.0], easy_btn_2).clicked() {
                                self.morpion_scene.morpion.reset();
                                self.state = GameState::Playing(GameMode::AIvAI);
                            }
                            if ui.add_sized([150.0, 50.0], medium_btn_2).clicked() {
                                self.morpion_scene.morpion.reset();
                                self.state = GameState::Playing(GameMode::AIvAI);
                            }
                            if ui.add_sized([150.0, 50.0], hard_btn_2).clicked() {
                                self.morpion_scene.morpion.reset();
                                self.state = GameState::Playing(GameMode::AIvAI);
                            }
                        }

                        if ui.add_sized([100.0, 30.0], back_btn).clicked() {
                            self.morpion_scene.morpion.reset();
                            self.state = GameState::StartMenu;
                        }
                    });
                });
                self.menu.gui.update(ctx);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(30, 30, 38));
        match self.state {
            GameState::Playing(_) => self.morpion_scene.draw(&mut canvas, DrawParam::new()),
            _ => self.menu.draw(&mut canvas, DrawParam::new()),
        }
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
            self.morpion_scene.clicked = Some((ult_index, index));
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
        self.morpion_scene.clicked = None;
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
