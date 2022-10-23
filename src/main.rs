use macroquad::window::{clear_background, next_frame};
use macroquad::{color, input, shapes, window};

mod util;
use util::{Point, Vector};

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

    // TODO: add ability to move, add, and remove spawners
    state.spawners.push(BallSpawner {
        position: Point::from((30.0, 30.0)),

        spawn_time: 45,
        frames: 0,
    });

    loop {
        clear_background(color::BLACK);

        for l in &state.lines {
            l.render();
        }

        for i in (0..state.balls.len()).rev() {
            let b = &mut state.balls[i];
            b.render(&state.lines);

            if b.position.x > window::screen_width()
                || b.position.y > window::screen_height()
                || b.position.x < 0.0
                || b.position.y < 0.0
            {
                state.balls.remove(i);
            }
        }

        for s in &mut state.spawners {
            if let Some(b) = s.render() {
                state.balls.push(b);
            }
        }

        // TODO: add ability to delete lines
        match state.line_start {
            Some(p) => {
                Line {
                    start: p,
                    end: Point::from(input::mouse_position()),
                }
                .render();

                if input::is_mouse_button_released(input::MouseButton::Left) {
                    let end = Point::from(input::mouse_position());

                    // start should be the upper point
                    let l = if p.y < end.y {
                        Line { start: p, end: end }
                    } else {
                        Line { start: end, end: p }
                    };

                    state.lines.push(l);

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
    pub fn render(&mut self, lines: &Vec<Line>) {
        let after = self.position + self.velocity;

        for l in lines {
            if util::intersect(self.position, after, l.start, l.end) {
                // perpendicular to slope of line `l`
                let p: f32 = -1.0 / util::slope(l.start, l.end);

                let x = (self.velocity.x.powi(2) + self.velocity.y.powi(2)).sqrt()
                    / (p * p + 1.0).sqrt();

                // check if before position is left of line `l`
                if util::is_left(l.start, l.end, self.position) {
                    self.velocity.x = -(x.abs());

                    self.velocity.y = -p * x;
                } else {
                    self.velocity.x = x.abs();

                    // correct
                    self.velocity.y = p * x;
                }
            }
        }

        // force of gravity
        self.velocity.y += 0.1;

        self.position = self.position + self.velocity;
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
                velocity: Vector { x: 0.0, y: 0.0 },
            });
        }

        None
    }
}
