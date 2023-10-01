use raylib::ffi::KeyboardKey::KEY_SPACE;
use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

struct Brick {
    position: Vector2,
    height: f32,
    width: f32,
    pub is_visible: bool,
}

pub fn drop<T>(_x: T) {}

pub trait BreakOnCollision {
    fn destory(&self) {
        drop(self);
    }

    fn break_wall(&mut self);
}

impl BreakOnCollision for Brick {
    fn break_wall(&mut self) {
        if self.is_visible {
            self.is_visible = !self.is_visible;
        }
    }
}

struct Wall {
    wall: Vec<Brick>,
    position: Vector2,
}

pub fn setup_wall() {
    const width: f32 = SCREEN_WIDTH;
    const height: f32 = SCREEN_HEIGHT;
    let mut brick: Brick = Brick {
        position: Vector2 {
            x: width / 2.0,
            y: height / 2.0,
        },

        width: 10.0,
        height: 10.0,
        is_visible: true,
    };
}
// pub fn build_wall<T: BreakOnCollision>(item: &T) {
//     let mut wall_v: Vec<BreakOnCollision>;

//     while (item) {
//         wall_v.push(item);
//     }
//     return wall_v;
// }

// impl BreakOnCollision for Wall {
//     fn new(name: &'static str) -> Self;
// }

struct Bar {
    position: Vector2,
    speed: Vector2,
    width: f32,
    height: f32,
}

struct Ball {
    position: Vector2,
    speed: Vector2,
    radius: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Hello,ss World")
        .build();

    rl.set_target_fps(60);

    let mut ball = Ball {
        position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        speed: Vector2::new(4.0, 3.0),
        radius: 25.0,
    };

    let mut bar: Bar = Bar {
        position: Vector2 {
            x: SCREEN_WIDTH / 2.0,
            y: 1.0,
        },
        speed: Vector2 { x: 0.0, y: 0.0 },
        width: 100.0,
        height: 20.0,
    };

    let brick: Brick = Brick {
        position: Vector2 { x: 0.0, y: 0.0 },
        width: 100.0,
        height: 20.0,
        is_visible: true,
    };

    let mut pause = false;
    let mut frame_count = 0;

    while !rl.window_should_close() {
        //key controls
        if rl.is_key_pressed(KEY_SPACE) {
            pause = !pause;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            bar.position.x += 10.0;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_LEFT) {
            bar.position.x -= 10.0;
        }
        if !pause {
            ball.position += ball.speed;

            if ball.position.x >= SCREEN_WIDTH - ball.radius || ball.position.x <= ball.radius {
                ball.speed.x *= -1.0;
            }

            if ball.position.y >= SCREEN_HEIGHT - ball.radius || ball.position.y <= ball.radius {
                ball.speed.y *= -1.0;
            }
        } else {
            frame_count += 1;
        }

        //draw the main context
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        let rect = Rectangle::new(bar.position.x, bar.position.y, bar.width, bar.height);

        d.clear_background(Color::BEIGE);
        d.draw_circle_v(ball.position, ball.radius, Color::PINK);
        d.draw_rectangle_rec(&rect, color::Color::YELLOW);

        let mut wall_count: i32 = 10;
        let mut prev_pos = Vector2 { x: 0.0, y: 0.0 };
        while wall_count > 0 {
            let new_pos = Vector2 {
                x: brick.position.x + prev_pos.x + 10.0,
                y: brick.position.y + prev_pos.y + 10.0,
            };
            let brick_render = Rectangle::new(new_pos.x, new_pos.y, brick.width, brick.height);
            // brick.position.x = brick.position.x + 10.0;
            d.draw_rectangle_rec(&brick_render, color::Color::BLACK);
            wall_count = wall_count - 1;
            prev_pos = new_pos;
        }

        d.draw_text(
            "Press SPACE to pause ball movement",
            10,
            (SCREEN_HEIGHT as i32) - 25,
            20,
            Color::LIGHTGRAY,
        );

        if pause && (frame_count / 30) % 2 == 0 {
            d.draw_text(
                "PAUSED",
                (SCREEN_WIDTH as i32) / 2,
                (SCREEN_HEIGHT as i32) / 2,
                30,
                Color::GRAY,
            );
        }

        d.draw_fps(10, 10);
        // d.draw_text("Helloss, world!", 12, 12, 20, Color::BLACK);
    }
}
