pub mod sphere {

    use crate::Ray;
    use crate::Point3;
    use crate::Vec3;
    use crate::dot;
    //use crate::HitRecord;
    use crate::hittable::hittable::{HitRecord, Hittable};

    pub struct Sphere {
        center: Point3,
        radius: f64,
    }

    impl Sphere {
        pub fn new(cen: Point3, r: f64) -> Self {
            Self {
                center: cen,
                radius: r,
            }
        }
    }

    impl Hittable for Sphere {
        fn hit(&self,
               r: Ray,
               t_min: f64,
               t_max: f64,
               rec: &mut HitRecord) -> bool {
            let oc = r.origin() - self.center;
            let a = r.direction().length_squared();
            let half_b = dot(oc, r.direction());
            let c = oc.length_squared() - self.radius * self.radius;

            let discriminant = half_b * half_b - a * c;

            if discriminant < 0.0 {
                return false
            }

            let sqrtd = discriminant.sqrt();

            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;

                if root < t_min || t_max < root {
                    return false
                }
            }

            rec.set_t(root);
            rec.set_p(r.at(rec.get_t()));
            rec.set_normal((rec.get_p() - self.center) / self.radius);

            true
        }
    }
}
