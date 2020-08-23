use crate::math::{Ray, Vec3};
use crate::renderer::Material;

pub struct Intersection {
    pub distance: f64,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}
