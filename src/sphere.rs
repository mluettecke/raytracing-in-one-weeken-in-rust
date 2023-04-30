use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Scatter;
use crate::vec::{dot, Point};

pub struct Sphere {
    center: Point,
    radius: f64,
    mat_ptr: Arc<dyn Scatter + Send + Sync>,
}

impl Sphere {
    pub fn new(cen: Point, r: f64, m: Arc<dyn Scatter + Send + Sync>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat_ptr: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            t: root,
            p: p,
            mat_ptr: self.mat_ptr.clone(),
            normal: (p - self.center) / self.radius,
            front_face: false,
        };
        rec.set_face_normal(r, rec.normal);
        Some(rec)
    }
}
