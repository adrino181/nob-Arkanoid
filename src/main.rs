use raylib::ffi::{KeyboardKey::KEY_SPACE, LoadModelAnimations};
use raylib::prelude::*;
use std::{any, fmt, vec};

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const BRICK_WIDTH: f32 = 100.0;
// Logger function for any type that implements Debug.
fn log<T: any::Any + fmt::Debug>(value: &T) {
    let value_any = value as &dyn any::Any;

    // Try to convert our value to a `String`. If successful, we want to
    // output the `String`'s length as well as its value. If not, it's a
    // different type: just print it out unadorned.
    match value_any.downcast_ref::<String>() {
        Some(as_string) => {
            println!("String ({}): {}", as_string.len(), as_string);
        }
        None => {
            println!("{value:?}");
        }
    }
}
#[derive(Copy, Clone, Debug)]
struct Brick {
    position: Vector2,
    height: f32,
    width: f32,
    pub is_visible: bool,
}

pub fn drop<T>(_x: T) {}

//Rectangle contains queues of sprite animation
struct SpriteAnimation {
    atlas: Texture2D,
    fps: f32,
    rectangele: Rectangle,
    rect_length: i32,
}

fn DrawSpriteAnimationPro() {}
pub trait BreakOnCollision {
    fn destory(&mut self) {
        drop(self);
    }

    fn break_wall(&mut self);
}

pub trait ClubHorizontal {
    fn club(&mut self);
}

impl BreakOnCollision for Brick {
    fn break_wall(&mut self) {
        if self.is_visible {
            self.is_visible = !self.is_visible;
        }
    }

    fn destory(&mut self) {
        self.is_visible = !self.is_visible
    }
}

// struct Wall {
//     pub brick_container: Vec<Brick>,
//     position: Vector2,
// }

// trait Wall {
//     fn build_wall(&mut self) -> Vec<Brick> {
//         return Vec<>
//     }

// fh check_collision(){
//     true
// }
// }

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
    pub sprite: Texture2D,
}
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

struct Ball {
    position: Vector2,
    speed: Vector2,
    radius: f32,
    sprite: Texture2D,
}
impl Ball {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, path: &'static str) -> Self {
        Self {
            position: Vector2 {
                x: SCREEN_WIDTH / 2.0,
                y: SCREEN_HEIGHT / 2.0,
            },
            speed: Vector2 { x: 2.0, y: 2.0 },
            radius: 10.0,
            sprite: rl.load_texture(thread, path).unwrap(),
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle, fps_count: i32) {
        let rec_width = 20.0;
        let mut sprite_position = fps_count as f32 * rec_width;
        println!("{}", sprite_position);
        // d.draw_texture_rec(
        //     &self.sprite,
        //     Rectangle::new(0.0, 0.0, 20.0, 20.0),
        //     &self.position,
        //     Color::WHITE,
        // );
        // d.draw_texture_rec(
        //     &self.sprite,
        //     Rectangle::new(20.0, 0.0, 20.0, 20.0),
        //     &self.position,
        //     Color::WHITE,
        // );
        // d.draw_texture_rec(
        //     &self.sprite,
        //     Rectangle::new(40.0, 0.0, 20.0, 20.0),
        //     &self.position,
        //     Color::WHITE,
        // );
        d.draw_texture_rec(
            &self.sprite,
            Rectangle::new(20.0, 0.0, 20.0, 20.0),
            &self.position,
            Color::WHITE,
        );
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Hello,ss World")
        .vsync()
        .build();

    rl.set_target_fps(60);

    let mut ball: Ball = Ball::new(&mut rl, &thread, "assets/ball/ball_sprite_1.png");
    let mut bar: Bar = Bar::new(&mut rl, &thread, "assets/bar/bar_2.png");

    let mut pause: bool = false;
    let mut frame_count = 0;
    let mut score = 0;
    let mut wall_count: i32 = 10;
    let mut prev_pos = Vector2 { x: 0.0, y: 0.0 };
    //brickVec is wall container
    let mut brickVec: Vec<Brick> = vec![];
    let mut frame_counter = 0;
    while wall_count > 0 {
        let mut brick_horizontal_count: i64 = (SCREEN_WIDTH / BRICK_WIDTH) as i64;
        let mut count = brick_horizontal_count;

        while count > 0 {
            println!("{}", count + 1);
            let new_pos = Vector2 {
                x: prev_pos.x + BRICK_WIDTH,
                y: prev_pos.y + 10.0,
            };
            count = count - 1;

            let brick: Brick = Brick {
                position: new_pos,
                width: BRICK_WIDTH,
                height: 20.0,
                is_visible: true,
            };
            let max_x = brick_horizontal_count;
            let mut wall_run = Vector2::zero();
            brickVec.push(brick);
            prev_pos = new_pos;
        }
        wall_count = wall_count - 1;
    }

    while !rl.window_should_close() {
        //key controls
        if !rl.is_window_focused() {
            pause = true;
            frame_count = 0;
        }
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

            // print!("{}{}{}", bar.position.x, "__|__", bar.position.y);
            if bar.position.y <= ball.position.y + ball.radius {
                ball.speed.y *= -1.0;
                // ball.speed.x *= -1.0 / 2.0;
            }

            score += 1;
        } else {
            frame_count += 1;
        }

        //draw the main context
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::BEIGE);

        frame_counter = (frame_counter + 1) % 5;
        ball.draw(&mut d, frame_counter as i32);
        bar.draw(&mut d);

        //Removes brick from brickVec
        brickVec.retain(|x| {
            let rectangle = Rectangle::new(x.position.x, x.position.y, BRICK_WIDTH, 10.0);
            let is_collision = rectangle.check_collision_circle_rec(ball.position, ball.radius);
            if is_collision {
                ball.speed.y *= -1.0;
                return false;
            }
            d.draw_rectangle_rec(&rectangle, color::Color::BLACK);
            return true;
        });

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

        d.draw_text(&score.to_string(), 10, 10, 30, Color::WHITE)
    }
}
