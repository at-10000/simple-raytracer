use crate :: vec3mod :: Vec3;
use crate :: vec3mod :: Point3;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new () -> Self {
        let mut origin: Point3 = Point3 :: new();
        let mut direction: Vec3 = Vec3 :: new();
        let mut result: Ray = Ray { orig: origin, dir: direction };
        result.orig = origin;
        result.dir = direction;
        result
    }

    pub fn origin (&self) -> Point3 {
        self.orig
    }

    pub fn direction (&self) -> Vec3 {
        self.dir
    }
}


