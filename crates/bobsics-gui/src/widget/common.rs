use std::ops::{Div, Sub};

use bobsics_render::{Brush, Color, QuadBrush};
use wgpu::{Device, TextureFormat};
use wgpu_glyph::GlyphCruncher;

// ====< FONTS >====
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}
pub enum Font {
    LeagueSpartan,
}

impl Font {
    pub fn get_font(&self) -> wgpu_glyph::ab_glyph::FontArc {
        match self {
            Font::LeagueSpartan => wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!(
                "../assets/LeagueSpartan-Bold.ttf"
            ))
            .unwrap(),
        }
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::LeagueSpartan
    }
}

// ====< POSITIONING >====
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const UNIT: Self = Self { x: 1.0, y: 1.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn from_scalar(scalar: f32) -> Self {
        Self {
            x: scalar,
            y: scalar,
        }
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl From<Vector2> for [f32; 2] {
    fn from(vec: Vector2) -> Self {
        [vec.x, vec.y]
    }
}
impl From<Vector2> for (f32, f32) {
    fn from(vec: Vector2) -> Self {
        (vec.x, vec.y)
    }
}
impl From<[f32; 2]> for Vector2 {
    fn from(vec: [f32; 2]) -> Self {
        Self {
            x: vec[0],
            y: vec[1],
        }
    }
}
impl From<(f32, f32)> for Vector2 {
    fn from(vec: (f32, f32)) -> Self {
        Self { x: vec.0, y: vec.1 }
    }
}
impl From<f32> for Vector2 {
    fn from(scalar: f32) -> Self {
        Self {
            x: scalar,
            y: scalar,
        }
    }
}

// operator overloading
impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::Mul for Vector2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
impl std::ops::Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Sub<f32> for Vector2 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

// ====< OTHER >====

pub struct UniversalBrush {
    pub glyph_brush: wgpu_glyph::GlyphBrush<()>,
    pub quad_brush: QuadBrush,
}

impl UniversalBrush {
    pub fn new(
        device: &Device,
        format: TextureFormat,
        globals_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            glyph_brush: wgpu_glyph::GlyphBrushBuilder::using_font(
                wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!(
                    "../assets/LeagueSpartan-Bold.ttf"
                ))
                .unwrap(),
            )
            .build(device, format),
            quad_brush: QuadBrush::new(device, format, globals_bind_group_layout),
        }
    }

    // ====< FONTS >====
    pub fn queue_text_raw(&mut self, section: &wgpu_glyph::Section) -> Result<(), &str> {
        self.glyph_brush.queue(section);
        Ok(())
    }
    pub fn measure(&mut self, section: &wgpu_glyph::Section) -> (f32, f32) {
        let bounds = self.glyph_brush.glyph_bounds(section).unwrap_or_else(|| {
            println!("WARNING: No bounds for section: {section:?}");
            wgpu_glyph::ab_glyph::Rect {
                min: wgpu_glyph::ab_glyph::Point { x: 0.0, y: 0.0 },
                max: wgpu_glyph::ab_glyph::Point { x: 0.0, y: 0.0 },
            }
        });
        (bounds.width(), bounds.height())
    }

    // ====< QUADS >====
    pub fn queue_quad_raw(&mut self, quad: bobsics_render::Quad) -> Result<(), &str> {
        self.quad_brush.queue_quad(quad);
        Ok(())
    }
}

impl Brush for UniversalBrush {
    fn draw_queued(
        &mut self,
        device: &wgpu::Device,
        staging_belt: &mut wgpu::util::StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        width: u32,
        height: u32,
        globals_bind_group: &wgpu::BindGroup,
    ) -> Result<(), &str> {
        self.quad_brush
            .draw_queued(device, staging_belt, encoder, target, globals_bind_group);

        self.glyph_brush
            .draw_queued(device, staging_belt, encoder, target, width, height)
            .unwrap();

        Ok(())
    }
}

// ====< BOUNDING BOX >====
pub struct BBox {
    pub min: Vector2,
    pub max: Vector2,
}

impl BBox {
    pub fn new(min: Vector2, max: Vector2) -> Self {
        Self { min, max }
    }

    pub fn from_wh(pos: Vector2, size: Vector2) -> Self {
        Self {
            min: pos,
            max: pos + size,
        }
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn center(&self) -> Vector2 {
        (self.min + self.max) / 2.0
    }

    pub fn size(&self) -> Vector2 {
        self.max - self.min
    }

    pub fn contains(&self, point: Vector2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    pub fn draw<'a>(
        &'a self,
        offset: Vector2,
        brush: &'a mut UniversalBrush,
        color: Color,
    ) -> Result<(), &str> {
        brush.queue_quad_raw(bobsics_render::Quad {
            top_left: (self.min + offset).into(),
            bottom_right: (self.max + offset).into(),
            color: Color::TRANSPARENT.into(),
            border_radius: 0.0,
            border_color: color.into(),
            border_width: 1.5,
        })
    }
}

impl From<BBox> for (f32, f32, f32, f32) {
    fn from(bbox: BBox) -> Self {
        (bbox.min.x, bbox.min.y, bbox.width(), bbox.height())
    }
}

impl From<(f32, f32, f32, f32)> for BBox {
    fn from(bbox: (f32, f32, f32, f32)) -> Self {
        Self {
            min: Vector2::new(bbox.0, bbox.1),
            max: Vector2::new(bbox.0 + bbox.2, bbox.1 + bbox.3),
        }
    }
}

// ====< EVENTS >====
pub enum GUIEvent {
    MousePressed(Vector2),
    MouseReleased(Vector2),
    CursorMoved(Vector2),
}
