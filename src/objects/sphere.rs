use super::{Intersectable, Intersection};
use crate::math::{Ray, Vec3};
use crate::renderer::Material;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn radius2(&self) -> f64 {
        self.radius * self.radius
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let direction = ray.origin - self.center;
        let a = Vec3::dot(&ray.direction, &ray.direction);
        let b = Vec3::dot(&direction, &ray.direction);
        let c = Vec3::dot(&direction, &direction) - self.radius2();
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;

            if t < t_max && t > t_min {
                let p = ray.get_point_along(t);
                return Some(Intersection {
                    distance: t,
                    position: p,
                    normal: (p - self.center) / self.radius,
                    material: self.material,
                });
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = ray.get_point_along(t);
                return Some(Intersection {
                    distance: t,
                    position: p,
                    normal: (p - self.center) / self.radius,
                    material: self.material,
                });
            }
        }

        None
    }
}
