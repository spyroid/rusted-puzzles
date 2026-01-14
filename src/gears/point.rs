use std::ops::Add;

#[derive(Debug)]
#[allow(warnings)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
