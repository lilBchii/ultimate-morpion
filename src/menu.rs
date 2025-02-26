use ggegui::Gui;
use ggez::{
    graphics::{self, Drawable},
    Context,
};
use glam::vec2;

use crate::constants::BORDER_PADDING;

pub struct Menu {
    pub gui: Gui,
}

impl Menu {
    pub fn new(ctx: &mut Context) -> Self {
        Self { gui: Gui::new(ctx) }
    }
}

impl Drawable for Menu {
    fn draw(&self, canvas: &mut graphics::Canvas, _param: impl Into<graphics::DrawParam>) {
        canvas.draw(
            &self.gui,
            graphics::DrawParam::default().dest(vec2(BORDER_PADDING, BORDER_PADDING)),
        );
    }

    fn dimensions(
        &self,
        _gfx: &impl ggez::context::Has<graphics::GraphicsContext>,
    ) -> Option<graphics::Rect> {
        None
    }
}
