use std :: ops :: {Add, Sub, Mul, Div};

#[derive(Clone, Copy, PartialEq, Debug)]
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

impl Vec3 {
    pub fn length_squared (&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn length (&self) -> f32 {
        let mut result: f32 = 0.0;
        result = self.length_squared();
        result = result.sqrt();
        result
    }
    pub fn unit_vector (v: Vec3) -> Vec3 {
        let mut result: Vec3 = Vec3 :: new();
        result = v / v.length();
        result
    }
}

pub fn dot (u: Vec3, v: Vec3) -> f32 {
    let result: f32 = u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2];
    result
}

pub fn cross (u: Vec3, v: Vec3) -> Vec3 {
    let mut result: Vec3 = Vec3 :: new();
    result.e[0] = u.e[1] * v.e[2] - u.e[2] * v.e[1];
    result.e[1] = u.e[2] * v.e[0] - u.e[0] * v.e[2];
    result.e[2] = u.e[0] * v.e[1] - u.e[1] * v.e[0];
    result
}

pub type Point3 = Vec3;


#[cfg(test)]
mod vec3_tests {
    use super :: *;

    #[test]
    fn test_add () {

        let mut vec1: Vec3 = Vec3 :: new();
        vec1.e[0] = 1.237;
        vec1.e[1] = 17.893;
        vec1.e[2] = -14.487;

        let mut vec2: Vec3 = Vec3 :: new();

        vec2.e[0] = 2.424;
        vec2.e[1] = -3.643;
        vec2.e[2] = 27.149;

        let mut vec3: Vec3 = Vec3 :: new();

        vec3.e[0] = 3.661;
        vec3.e[1] = 14.25;
        vec3.e[2] = 12.662;

        assert!((vec1 + vec2) == vec3);
    }

    #[test]
    fn test_sub() {
        let mut vec1 = Vec3::new();
        vec1.e = [5.0, 7.0, 3.0];

        let mut vec2 = Vec3::new();
        vec2.e = [2.0, 1.0, 5.0];

        let mut expected = Vec3::new();
        expected.e = [3.0, 6.0, -2.0];

        assert_eq!(vec1 - vec2, expected);
    }

    #[test]
    fn test_scalar_mul() {
        let mut vec = Vec3::new();
        vec.e = [1.0, -2.0, 3.5];

        let scalar = 2.0;

        let mut expected = Vec3::new();
        expected.e = [2.0, -4.0, 7.0];

        assert_eq!(vec * scalar, expected);
    }

    #[test]
    fn test_vector_mul() {
        let mut vec1 = Vec3::new();
        vec1.e = [1.0, 2.0, 3.0];

        let mut vec2 = Vec3::new();
        vec2.e = [4.0, -1.0, 0.5];

        let mut expected = Vec3::new();
        expected.e = [4.0, -2.0, 1.5];

        assert_eq!(vec1 * vec2, expected);
    }

    #[test]
    fn test_div() {
        let mut vec = Vec3::new();
        vec.e = [4.0, -8.0, 2.0];

        let divisor = 2.0;

        let mut expected = Vec3::new();
        expected.e = [2.0, -4.0, 1.0];

        assert_eq!(vec / divisor, expected);
    }

    #[test]
    fn test_length_and_length_squared() {
        let mut vec = Vec3::new();
        vec.e = [3.0, 4.0, 12.0];

        assert_eq!(vec.length_squared(), 3.0*3.0 + 4.0*4.0 + 12.0*12.0);
        assert_eq!(vec.length(), 13.0);
    }

    #[test]
    fn test_unit_vector() {
        let mut vec = Vec3::new();
        vec.e = [0.0, 3.0, 4.0];

        let unit = Vec3::unit_vector(vec);
        let length = unit.length();

        // The unit vector should have length 1
        assert!((length - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_dot_product() {
        let mut u = Vec3::new();
        u.e = [1.0, 2.0, 3.0];

        let mut v = Vec3::new();
        v.e = [4.0, -5.0, 6.0];

        let expected = 1.0*4.0 + 2.0*(-5.0) + 3.0*6.0;

        assert_eq!(dot(u, v), expected);
    }

    #[test]
    fn test_cross_product() {
        let mut u = Vec3::new();
        u.e = [1.0, 2.0, 3.0];

        let mut v = Vec3::new();
        v.e = [4.0, 5.0, 6.0];

        let mut expected = Vec3::new();
        expected.e = [
            2.0*6.0 - 3.0*5.0,  // 12 - 15 = -3
            3.0*4.0 - 1.0*6.0,  // 12 - 6 = 6
            1.0*5.0 - 2.0*4.0   // 5 - 8 = -3
        ];

        assert_eq!(cross(u, v), expected);
    }
}


