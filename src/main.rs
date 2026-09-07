use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

struct Text {
    data: String,
    x: i32,
    y: i32,
    font_size: i32,
    color: Color,
}

impl Text {
    fn new(data: String, x: i32, y: i32, font_size: i32, color: Color) -> Self {
        Self {
            data,
            x,
            y,
            font_size,
            color,
        }
    }

    fn default() -> Self {
        Self {
            data: String::from("Hello world!"),
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

impl TextBuffer {
    fn new() -> Self {
        Self {
            texts: Vec::new()
        }
    }

    fn push(&mut self, t: Text) {
       self.texts.push(t); 
    }
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
    let mut buf = TextBuffer::new();
    let ui_names = vec!["timer", "tick_speed", "color_index"];
    let ui_positions_y = vec![0, 10, 20];
    for i in 0..ui_names.len() {
        let t = Text::new(
            ui_names[i].to_string(),
            12,
            ui_positions_y[i],
            30,
            Color::BLACK,
        );
        buf.push(t);
    }

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
            my_text.data.as_str(),
            my_text.x,
            my_text.y,
            my_text.font_size,
            colors[color_index],
        );
        for text in buf {
            d.draw_text{
                text.data.as_str(),
                text.x,
                text.y,
                text.font_size,
                text.color,
            }
        }
    }
}
