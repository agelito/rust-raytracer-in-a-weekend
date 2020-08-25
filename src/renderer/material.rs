use crate::color::Color;
use crate::math::{Ray, Vec3};
use crate::objects::Intersection;
use rand::prelude::*;

fn random_unit_sphere(rng: &mut dyn RngCore) -> Vec3 {
    let mut p: Vec3;
    while {
        p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);

        p.sqr_magnitude() >= 1.0
    } {}

    p
}

fn schlick(cosine: f64, index: f64) -> f64 {
    let r0 = (1.0 - index) / (1.0 + index);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn scatter(
        &self,
        ray: &Ray,
        intersection: &Intersection,
        rng: &mut dyn RngCore,
    ) -> Option<(Color, Ray)> {
        let target = intersection.position + intersection.normal + random_unit_sphere(rng);
        let scattered = Ray::at_time(
            intersection.position,
            target - intersection.position,
            ray.time,
        );

        Some((self.albedo, scattered))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    pub fn scatter(
        &self,
        ray: &Ray,
        intersection: &Intersection,
        rng: &mut dyn RngCore,
    ) -> Option<(Color, Ray)> {
        let reflection = Vec3::reflect(&ray.direction.normalize(), &intersection.normal);
        let scattered = Ray::at_time(
            intersection.position,
            reflection + self.fuzz * random_unit_sphere(rng),
            ray.time,
        );

        if Vec3::dot(&scattered.direction, &intersection.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dialectric {
    pub index: f64,
}

impl Dialectric {
    pub fn scatter(
        &self,
        ray: &Ray,
        intersection: &Intersection,
        rng: &mut dyn RngCore,
    ) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&ray.direction, &intersection.normal);

        let dir = if Vec3::dot(&ray.direction, &intersection.normal) > 0.0 {
            (
                self.index,
                -intersection.normal,
                self.index
                    * (Vec3::dot(&ray.direction, &intersection.normal) / ray.direction.magnitude()),
            )
        } else {
            (
                1.0 / self.index,
                intersection.normal,
                -Vec3::dot(&ray.direction, &intersection.normal) / ray.direction.magnitude(),
            )
        };

        let attenuation = Color::new(1.0, 1.0, 1.0, 1.0);

        match dir {
            (ni_over_nt, outward_normal, cosine) => {
                if let Some(refracted) = Vec3::refract(&ray.direction, &outward_normal, ni_over_nt)
                {
                    let prob = schlick(cosine, self.index);
                    if rng.gen::<f64>() < prob {
                        Some((
                            attenuation,
                            Ray::at_time(intersection.position, reflected, ray.time),
                        ))
                    } else {
                        Some((
                            attenuation,
                            Ray::at_time(intersection.position, refracted, ray.time),
                        ))
                    }
                } else {
                    Some((
                        attenuation,
                        Ray::at_time(intersection.position, reflected, ray.time),
                    ))
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dialectric(Dialectric),
}

impl Material {
    pub fn scatter(
        &self,
        ray: &Ray,
        intersection: &Intersection,
        rng: &mut dyn RngCore,
    ) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, intersection, rng),
            Material::Metal(m) => m.scatter(ray, intersection, rng),
            Material::Dialectric(d) => d.scatter(ray, intersection, rng),
        }
    }
}
