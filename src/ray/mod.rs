pub mod ray {
    use crate::Vec3;

    pub type Point3 = Vec3;

    #[derive(Copy,Clone, Debug)]
    pub struct Ray {
        orig: Point3, 
        dir: Vec3,
    }

    impl Ray {
        pub fn new(origin: Point3, direction: Vec3) -> Self {
            Self {
                orig: origin,
                dir: direction,
            }
        }

        pub fn origin(&self) -> Point3 {
            self.orig
        }

        pub fn direction(&self) -> Vec3 {
            self.dir
        }

        pub fn at(&self, t: f64) -> Point3 {
            self.orig + (self.dir * t)
        }
    }
}
