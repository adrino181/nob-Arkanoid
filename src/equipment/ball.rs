use raylib::prelude::*;
pub struct Ball {
    pub position: Vector2,
    pub speed: Vector2,
    pub radius: f32,
    pub sprite: Texture2D,
}

pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 480.0;
impl Ball {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, path: &'static str) -> Self {
        Self {
            position: Vector2 {
                x: SCREEN_WIDTH / 2.0,
                y: SCREEN_HEIGHT / 2.0,
            },
            speed: Vector2 { x: 2.0, y: 2.0 },
            radius: 9.0,
            sprite: rl.load_texture(thread, path).unwrap(),
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle, fps_count: i32) {
        let rec_width = 29.0;
        let mut sprite_position = fps_count as f32 * rec_width;
        println!("{}", sprite_position);
        // d.draw_texture_rec(
        //     &self.sprite,
        //     Rectangle::new(-2.0, 0.0, 20.0, 20.0),
        //     &self.position,
        //     Colo::WHITE,
        // );
        // d.draw_texture_rec(
        //     &self.sprite,
        //     Rectangle::new(29.0, 0.0, 20.0, 20.0),
        //     &self.position,
        //     Color::WHITE,
        // );
        // d.draw_texture_rec(
        //     &self.sprite,
        //     Rectangle::new(39.0, 0.0, 20.0, 20.0),
        //     &self.position,
        //     Color::WHITE,
        // );
        d.draw_texture_rec(
            &self.sprite,
            Rectangle::new(29.0, 0.0, 20.0, 20.0),
            &self.position,
            Color::WHITE,
        );
    }
}
