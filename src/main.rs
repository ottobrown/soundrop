use macroquad::window::{clear_background, next_frame};
use macroquad::{color, input, shapes, window};

mod types;

use types::{Point, Vector};

fn config() -> window::Conf {
    window::Conf {
        ..Default::default()
    }
}

struct State {
    lines: Vec<Line>,
    balls: Vec<Ball>,
    spawners: Vec<BallSpawner>,

    line_start: Option<Point>,
}

#[macroquad::main(config)]
async fn main() {
    let mut state = State {
        lines: Vec::new(),
        balls: Vec::new(),
        spawners: Vec::new(),

        line_start: None,
    };

    state.spawners.push(BallSpawner {
        position: Point::from((10.0, 10.0)),

        spawn_time: 45,
        frames: 0,
    });

    loop {
        clear_background(color::BLACK);

        for l in &state.lines {
            l.render();
        }

        for b in &mut state.balls {
            b.render();
        }

        for s in &mut state.spawners {
            if let Some(b) = s.render() {
                state.balls.push(b);
            }
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

struct Ball {
    pub position: Point,
    pub velocity: Vector,
}
impl Ball {
    pub fn render(&mut self) {
        self.position.add(self.velocity);

        shapes::draw_circle(self.position.x, self.position.y, 3.0, color::WHITE);
    }
}

struct BallSpawner {
    pub position: Point,
    /// number of frames between ball spawns
    pub spawn_time: u32,
    /// number of frames that have passed
    pub frames: u32,
}
impl BallSpawner {
    /// returns Some if it spawns a ball on this frame
    pub fn render(&mut self) -> Option<Ball> {
        self.frames += 1;

        shapes::draw_circle_lines(self.position.x, self.position.y, 3.0, 1.0, color::WHITE);

        if self.frames % self.spawn_time == 0 {
            return Some(Ball {
                position: self.position,
                velocity: Vector { x: 0.0, y: 2.0 },
            });
        }

        None
    }
}
