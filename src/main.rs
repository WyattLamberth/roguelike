use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

struct Text {
    text: String,
    x: i32,
    y: i32,
    font_size: i32,
    color: Color,
}

impl Text {
    fn new(text: String, x: i32, y: i32, font_size: i32, color: Color) -> Self {
        Self {
            text,
            x,
            y,
            font_size,
            color,
        }
    }

    fn default() -> Self {
        Self {
            text: String::from("Hello world!"),
            x: 12,
            y: 12,
            font_size: 30,
            color: Color::BLACK,
        }
    }
}

struct TextBuffer {
    texts: Vec<Text>,
}

fn main() {
    const SCREEN_WIDTH: i32 = 640;
    const SCREEN_HEIGHT: i32 = 480;

    let (mut rl_handle, rl_thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    let colors = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];


    let mut color_index = 0;
    let mut timer = 0.0;
    let tick_speed = 0.25; // seconds
    let ui_names = vec!["timer", "tick_speed", "color_index"];

    while !rl_handle.window_should_close() {
        timer += rl_handle.get_frame_time();

        if timer >= tick_speed {
            timer -= tick_speed;
            color_index = (color_index + 1) % colors.len();
        }
        let mut d = rl_handle.begin_drawing(&rl_thread);
        let my_text = Text::default();
        d.clear_background(Color::WHITE);
        d.draw_text(
            my_text.text.as_str(),
            my_text.x,
            my_text.y,
            my_text.font_size,
            colors[color_index],
        );
    }
}
