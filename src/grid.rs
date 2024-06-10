use std::ops;
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

pub static NORTH: Point = Point { x: 0, y: 1 };
pub static EAST: Point = Point { x: 1, y: 0 };
pub static SOUTH: Point = Point { x: 0, y: -1 };
pub static WEST: Point = Point { x: -1, y: 0 };

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
