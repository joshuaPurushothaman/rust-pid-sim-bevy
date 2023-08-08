// I am completely aware that there are a million vec2f impls out there,
// i'm practicing.
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn length(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt(); //  wow, syntax
    }

    pub fn normalize(&self) -> Vec2f {
        return *self / self.length();
    }

    pub fn scale_to_length(&self, new_length: f32) -> Vec2f {
        return self.normalize() * new_length;
    }
}

// oh god this should be macro'd but whatever.
// that's beyond my rather... intermediate scope :sob:
impl ops::Add<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: Vec2f) -> Self::Output {
        Vec2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Vec2f> for Vec2f {
    fn add_assign(&mut self, rhs: Vec2f) {
        *self = *self + rhs
    }
}

impl ops::Add<f32> for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: f32) -> Self::Output {
        self + Vec2f { x: rhs, y: rhs }
    }
}

impl ops::AddAssign<f32> for Vec2f {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: Vec2f) -> Self::Output {
        Vec2f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign<Vec2f> for Vec2f {
    fn sub_assign(&mut self, rhs: Vec2f) {
        *self = *self - rhs;
    }
}

impl ops::Sub<f32> for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: f32) -> Self::Output {
        self - Vec2f { x: rhs, y: rhs }
    }
}

impl ops::SubAssign<f32> for Vec2f {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f32> for Vec2f {
    type Output = Vec2f;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2f {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Div<f32> for Vec2f {
    type Output = Vec2f;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2f {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
