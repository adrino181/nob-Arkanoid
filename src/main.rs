use raylib::ffi::KeyboardKey::KEY_SPACE;
use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
struct Bar {}

struct Ball {
    position: Vector2,
    speed: Vector2,
    radius: f32,
}

/*
   @params

*/
enum Position {
    X(String),
    Y(String),
}
struct Brick {}
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

    let mut pause = false;
    let mut frame_count = 0;

    while !rl.window_should_close() {
        //key controls
        if (rl.is_key_pressed(KEY_SPACE)) {
            pause = !pause;
        }

        if (!pause) {
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
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BEIGE);
        d.draw_circle_v(ball.position, ball.radius, Color::PINK);
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

//use cmake;

// Builds the project in the directory located in `libfoo`, installing it
// into $OUT_DIR

// fn main() {
//     let dst = cmake::build("libfoo");

//     println!("cargo:rustc-link-search=native={}", dst.display());
//     println!("cargo:rustc-link-lib=static=foo");
// }
