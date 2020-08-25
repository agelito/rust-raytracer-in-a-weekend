use super::{Ray, Vec3};

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new() -> AABB {
        AABB {
            min: Vec3::zero(),
            max: Vec3::zero(),
        }
    }

    pub fn from_min_max(min: Vec3, max: Vec3) -> AABB {
        AABB { min: min, max: max }
    }

    pub fn combine(aabb0: &AABB, aabb1: &AABB) -> AABB {
        let smallest = Vec3::new(
            aabb0.min.x.min(aabb1.min.x),
            aabb0.min.y.min(aabb1.min.y),
            aabb0.min.z.min(aabb1.min.z),
        );
        let biggest = Vec3::new(
            aabb0.max.x.max(aabb1.max.x),
            aabb0.max.y.max(aabb1.max.y),
            aabb0.max.z.max(aabb1.max.z),
        );

        AABB {
            min: smallest,
            max: biggest,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<()> {
        {
            // test x
            let inv_d = 1.0 / ray.direction.x;
            let mut t0 = (self.min.x - ray.origin.x) / inv_d;
            let mut t1 = (self.max.x - ray.origin.x) / inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return None;
            }
        }

        {
            // test y
            let inv_d = 1.0 / ray.direction.y;
            let mut t0 = (self.min.y - ray.origin.y) / inv_d;
            let mut t1 = (self.max.y - ray.origin.y) / inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return None;
            }
        }

        {
            // test z
            let inv_d = 1.0 / ray.direction.z;
            let mut t0 = (self.min.z - ray.origin.z) / inv_d;
            let mut t1 = (self.max.z - ray.origin.z) / inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return None;
            }
        }

        Some(())
    }
}
