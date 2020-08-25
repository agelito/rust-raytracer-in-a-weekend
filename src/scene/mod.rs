mod camera;

use crate::math::{Ray, AABB};
use crate::objects::{Intersectable, Intersection, Object};
use bvh::bvh::BVH;
use bvh::nalgebra::{Point3, Vector3};
use bvh::ray::Ray as BVH_Ray;

pub use camera::Camera;

pub struct Scene {
    pub max_recursion: u32,
    pub objects: Vec<Object>,

    bvh: BVH,
}

impl Scene {
    pub fn create_with_bvh(objects: &[Object], max_recursion: u32) -> Scene {
        let mut objects_vec = objects.to_vec();

        Scene {
            max_recursion,
            objects: objects.to_vec(),
            bvh: BVH::build(&mut objects_vec),
        }
    }
}

impl Intersectable for Scene {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let bvh_ray = BVH_Ray::new(
            Point3::new(
                ray.origin.x as f32,
                ray.origin.y as f32,
                ray.origin.z as f32,
            ),
            Vector3::new(
                ray.direction.x as f32,
                ray.direction.y as f32,
                ray.direction.z as f32,
            ),
        );

        self.bvh
            // TODO: Mutable copy of each scene (object list) for each thread. This is very inefficient and might
            // also hurt the BVH efficiency by not allowing it to cache the indices properly.
            .traverse(&bvh_ray, &mut self.objects.to_vec())
            .iter()
            .filter_map(|s| s.intersect(ray, t_min, t_max))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
        // self.objects
        //     .iter()
        //     .filter_map(|s| s.intersect(ray, t_min, t_max))
        //     .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.objects.len() == 0 {
            return None;
        }

        let first_object = self.objects.first().unwrap();

        let mut result: AABB;
        if let Some(aabb) = first_object.bounding_box(t0, t1) {
            result = aabb;
        } else {
            return None;
        }

        for object in &self.objects {
            if let Some(aabb) = object.bounding_box(t0, t1) {
                result = AABB::combine(&aabb, &result);
            }
        }

        Some(result)
    }
}
