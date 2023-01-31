#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub const TRANSPARENT: Self = Self::from_rgba(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Self = Self::from_rgba(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::from_rgba(1.0, 1.0, 1.0, 1.0);

    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex(hex: u32) -> Self {
        let linear = to_linear_rgb(hex);
        Self {
            r: linear[0],
            g: linear[1],
            b: linear[2],
            a: 1.0,
        }
    }

    pub fn r(&self) -> u8 {
        (self.r * 255.0) as u8
    }
    pub fn g(&self) -> u8 {
        (self.g * 255.0) as u8
    }
    pub fn b(&self) -> u8 {
        (self.b * 255.0) as u8
    }
    pub fn a(&self) -> u8 {
        (self.a * 255.0) as u8
    }
}

impl From<Color> for [f32; 4] {
    fn from(val: Color) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}

fn to_linear_rgb(c: u32) -> [f32; 3] {
    let f = |xu: u32| {
        let x = (xu & 0xFF) as f32 / 255.0;
        if x > 0.04045 {
            ((x + 0.055) / 1.055).powf(2.4)
        } else {
            x / 12.92
        }
    };
    [f(c >> 16), f(c >> 8), f(c)]
}

impl From<Color> for wgpu::Color {
    fn from(val: Color) -> Self {
        wgpu::Color {
            r: val.r as f64,
            g: val.g as f64,
            b: val.b as f64,
            a: val.a as f64,
        }
    }
}

pub fn ndc_to_framebuffer_space(ndc: [f32; 2], framebuffer_size: [u32; 2]) -> [u32; 2] {
    [
        ((ndc[0] + 1.0) / 2.0 * framebuffer_size[0] as f32) as u32,
        ((-ndc[1] + 1.0) / 2.0 * framebuffer_size[1] as f32) as u32,
    ]
}
pub fn framebuffer_space_to_ndc(coordinates: [u32; 2], framebuffer_size: [u32; 2]) -> [f32; 2] {
    [
        (coordinates[0] as f32 / framebuffer_size[0] as f32) * 2.0 - 1.0,
        -((coordinates[1] as f32 / framebuffer_size[1] as f32) * 2.0 - 1.0),
    ]
}
