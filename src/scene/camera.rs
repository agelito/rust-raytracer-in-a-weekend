use crate::math::{Ray, Vec3};
use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,

    u: Vec3,
    v: Vec3,

    lens_radius: f64,
}

fn random_in_unit_disk(rng: &mut dyn RngCore) -> Vec3 {
    let mut p: Vec3;
    while {
        p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        Vec3::dot(&p, &p) >= 1.0
    } {}

    p
}

impl Camera {
    pub fn perspective(
        from: Vec3,
        at: Vec3,
        up: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let u: Vec3;
        let v: Vec3;
        let w: Vec3;
        let theta = vfov * ::std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        w = (from - at).normalize();
        u = Vec3::cross(&up, &w).normalize();
        v = Vec3::cross(&w, &u);

        Camera {
            lower_left: from
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: from,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut dyn RngCore) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
