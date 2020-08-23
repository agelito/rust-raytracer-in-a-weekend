mod intersectable;
mod sphere;

use crate::math::Ray;

pub use intersectable::{Intersectable, Intersection};
pub use sphere::Sphere;

#[derive(Copy, Clone)]
pub enum Object {
    Sphere(Sphere),
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        match *self {
            Object::Sphere(ref s) => s.intersect(ray, t_min, t_max),
        }
    }
}
