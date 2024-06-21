use raylib::prelude::*;

const WINDOW_SIZE: i32 = 640;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);
    rl.set_window_position(200, 200);
    rl.set_window_min_size(WINDOW_SIZE, WINDOW_SIZE);
    rl.set_window_max_size(WINDOW_SIZE, WINDOW_SIZE);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
