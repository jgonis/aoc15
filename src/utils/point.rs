#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub(crate) fn west(&self, magnitude: i64) -> Point {
        Self {
            x: self.x - magnitude,
            y: self.y,
        }
    }
}

impl Point {
    pub(crate) fn east(&self, magnitude: i64) -> Point {
        Self {
            x: self.x + magnitude,
            y: self.y,
        }
    }
}

impl Point {
    pub(crate) fn down(&self, magnitude: i64) -> Point {
        Self {
            x: self.x,
            y: self.y - magnitude,
        }
    }
}

impl Point {
    pub(crate) fn up(&self, magnitude: i64) -> Point {
        Self {
            x: self.x,
            y: self.y + magnitude,
        }
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
