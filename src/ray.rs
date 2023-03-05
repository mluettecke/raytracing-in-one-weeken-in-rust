mod vec;
use vec::{Point, Vec};

pub struct Ray {
    orig: Point,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) {
        Ray {
            orig: orig,
            dir: dir,
        }
    }

    pub fn origin(&self) -> Point {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) {
        self.orig + t * self.dir
    }
}
