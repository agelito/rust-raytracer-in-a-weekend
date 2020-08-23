use crate::math::Vec3;
use image::Rgba;
use std::convert::From;
use std::ops::{Add, Mul};

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            r: gamma_decode((rgba[0] as f32) / 255.0),
            g: gamma_decode((rgba[1] as f32) / 255.0),
            b: gamma_decode((rgba[2] as f32) / 255.0),
            a: gamma_decode((rgba[3] as f32) / 255.0),
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
            a: self.a.min(1.0).max(0.0),
        }
    }

    pub fn into_pixel(&self) -> u32 {
        let r: u32 = (((gamma_encode(self.r) * 255.0) as u8) as u32) << 16;
        let g: u32 = (((gamma_encode(self.g) * 255.0) as u8) as u32) << 8;
        let b: u32 = (((gamma_encode(self.b) * 255.0) as u8) as u32) << 0;
        let a: u32 = (((gamma_encode(self.a) * 255.0) as u8) as u32) << 24;

        r | g | b | a
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
            a: self.a * other,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        self * other as f32
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl From<Vec3> for Color {
    fn from(source: Vec3) -> Self {
        Color::new(source.x as f32, source.y as f32, source.z as f32, 1.0)
    }
}
