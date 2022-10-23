#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for Point {
    fn from(p: (f32, f32)) -> Point {
        Point { x: p.0, y: p.1 }
    }
}
