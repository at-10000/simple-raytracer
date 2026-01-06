use std :: ops :: {Add, Sub, Mul, Div};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0.0; 3] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }
    
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    
    pub fn z(&self) -> f32 {
        self.e[2]
    }

}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add (self, other: Self) -> Vec3 {
        let mut result: Vec3 = Vec3 :: new();
        result.e[0] = self.x() + &other.x();
        result.e[1] = self.y() + &other.y();
        result.e[2] = self.z() + &other.z();
        result
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub (self, other: Self) -> Self {
        let mut result: Vec3 = Vec3 :: new();
        result.e[0] = self.x() - &other.x();
        result.e[1] = self.y() - &other.y();
        result.e[2] = self.z() - &other.z();
        result
    }
}


impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul (self, other: Vec3) -> Vec3 {
        let mut result: Vec3 = Vec3 :: new();
        result.e[0] = self.x() * &other.x();
        result.e[1] = self.y() * &other.y();
        result.e[2] = self.z() * &other.z();
        result
    }
}


impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul (self, other: f32) -> Vec3 {
        let mut result: Vec3 = Vec3 :: new();
        result.e[0] = self.e[0] * other;
        result.e[1] = self.e[1] * other;
        result.e[2] = self.e[2] * other;
        result
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div (self, other: f32) -> Vec3 {
        self * (1.0 / other)
    }
}

// pub fn dot () -> Vec3 {}



