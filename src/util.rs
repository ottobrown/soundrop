#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
impl std::ops::Add<Vector> for Point {
    type Output = Self;

    fn add(self, v: Vector) -> Self::Output {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl From<(f32, f32)> for Point {
    fn from(p: (f32, f32)) -> Point {
        Point { x: p.0, y: p.1 }
    }
}

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

fn ccw(a: Point, b: Point, c: Point) -> bool {
    (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
}

/// Check if line segments ab and bc intersect
pub fn intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d)
}

pub fn slope(a: Point, b: Point) -> f32 {
    (b.y - a.y) / (b.x - a.x)
}
