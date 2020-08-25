mod intersectable;
mod moving_sphere;
mod sphere;

use crate::math::{Ray, AABB};

pub use intersectable::{Intersectable, Intersection};
pub use moving_sphere::MovingSphere;
pub use sphere::Sphere;

use bvh::aabb::{Bounded, AABB as BVH_AABB};
use bvh::bounding_hierarchy::BHShape;
use bvh::nalgebra::Point3;

#[derive(Copy, Clone)]
pub enum Object {
    Sphere(Sphere),
    MovingSphere(MovingSphere),
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        match *self {
            Object::Sphere(ref s) => s.intersect(ray, t_min, t_max),
            Object::MovingSphere(ref ms) => ms.intersect(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        match *self {
            Object::Sphere(ref s) => s.bounding_box(t0, t1),
            Object::MovingSphere(ref ms) => ms.bounding_box(t0, t1),
        }
    }
}

impl Bounded for Object {
    fn aabb(&self) -> BVH_AABB {
        let aabb = if let Some(bounding_box) = self.bounding_box(0.0, 1.0) {
            bounding_box
        } else {
            AABB::new()
        };

        let min = Point3::new(aabb.min.x as f32, aabb.min.y as f32, aabb.min.z as f32);
        let max = Point3::new(aabb.max.x as f32, aabb.max.y as f32, aabb.max.z as f32);

        BVH_AABB::with_bounds(min, max)
    }
}

impl BHShape for Object {
    fn set_bh_node_index(&mut self, index: usize) {
        match *self {
            Object::Sphere(mut s) => s.node_index = index,
            Object::MovingSphere(mut ms) => ms.node_index = index,
        }
    }

    fn bh_node_index(&self) -> usize {
        match *self {
            Object::Sphere(ref s) => s.node_index,
            Object::MovingSphere(ref ms) => ms.node_index,
        }
    }
}
