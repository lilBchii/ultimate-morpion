pub const CELL_PADDING: f32 = 10.0;
pub const BORDER_PADDING: f32 = 50.0;
pub const CELL_SIZE: f32 = 75.0;
pub const BIG_CELL_SIZE: f32 = 3.0 * CELL_SIZE + 2.0 * CELL_PADDING;

pub const SCREEN_SIZE: (f32, f32) = (
    (CELL_SIZE * 9.0) + (6.0 * CELL_PADDING) + (2.0 * BORDER_PADDING),
    (CELL_SIZE * 9.0) + (6.0 * CELL_PADDING) + (3.0 * BORDER_PADDING),
);

pub const CROSS_CIRCLE_SCALE_FACTOR: f32 = 0.30612245;

pub const DESIRED_FPS: u32 = 15;
