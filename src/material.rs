use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{dot, unit_vector, Color, Vec3},
};

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self { albedo: a }
    }
}

impl Scatter for Lambertian {
    #[allow(unused_variables)]
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(a: Color) -> Self {
        Self { albedo: a }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        if dot(scattered.direction(), rec.normal) < 0.0 {
            return None;
        } else {
            Some((self.albedo, scattered))
        }
    }
}
