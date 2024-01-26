use cgmath::{Point2, point2, Vector2, vec2};
use cgmath::prelude::*;

#[derive(Copy, Clone)]
pub struct Rectangle2 {
    pub min: Point2<f32>,
    pub max: Point2<f32>,
}

impl Rectangle2 {
    pub fn empty() -> Self {
        Rectangle2 {
            min: point2(f32::NEG_INFINITY, f32::NEG_INFINITY),
            max: point2(f32::INFINITY, f32::INFINITY),
        }
    }

    pub fn single(p: Point2<f32>) -> Self {
        Rectangle2 {min: p, max: p}
    }

    pub fn add(self, p: Point2<f32>) -> Self {
        Rectangle2 {
            min: point2(self.min.x.min(p.x), self.min.y.min(p.y)),
            max: point2(self.max.x.max(p.x), self.max.y.max(p.y)),
        }
    }

    pub fn add_margin(&mut self, margin: f32) {
        self.min -= vec2(margin, margin);
        self.max += vec2(margin, margin);
    }

    pub fn center(&self) -> Point2<f32> {
        self.max.midpoint(self.min)
    }

    pub fn size(&self) -> Vector2<f32> {
        return self.max - self.min
    }
}

#[repr(C)]
#[derive(Copy,Clone,bytemuck::Pod,bytemuck::Zeroable)]
pub struct Camera2D {
    pub zoom_xy: [f32;2],
    pub center: [f32;2],
}

impl Camera2D {
    pub fn fit_rect(rect: &Rectangle2, aspect: f32) -> Self {
        let zooms = rect.size().div_element_wise(vec2(2.0 * aspect, 2.0));
        let uzoom = zooms.x.max(zooms.y);
        Camera2D {
            zoom_xy: [uzoom * aspect, uzoom],
            center: rect.center().into(),
        }
    }

    pub fn id() -> Self {
        Camera2D {
            zoom_xy: [1.0, 1.0],
            center: [0.0, 0.0],
        }
    }
}

// Same format as the color vec4f returned by a fragment shader.
// For use in buffers.
#[repr(C)]
#[derive(Copy,Clone,Debug,bytemuck::Pod,bytemuck::Zeroable)]
pub struct RGBA32float {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub alpha: f32,
}

pub fn rgba32(r: f32, g: f32, b: f32, a: f32) -> RGBA32float {
    RGBA32float {r, g, b, alpha: a}
}

fn decode_srgb_byte(v: u8) -> f32 {
    let vf = v as f32 / 255.0;
	if vf < 0.0404599 {
		vf / 12.9232102
	} else {
		((vf + 0.055) / 1.055).powf(2.4)
	}
}

// Decode color from css hex format.
// If encoded is big-endian, it is also in rega8unorm-srgb format.
pub fn srgba(hex: u32) -> RGBA32float {
    let bytes: [u8;4] = hex.to_be_bytes();
    RGBA32float {
        r: decode_srgb_byte(bytes[0]),
        g: decode_srgb_byte(bytes[1]),
        b: decode_srgb_byte(bytes[2]),
        alpha: bytes[3] as f32 / 255.0,
    }
}

impl Into<wgpu::Color> for RGBA32float {
    fn into(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.alpha as f64,
        }
    }
}
