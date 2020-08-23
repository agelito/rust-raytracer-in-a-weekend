mod material;

use crate::color::Color;
use crate::math::{Ray, Vec3};
use crate::objects::Intersectable;
use crate::scene::{Camera, Scene};
use crate::{Chunk, SharedBuffer, SharedScene};
use rand::prelude::*;

pub use material::{Dialectric, Lambertian, Material, Metal};

pub fn get_color(ray: &Ray, scene: &Scene, rng: &mut dyn RngCore, depth: u32) -> Color {
    if let Some(i) = scene.intersect(ray, 0.001, std::f64::INFINITY) {
        if depth >= scene.max_recursion {
            return Color::new(0.0, 0.0, 0.0, 1.0);
        }

        if let Some(s) = i.material.scatter(ray, &i, rng) {
            return s.0 * get_color(&s.1, scene, rng, depth + 1);
        } else {
            return Color::new(0.0, 0.0, 0.0, 1.0);
        }
    }

    color_from_direction(ray)
}

pub fn color_from_direction(ray: &Ray) -> Color {
    let direction = ray.direction.normalize();
    let t = 0.5 * (direction.y + 1.0);

    let color_vec = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);

    Color::new(
        color_vec.x as f32,
        color_vec.y as f32,
        color_vec.z as f32,
        1.0,
    )
}

pub fn render_chunk(
    cp: Chunk,
    width: usize,
    height: usize,
    camera: &Camera,
    scene: &SharedScene,
    buffer: &SharedBuffer,
    rng: &mut dyn RngCore,
    ms: u32,
) {
    let wr = 1.0 / width as f32;
    let hr = 1.0 / height as f32;

    let mut result = vec![0; cp.w * cp.h];

    let fw = cp.x + cp.w;
    let fh = cp.y + cp.h;

    let cw = if fw >= width { width - cp.x } else { cp.w };
    let ch = if fh >= height { height - cp.y } else { cp.h };

    for x in 0..cw {
        for y in 0..ch {
            let mut color = Color::new(0.0, 0.0, 0.0, 1.0);
            for _s in 0..ms {
                let u = (((cp.x + x) as f32) + rng.gen::<f32>()) * wr;
                let v = 1.0 - (((cp.y + y) as f32) + rng.gen::<f32>()) * hr;

                let ray = camera.get_ray(u, v, rng);
                color = color + get_color(&ray, scene, rng, 0);
            }

            color = color * (1.0 / (ms as f32));

            result[x + y * cw] = color.into_pixel();
        }
    }
    {
        let mut buffer = buffer.lock().unwrap();
        for x in 0..cw {
            for y in 0..ch {
                buffer[(x + cp.x) + (y + cp.y) * width] = result[x + y * cw];
            }
        }
    }
}
