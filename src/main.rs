mod color;
mod math;
mod objects;
mod renderer;
mod scene;

use color::Color;
use crossbeam_queue::SegQueue;
use math::Vec3;
use minifb::{Key, Window, WindowOptions};
use objects::{MovingSphere, Object, Sphere};
use rand::prelude::*;
use renderer::{Dialectric, Lambertian, Material, Metal};
use scene::{Camera, Scene};
use std::sync::{Arc, Mutex};
use std::thread;

const WINDOW_WIDTH: usize = 512;
const WINDOW_HEIGHT: usize = 512;

const CHUNK_WIDTH: usize = 128;
const CHUNK_HEIGHT: usize = 128;

pub type SharedBuffer = Arc<Mutex<Vec<u32>>>;
pub type SharedScene = Arc<Scene>;

#[derive(Copy, Clone)]
pub struct Chunk {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[derive(Copy, Clone)]
pub struct RenderJob {
    pub chunk: Chunk,
    pub ms: u32,
}

fn main() {
    let buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let buffer = Arc::new(Mutex::new(buffer));

    let mut window = Window::new(
        "Ray Tracer",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            borderless: false,
            resize: true,
            scale: minifb::Scale::X1,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            title: true,
            topmost: false,
            transparency: false,
        },
    )
    .unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_millis(10)));

    let mut rng = rand::thread_rng();

    let scene = Scene::create_with_bvh(&random_spheres(&mut rng), 32);
    let scene = Arc::new(scene);

    let aspect = (WINDOW_WIDTH as f64) / (WINDOW_HEIGHT as f64);
    let from = Vec3::new(13.0, 2.0, 3.0);
    let at = Vec3::new(0.0, 0.0, 0.0);
    let dist = 10.0;
    let aperture = 0.0;

    let camera = Camera::perspective_with_time(
        from,
        at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect,
        aperture,
        dist,
        0.0,
        1.0,
    );

    let chunks_x = WINDOW_WIDTH / CHUNK_WIDTH;
    let chunks_y = WINDOW_HEIGHT / CHUNK_HEIGHT;
    let mut chunks: Vec<Chunk> = (0..(chunks_x * chunks_y))
        .map(|i| Chunk {
            x: (i % chunks_x) * CHUNK_WIDTH,
            y: (i / chunks_x) * CHUNK_HEIGHT,
            w: CHUNK_WIDTH,
            h: CHUNK_HEIGHT,
        })
        .collect();

    chunks.shuffle(&mut rng);

    let job_queue = SegQueue::<RenderJob>::new();
    let job_queue = Arc::new(job_queue);

    for chunk in &chunks {
        job_queue.push(RenderJob {
            chunk: *chunk,
            ms: 4,
        });
    }

    for chunk in &chunks {
        job_queue.push(RenderJob {
            chunk: *chunk,
            ms: 32,
        });
    }

    for chunk in &chunks {
        job_queue.push(RenderJob {
            chunk: *chunk,
            ms: 128,
        });
    }

    for chunk in &chunks {
        job_queue.push(RenderJob {
            chunk: *chunk,
            ms: 256,
        });
    }

    for chunk in &chunks {
        job_queue.push(RenderJob {
            chunk: *chunk,
            ms: 512,
        });
    }

    for _ in 0..4 {
        let thread_scene = scene.clone();
        let thread_buffer = Arc::clone(&buffer);
        let thread_queue = Arc::clone(&job_queue);

        thread::spawn(move || {
            println!("starting worker thread");

            let mut rng = rand::thread_rng();
            loop {
                if let Ok(job) = thread_queue.pop() {
                    println!(
                        "doing render job: {}, {}, {}",
                        job.chunk.x, job.chunk.y, job.ms
                    );
                    renderer::render_chunk(
                        job.chunk,
                        WINDOW_WIDTH,
                        WINDOW_HEIGHT,
                        &camera,
                        &thread_scene,
                        &thread_buffer,
                        &mut rng,
                        job.ms,
                    );

                    println!(
                        "done render job: {}, {}, {}",
                        job.chunk.x, job.chunk.y, job.ms
                    );
                    println!("remaining jobs: {}", thread_queue.len());
                }

                if thread_queue.is_empty() {
                    thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        });
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let pixels: Vec<u32>;
        {
            let buffer = &buffer.lock().unwrap();
            pixels = buffer.to_vec();
        }

        window
            .update_with_buffer(&pixels, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}

fn random_spheres(rng: &mut dyn RngCore) -> Vec<Object> {
    let mut result = vec![];

    result.push(Object::Sphere(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5, 1.0),
        }),
        node_index: 0,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let material_rng: f32 = rng.gen();
            let center = Vec3::new(
                (a as f64) + 0.9 * rng.gen::<f64>(),
                0.2,
                (b as f64) + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let material = if material_rng < 0.8 {
                    Material::Lambertian(Lambertian {
                        albedo: Color::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            1.0,
                        ),
                    })
                } else if material_rng < 0.95 {
                    Material::Metal(Metal {
                        albedo: Color::new(
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                            1.0,
                        ),
                        fuzz: 0.5 * rng.gen::<f64>(),
                    })
                } else {
                    Material::Dialectric(Dialectric { index: 1.5 })
                };

                let type_rng: f32 = rng.gen();
                if type_rng < 0.7 {
                    result.push(Object::Sphere(Sphere {
                        center: center,
                        radius: 0.2,
                        material: material,
                        node_index: 0,
                    }));
                } else {
                    result.push(Object::MovingSphere(MovingSphere {
                        center0: center,
                        center1: center + Vec3::new(0.0, 0.5 * rng.gen::<f64>(), 0.0),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: material,
                        node_index: 0,
                    }));
                }
            };
        }
    }

    result.push(Object::Sphere(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dialectric(Dialectric { index: 1.5 }),
        node_index: 0,
    }));

    result.push(Object::Sphere(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1, 1.0),
        }),
        node_index: 0,
    }));

    result.push(Object::Sphere(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Metal {
            albedo: Color::new(0.7, 0.6, 0.5, 1.0),
            fuzz: 0.0,
        }),
        node_index: 0,
    }));

    result
}
