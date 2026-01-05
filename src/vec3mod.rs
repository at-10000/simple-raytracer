use std :: ops :: {Add, Sub};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3{
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

impl Add for Vec3 {
    type Output = Self;

    fn add (self, other: Self) -> Self {
        let mut result: Self = Vec3 :: new();
        result.e[0] = self.x() + &other.x();
        result.e[1] = self.y() + &other.y();
        result.e[2] = self.z() + &other.z();
        result
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub (self, other: Self) -> Self {
        let mut result: Self = Vec3 :: new();
        result.e[0] = self.x() - &other.x();
        result.e[1] = self.y() - &other.y();
        result.e[2] = self.z() - &other.z();
        result
    }
}



