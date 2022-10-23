use macroquad::window::{clear_background, next_frame};
use macroquad::{color, input, shapes, window};

mod types;

use types::Point;

fn config() -> window::Conf {
    window::Conf {
        ..Default::default()
    }
}

struct State {
    lines: Vec<Line>,
    line_start: Option<Point>,
}

#[macroquad::main(config)]
async fn main() {
    let mut state = State {
        lines: Vec::new(),
        line_start: None,
    };

    loop {
        clear_background(color::BLACK);

        for l in &state.lines {
            l.render();
        }

        match state.line_start {
            Some(p) => {
                Line {
                    start: p,
                    end: Point::from(input::mouse_position()),
                }
                .render();

                if input::is_mouse_button_released(input::MouseButton::Left) {
                    state.lines.push(Line {
                        start: p,
                        end: Point::from(input::mouse_position()),
                    });

                    state.line_start = None;
                }
            }
            None => {
                if input::is_mouse_button_down(input::MouseButton::Left) {
                    state.line_start = Some(Point::from(input::mouse_position()));
                }
            }
        }

        next_frame().await;
    }
}

#[derive(Clone, Copy)]
struct Line {
    pub start: Point,
    pub end: Point,
}
impl Line {
    pub fn render(&self) {
        shapes::draw_line(
            self.start.x,
            self.start.y,
            self.end.x,
            self.end.y,
            4.0,
            color::WHITE,
        );
    }
}
