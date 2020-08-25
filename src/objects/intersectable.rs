use crate::math::{Ray, Vec3, AABB};
use crate::renderer::Material;

pub struct Intersection {
    pub distance: f64,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}
