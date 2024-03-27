use raylib::prelude::*;
pub struct Bar {
    pub position: Vector2,
    pub speed: Vector2,
    pub width: f32,
    pub height: f32,
    pub sprite: Texture2D,
}

pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 480.0;
impl Bar {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, path: &'static str) -> Self {
        Self {
            position: Vector2 {
                x: SCREEN_WIDTH / 2.0,
                y: SCREEN_HEIGHT - 20.0,
            },
            speed: Vector2 { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 20.0,
            sprite: rl.load_texture(thread, path).unwrap(),
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_rec(
            &self.sprite,
            Rectangle::new(0.0, 0.0, 100.0, 15.0),
            &self.position,
            Color::WHITE,
        );
    }
}
