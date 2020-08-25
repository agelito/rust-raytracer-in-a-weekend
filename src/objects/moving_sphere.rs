use super::{Intersectable, Intersection};
use crate::math::{Ray, Vec3, AABB};
use crate::renderer::Material;

#[derive(Copy, Clone)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Material,
    pub node_index: usize,
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }

    pub fn radius2(&self) -> f64 {
        self.radius * self.radius
    }
}

impl Intersectable for MovingSphere {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let direction = ray.origin - self.center(ray.time);
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
                    normal: (p - self.center(ray.time)) / self.radius,
                    material: self.material,
                });
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = ray.get_point_along(t);
                return Some(Intersection {
                    distance: t,
                    position: p,
                    normal: (p - self.center(ray.time)) / self.radius,
                    material: self.material,
                });
            }
        }

        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let box0 = AABB::from_min_max(
            self.center(t0) - Vec3::new_xyz(self.radius),
            self.center(t0) + Vec3::new_xyz(self.radius),
        );

        let box1 = AABB::from_min_max(
            self.center(t1) - Vec3::new_xyz(self.radius),
            self.center(t1) + Vec3::new_xyz(self.radius),
        );

        Some(AABB::combine(&box0, &box1))
    }
}
