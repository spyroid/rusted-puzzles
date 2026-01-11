#[derive(Debug)]
#[allow(warnings)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[allow(warnings)]
impl Point {
    pub fn plus(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
