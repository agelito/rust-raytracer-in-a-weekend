use crate::math::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            time: 0.0,
        }
    }

    pub fn at_time(origin: Vec3, direction: Vec3, time: f64) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            time: time,
        }
    }

    pub fn get_point_along(&self, distance: f64) -> Vec3 {
        return self.origin + self.direction * distance;
    }
}
