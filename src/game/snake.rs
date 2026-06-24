use macroquad::{color::RED, shapes::draw_rectangle, window::screen_width};

const STEP: f32 = 1.0;

pub struct Snake {
    x: f32,
    y: f32,

    dx: f32,
    dy: f32,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            x: 10.0,
            y: 10.0,
            dx: STEP,
            dy: 0.0,
        }
    }

    fn get_block_size() -> f32 {
        screen_width() * 0.125 / 100.0
    }

    pub fn render_snake(&self) {
        let block_size = Self::get_block_size();
        draw_rectangle(self.x, self.y, 20.0 * block_size, 20.0 * block_size, RED);
    }

    pub fn move_snake(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    pub fn move_up(&mut self) {
        self.dx = 0.0;
        self.dy = -STEP;
    }

    pub fn move_down(&mut self) {
        self.dx = 0.0;
        self.dy = STEP;
    }

    pub fn move_right(&mut self) {
        self.dx = STEP;
        self.dy = 0.0;
    }

    pub fn move_left(&mut self) {
        self.dx = -STEP;
        self.dy = 0.0;
    }
}
