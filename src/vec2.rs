#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

pub type Point2D = Vec2;

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn length(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }

    pub fn dist(self, rhs: Vec2) -> f32 {
        (self - rhs).length()
    }
}

impl core::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl core::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl core::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl core::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
