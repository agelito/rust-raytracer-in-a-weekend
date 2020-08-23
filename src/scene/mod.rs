mod camera;

use crate::math::Ray;
use crate::objects::{Intersectable, Intersection, Object};

pub use camera::Camera;

pub struct Scene {
    pub max_recursion: u32,
    pub objects: Vec<Object>,
}

impl Intersectable for Scene {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|s| s.intersect(ray, t_min, t_max))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
