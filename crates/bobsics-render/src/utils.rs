
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub const TRANSPARENT: Self = Self::from_rgba(0.0, 0.0, 0.0, 0.0);

    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r, g, b, a
        }
    }

    pub fn from_hex(hex: &str) -> Self {
        Self {
            r: hex[0..2].parse().unwrap(),
            g: hex[2..4].parse().unwrap(),
            b: hex[4..6].parse().unwrap(),
            a: hex[6..8].parse().unwrap_or(1.0)
        }
    }

    pub fn r(&self) -> u8 { (self.r * 255.0) as u8 }
    pub fn g(&self) -> u8 { (self.g * 255.0) as u8 }
    pub fn b(&self) -> u8 { (self.b * 255.0) as u8 }
    pub fn a(&self) -> u8 { (self.a * 255.0) as u8 }
}

impl From<Color> for [f32; 4] {
    fn from(val: Color) -> Self {
        [
           val.r,
           val.g,
           val.b,
           val.a,
        ]
    }
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