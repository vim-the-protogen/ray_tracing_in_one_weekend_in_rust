pub mod hittable {
    use crate::Vec3;
    use crate::Ray;
    use crate::Point3;
    use crate::dot;

    pub struct HitRecord {
        p: Point3,
        normal: Vec3,
        t: f64,
        front_face: bool
    }

    impl HitRecord {
        pub fn set_p(&mut self, p: Point3) {
            self.p = p;
        }

        pub fn set_t(&mut self, t: f64) {
            self.t = t;
        }

        pub fn set_normal(&mut self, normal: Vec3) {
            self.normal = normal;
        }

        pub fn get_p(&self) -> Point3 {
            self.p
        }

        pub fn get_t(&self) -> f64 {
            self.t
        }

        pub fn get_normal(&self) -> Vec3 {
            self.normal
        }

        pub fn set_face_normal(&mut self,
                               ray: Ray,
                               outward_normal: Vec3) {
            self.front_face = dot(ray.direction(), outward_normal) < 0.0;
            self.normal = match self.front_face {
                true => outward_normal,
                false => outward_normal * -1.0
            };
        }
    }

    pub trait Hittable {
       fn hit(&self,
              _r: Ray,
              _t_min: f64,
              _t_max: f64,
              _rec:&mut HitRecord) -> bool {
           false
       }
    }
}
